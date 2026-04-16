use crate::config::GpsMode;
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
use rayhunter::pcap::{GpsPoint, GsmtapPcapWriter};
use rayhunter::qmdl::QmdlReader;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncWrite, duplex};
use tokio_util::io::ReaderStream;

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

pub(crate) async fn load_gps_records_for_entry(
    state: &Arc<ServerState>,
    entry_index: usize,
) -> Vec<GpsRecord> {
    // Always try the per-session sidecar first — it reflects what was actually
    // recorded regardless of what the current gps_mode config is.
    let entry_gps_mode;
    {
        let qmdl_store = state.qmdl_store_lock.read().await;
        if let Ok(file) = qmdl_store.open_entry_gps(entry_index).await {
            let records = load_gps_records(file).await;
            if !records.is_empty() {
                return records;
            }
        }
        // Capture the entry's recorded GPS mode before releasing the lock.
        entry_gps_mode = qmdl_store
            .manifest
            .entries
            .get(entry_index)
            .and_then(|e| e.gps_mode);
    }
    // Sidecar missing or empty — fall back using the entry's own recorded GPS mode,
    // not the current config, so old fixed-mode sessions still get coordinates even
    // if the mode has since been changed. Use the configured fixed coords directly
    // rather than gps_state, which can be overwritten by API calls or be None.
    if entry_gps_mode == Some(GpsMode::Fixed)
        && let (Some(lat), Some(lon)) = (
            state.config.gps_fixed_latitude,
            state.config.gps_fixed_longitude,
        )
    {
        return vec![GpsRecord {
            unix_ts: 0,
            lat,
            lon,
        }];
    }
    vec![]
}

fn find_nearest_gps(records: &[GpsRecord], packet_unix_ts: i64) -> Option<GpsPoint> {
    if records.is_empty() {
        return None;
    }
    let idx = records.partition_point(|r| r.unix_ts <= packet_unix_ts);
    let record = if idx == 0 {
        &records[0]
    } else if idx >= records.len() {
        &records[records.len() - 1]
    } else {
        let (before, after) = (&records[idx - 1], &records[idx]);
        let before_delta = packet_unix_ts - before.unix_ts;
        let after_delta = after.unix_ts - packet_unix_ts;
        if before_delta <= after_delta {
            before
        } else {
            after
        }
    };
    Some(GpsPoint {
        latitude: record.lat,
        longitude: record.lon,
        unix_ts: record.unix_ts,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(unix_ts: i64, lat: f64, lon: f64) -> GpsRecord {
        GpsRecord { unix_ts, lat, lon }
    }

    #[test]
    fn test_empty_returns_none() {
        assert!(find_nearest_gps(&[], 100).is_none());
    }

    #[test]
    fn test_single_record_always_returned() {
        let records = vec![rec(100, 1.0, 2.0)];
        assert_eq!(find_nearest_gps(&records, 0).unwrap().unix_ts, 100);
        assert_eq!(find_nearest_gps(&records, 200).unwrap().unix_ts, 100);
    }

    #[test]
    fn test_before_all_records_returns_first() {
        let records = vec![rec(100, 1.0, 2.0), rec(200, 3.0, 4.0)];
        assert_eq!(find_nearest_gps(&records, 50).unwrap().unix_ts, 100);
    }

    #[test]
    fn test_after_all_records_returns_last() {
        let records = vec![rec(100, 1.0, 2.0), rec(200, 3.0, 4.0)];
        assert_eq!(find_nearest_gps(&records, 300).unwrap().unix_ts, 200);
    }

    #[test]
    fn test_exact_match() {
        let records = vec![rec(100, 1.0, 2.0), rec(200, 3.0, 4.0), rec(300, 5.0, 6.0)];
        assert_eq!(find_nearest_gps(&records, 200).unwrap().unix_ts, 200);
    }

    #[test]
    fn test_closer_to_before() {
        // packet at 130: delta to before(100)=30, delta to after(200)=70 → picks before
        let records = vec![rec(100, 1.0, 2.0), rec(200, 3.0, 4.0)];
        assert_eq!(find_nearest_gps(&records, 130).unwrap().unix_ts, 100);
    }

    #[test]
    fn test_closer_to_after() {
        // packet at 170: delta to before(100)=70, delta to after(200)=30 → picks after
        let records = vec![rec(100, 1.0, 2.0), rec(200, 3.0, 4.0)];
        assert_eq!(find_nearest_gps(&records, 170).unwrap().unix_ts, 200);
    }

    #[test]
    fn test_equidistant_prefers_before() {
        // packet at 150: delta to before(100)=50, delta to after(200)=50 → tie, picks before
        let records = vec![rec(100, 1.0, 2.0), rec(200, 3.0, 4.0)];
        assert_eq!(find_nearest_gps(&records, 150).unwrap().unix_ts, 100);
    }
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
                        let packet_unix_ts = timestamp.to_datetime().timestamp();
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
