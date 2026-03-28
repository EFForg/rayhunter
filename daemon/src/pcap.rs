use crate::gps::{GpsRecord, load_gps_records};
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
use rayhunter::pcap::{GsmtapPcapWriter, KismetGpsPoint};
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
    let (reader, writer) = duplex(1024);
    let gps_records = load_gps_records_for_entry(&state, entry_index).await;
    drop(qmdl_store);

    tokio::spawn(async move {
        if let Err(e) = generate_pcap_data(writer, qmdl_file, qmdl_size_bytes, gps_records).await {
            error!("failed to generate PCAP: {e:?}");
        }
    });

    let headers = [(CONTENT_TYPE, "application/vnd.tcpdump.pcap")];
    let body = Body::from_stream(ReaderStream::new(reader));
    Ok((headers, body).into_response())
}

/// Loads GPS records for a recording entry.
///
/// - `gps_mode == 0`: returns empty vec (no GPS)
/// - `gps_mode == 1`: returns a single synthetic record with `unix_ts = 0` (fixed coordinates)
/// - `gps_mode == 2`: loads per-fix records from the GPS sidecar file
pub(crate) async fn load_gps_records_for_entry(
    state: &Arc<ServerState>,
    entry_index: usize,
) -> Vec<GpsRecord> {
    if state.config.gps_mode == 0 {
        return vec![];
    }
    if state.config.gps_mode == 1 {
        let guard = state.gps_state.read().await;
        return guard
            .as_ref()
            .map(|g| {
                vec![GpsRecord {
                    unix_ts: 0, // 0 signals fixed/synthetic to the Kismet option builder
                    lat: g.latitude,
                    lon: g.longitude,
                }]
            })
            .unwrap_or_default();
    }
    // gps_mode == 2: load from sidecar
    let qmdl_store = state.qmdl_store_lock.read().await;
    match qmdl_store.open_entry_gps(entry_index).await {
        Ok(file) => load_gps_records(file).await,
        Err(_) => vec![],
    }
}

/// Returns the GPS fix from `records` whose `unix_ts` is closest to `packet_unix_ts`.
/// Returns `None` if `records` is empty.
fn find_nearest_gps(records: &[GpsRecord], packet_unix_ts: u32) -> Option<KismetGpsPoint> {
    if records.is_empty() {
        return None;
    }
    let idx = records.partition_point(|r| r.unix_ts <= packet_unix_ts);
    let record = if idx == 0 {
        &records[0]
    } else if idx >= records.len() {
        &records[records.len() - 1]
    } else {
        let before = &records[idx - 1];
        let after = &records[idx];
        if packet_unix_ts - before.unix_ts <= after.unix_ts - packet_unix_ts {
            before
        } else {
            after
        }
    };
    Some(KismetGpsPoint {
        latitude: record.lat,
        longitude: record.lon,
        timestamp_unix_secs: record.unix_ts,
    })
}

pub async fn generate_pcap_data<R, W>(
    writer: W,
    qmdl_file: R,
    qmdl_size_bytes: usize,
    gps_records: Vec<GpsRecord>,
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
                        let packet_unix_ts =
                            timestamp.to_datetime().timestamp().max(0) as u32;
                        let gps = find_nearest_gps(&gps_records, packet_unix_ts);
                        pcap_writer
                            .write_gsmtap_message(gsmtap_msg, timestamp, gps.as_ref())
                            .await?;
                    }
                }
                Err(e) => error!("error parsing message: {e:?}"),
            }
        }
    }

    Ok(())
}
