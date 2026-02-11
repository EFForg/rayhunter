use crate::server::ServerState;

use anyhow::Error;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};
use log::error;
use rayhunter::diag::DataType;
use rayhunter::gsmtap_parser;
use rayhunter::pcap::GsmtapPcapWriter;
use rayhunter::qmdl::QmdlReader;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite, duplex};
use tokio_util::io::ReaderStream;

// Streams a pcap file chunk-by-chunk to the client by reading the QMDL data
// written so far. This is done by spawning a thread which streams chunks of
// pcap data to a channel that's piped to the client.
#[cfg_attr(feature = "apidocs", utoipa::path(
    get,
    path = "/api/pcap/{name}",
    tag = "Recordings",
    responses(
        (status = StatusCode::OK, description = "PCAP conversion successful", content_type = "application/vnd.tcpdump.pcap"),
        (status = StatusCode::NOT_FOUND, description = "Could not find file {name}"),
        (status = StatusCode::SERVICE_UNAVAILABLE, description = "QMDL file is empty")
    ),
    params(
        ("name" = String, Path, description = "QMDL filename to convert and download")
    ),
    summary = "Download a PCAP file",
    description = "Stream a PCAP file to a client in chunks by converting the QMDL data for file {name} written so far."
))]
pub async fn get_pcap(
    State(state): State<Arc<ServerState>>,
    Path(mut qmdl_name): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    let qmdl_store = state.qmdl_store_lock.read().await;
    if qmdl_name.ends_with("pcapng") {
        qmdl_name = qmdl_name.trim_end_matches(".pcapng").to_string();
    }
    let (entry_index, entry) = qmdl_store.entry_for_name(&qmdl_name).ok_or((
        StatusCode::NOT_FOUND,
        format!("couldn't find manifest entry with name {qmdl_name}"),
    ))?;
    if entry.qmdl_size_bytes == 0 {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "QMDL file is empty, try again in a bit!".to_string(),
        ));
    }
    let qmdl_size_bytes = entry.qmdl_size_bytes;
    let qmdl_file = qmdl_store
        .open_entry_qmdl(entry_index)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")))?;
    // the QMDL reader should stop at the last successfully written data chunk
    // (entry.size_bytes)
    let (reader, writer) = duplex(1024);

    tokio::spawn(async move {
        if let Err(e) = generate_pcap_data(writer, qmdl_file, qmdl_size_bytes).await {
            error!("failed to generate PCAP: {e:?}");
        }
    });

    let headers = [(CONTENT_TYPE, "application/vnd.tcpdump.pcap")];
    let body = Body::from_stream(ReaderStream::new(reader));
    Ok((headers, body).into_response())
}

pub async fn generate_pcap_data<R, W>(
    writer: W,
    qmdl_file: R,
    qmdl_size_bytes: usize,
) -> Result<(), Error>
where
    W: AsyncWrite + Unpin + Send,
    R: AsyncRead + Unpin,
{
    let mut pcap_writer = GsmtapPcapWriter::new(writer).await?;
    pcap_writer.write_iface_header().await?;

    let mut reader = QmdlReader::new(qmdl_file, Some(qmdl_size_bytes));
    while let Some(container) = reader.get_next_messages_container().await? {
        if container.data_type != DataType::UserSpace {
            continue;
        }

        for maybe_msg in container.into_messages() {
            match maybe_msg {
                Ok(msg) => {
                    let maybe_gsmtap_msg = gsmtap_parser::parse(msg)?;
                    if let Some((timestamp, gsmtap_msg)) = maybe_gsmtap_msg {
                        pcap_writer
                            .write_gsmtap_message(gsmtap_msg, timestamp)
                            .await?;
                    }
                }
                Err(e) => error!("error parsing message: {e:?}"),
            }
        }
    }

    Ok(())
}
