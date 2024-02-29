use crate::ServerState;

use rayhunter::diag::DataType;
use rayhunter::gsmtap_parser::GsmtapParser;
use rayhunter::pcap::GsmtapPcapWriter;
use rayhunter::qmdl::QmdlReader;
use axum::body::Body;
use axum::http::header::CONTENT_TYPE;
use axum::extract::{State, Path};
use axum::http::StatusCode;
use axum::response::{Response, IntoResponse};
use tokio::io::duplex;
use tokio_util::io::ReaderStream;
use std::{future, pin::pin};
use std::sync::Arc;
use log::error;
use futures::TryStreamExt;

// Streams a pcap file chunk-by-chunk to the client by reading the QMDL data
// written so far. This is done by spawning a thread which streams chunks of
// pcap data to a channel that's piped to the client.
pub async fn get_pcap(State(state): State<Arc<ServerState>>, Path(qmdl_name): Path<String>) -> Result<Response, (StatusCode, String)> {
    let qmdl_store = state.qmdl_store_lock.read().await;
    let entry = qmdl_store.entry_for_name(&qmdl_name)
        .ok_or((StatusCode::NOT_FOUND, format!("couldn't find qmdl file with name {}", qmdl_name)))?;
    if entry.size_bytes == 0 {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "QMDL file is empty, try again in a bit!".to_string()
        ));
    }

    let qmdl_file = qmdl_store.open_entry(&entry).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", e)))?;
    // the QMDL reader should stop at the last successfully written data chunk
    // (entry.size_bytes)
    let mut gsmtap_parser = GsmtapParser::new();
    let (reader, writer) = duplex(1024);
    let mut pcap_writer = GsmtapPcapWriter::new(writer).await.unwrap();
    pcap_writer.write_iface_header().await.unwrap();

    tokio::spawn(async move {
        let mut reader = QmdlReader::new(qmdl_file, Some(entry.size_bytes));
        let mut messages_stream = pin!(reader.as_stream()
            .try_filter(|container| future::ready(container.data_type == DataType::UserSpace)));
        
        while let Some(container) = messages_stream.try_next().await.expect("failed getting QMDL container") {
            for maybe_msg in container.into_messages() {
                match maybe_msg {
                    Ok(msg) => {
                        let maybe_gsmtap_msg = gsmtap_parser.recv_message(msg)
                            .expect("error parsing gsmtap message");
                        if let Some((timestamp, gsmtap_msg)) = maybe_gsmtap_msg {
                            pcap_writer.write_gsmtap_message(gsmtap_msg, timestamp).await
                                .expect("error writing pcap packet");
                        }
                    },
                    Err(e) => error!("error parsing message: {:?}", e),
                }
            }
        }
    });

    let headers = [(CONTENT_TYPE, "application/vnd.tcpdump.pcap")];
    let body = Body::from_stream(ReaderStream::new(reader));
    Ok((headers, body).into_response())
}
