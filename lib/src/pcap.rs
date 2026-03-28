//! Parse QMDL files and create a pcap file.
//! Creates a plausible IP header and [GSMtap](https://osmocom.org/projects/baseband/wiki/GSMTAP) header and then puts the rest of the data under that for wireshark to parse.
use crate::diag::Timestamp;
use crate::gsmtap::GsmtapMessage;

use chrono::prelude::*;
use deku::prelude::*;
use pcap_file_tokio::pcapng::PcapNgWriter;
use pcap_file_tokio::pcapng::RawBlock;
use pcap_file_tokio::pcapng::blocks::enhanced_packet::EnhancedPacketBlock;
use pcap_file_tokio::pcapng::blocks::interface_description::InterfaceDescriptionBlock;
use pcap_file_tokio::pcapng::blocks::section_header::{SectionHeaderBlock, SectionHeaderOption};
use pcap_file_tokio::{Endianness, PcapError};
use std::borrow::Cow;
use thiserror::Error;
use tokio::io::AsyncWrite;

#[derive(Error, Debug)]
pub enum GsmtapPcapError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Pcap error: {0}")]
    Pcap(#[from] PcapError),
    #[error("Timestamp out of range: {0}")]
    TimestampOutOfRange(#[from] chrono::OutOfRangeError),
    #[error("Deku error: {0}")]
    Deku(#[from] DekuError),
}

/// A GPS fix to embed in each PCAP packet as a Kismet-compatible custom option.
///
/// Set `timestamp_unix_secs = 0` to signal a fixed/synthetic coordinate (no real GPS time).
pub struct KismetGpsPoint {
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp_unix_secs: u32,
}

// Block type constant for Enhanced Packet Block (pcapng spec §4.3)
const ENHANCED_PACKET_BLOCK: u32 = 0x00000006;

/// Serialises a Kismet GPS custom option into a byte buffer suitable for
/// appending directly to an EPB body.
///
/// Wire layout (section is big-endian; GPS custom payload is little-endian per
/// Kismet convention):
///
///   option_code  u16 BE = 2989  (custom binary, non-copyable)
///   option_len   u16 BE = 24    (PEN 4 + payload 20)
///   pen          u32 BE = 55922 (Kismet IANA PEN)
///   --- custom payload (20 bytes, all fields little-endian) ---
///   magic        u8  = 0x47
///   version      u8  = 0x01
///   fields_len   u16 LE = 16   (bitmask + lon + lat + ts)
///   bitmask      u32 LE = 0x26 (lon=0x2, lat=0x4, gps_time=0x20)
///   longitude    i32 LE fixed37 (degrees × 1e7)
///   latitude     i32 LE fixed37 (degrees × 1e7)
///   gps_time     u32 LE unix seconds (0 = fixed/unknown)
///   --- end-of-options marker ---
///   end_code     u16 BE = 0
///   end_len      u16 BE = 0
fn build_gps_option_bytes(gps: &KismetGpsPoint) -> Vec<u8> {
    // --- opt_comment (code 1): human-readable GPS for Wireshark ---
    // Format chosen to match what Kismet writes so tools see a consistent label.
    let comment = if gps.timestamp_unix_secs == 0 {
        format!("GPS fixed lat={:.7} lon={:.7}", gps.latitude, gps.longitude)
    } else {
        format!("GPS lat={:.7} lon={:.7} ts={}", gps.latitude, gps.longitude, gps.timestamp_unix_secs)
    };
    let comment_bytes = comment.as_bytes();
    let comment_pad = (4 - (comment_bytes.len() % 4)) % 4;

    // --- Kismet GPS custom option (code 2989) ---
    let lon_fixed: i32 = (gps.longitude * 1e7) as i32;
    let lat_fixed: i32 = (gps.latitude * 1e7) as i32;

    // Custom payload: 20 bytes, all little-endian (Kismet convention)
    let fields_len: u16 = 16; // bitmask(4) + lon(4) + lat(4) + ts(4)
    let bitmask: u32 = 0x2 | 0x4 | 0x20; // lon | lat | gps_time
    let mut payload = Vec::<u8>::with_capacity(20);
    payload.push(0x47); // magic
    payload.push(0x01); // version
    payload.extend_from_slice(&fields_len.to_le_bytes());
    payload.extend_from_slice(&bitmask.to_le_bytes());
    payload.extend_from_slice(&lon_fixed.to_le_bytes());
    payload.extend_from_slice(&lat_fixed.to_le_bytes());
    payload.extend_from_slice(&gps.timestamp_unix_secs.to_le_bytes());
    // payload is exactly 20 bytes, already 4-byte aligned

    // option_len = PEN (4) + payload (20) = 24, also 4-byte aligned
    let gps_opt_len: u16 = 24;

    let mut out = Vec::new();

    // opt_comment header + value + padding (big-endian section)
    out.extend_from_slice(&1u16.to_be_bytes());                        // option_code = 1
    out.extend_from_slice(&(comment_bytes.len() as u16).to_be_bytes()); // option_len
    out.extend_from_slice(comment_bytes);
    out.extend_from_slice(&[0u8; 3][..comment_pad]);                   // padding to 4-byte boundary

    // Kismet GPS option header + PEN + custom payload (big-endian section, LE payload)
    out.extend_from_slice(&2989u16.to_be_bytes()); // option_code
    out.extend_from_slice(&gps_opt_len.to_be_bytes());
    out.extend_from_slice(&55922u32.to_be_bytes()); // PEN
    out.extend_from_slice(&payload);

    // end-of-options marker (big-endian)
    out.extend_from_slice(&0u16.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes());
    out
}

pub struct GsmtapPcapWriter<T>
where
    T: AsyncWrite,
{
    writer: PcapNgWriter<T>,
    ip_id: u16,
}

const IP_HEADER_LEN: u16 = 20;
#[derive(DekuWrite)]
#[deku(endian = "big")]
struct IpHeader {
    version_and_ihl: u8,
    dscp: u8,
    total_len: u16,
    identification: u16,
    flags_and_frag_offset: u8,
    idk: u8,
    ttl: u8,
    protocol: u8,
    checksum: u16,
    src_addr: u32,
    dst_addr: u32,
}

const UDP_HEADER_LEN: u16 = 8;
const GSMTAP_PORT: u16 = 4729;
#[derive(DekuWrite)]
#[deku(endian = "big")]
struct UdpHeader {
    src_port: u16,
    dst_port: u16,
    length: u16,
    checksum: u16,
}

impl<T> GsmtapPcapWriter<T>
where
    T: AsyncWrite + Unpin + Send,
{
    pub async fn new(writer: T) -> Result<Self, GsmtapPcapError> {
        let metadata = crate::util::RuntimeMetadata::new();
        let package = format!(
            "{} {}",
            env!("CARGO_PKG_NAME").to_owned(),
            metadata.rayhunter_version
        );
        let section = SectionHeaderBlock {
            endianness: Endianness::Big,
            major_version: 1,
            minor_version: 0,
            section_length: -1,
            options: vec![
                SectionHeaderOption::Hardware(Cow::from(metadata.arch)),
                SectionHeaderOption::OS(Cow::from(metadata.system_os)),
                SectionHeaderOption::UserApplication(Cow::from(package)),
            ],
        };
        let writer = PcapNgWriter::with_section_header(writer, section).await?;
        Ok(GsmtapPcapWriter { writer, ip_id: 0 })
    }

    pub async fn write_iface_header(&mut self) -> Result<(), GsmtapPcapError> {
        let interface = InterfaceDescriptionBlock {
            linktype: pcap_file_tokio::DataLink::IPV4,
            snaplen: 0xffff,
            options: vec![],
        };
        self.writer.write_pcapng_block(interface).await?;
        Ok(())
    }

    pub async fn write_gsmtap_message(
        &mut self,
        msg: GsmtapMessage,
        timestamp: Timestamp,
        gps: Option<&KismetGpsPoint>,
    ) -> Result<(), GsmtapPcapError> {
        let duration = timestamp
            .to_datetime()
            .signed_duration_since(DateTime::UNIX_EPOCH)
            .to_std()?;

        // despite the timestamp above being correct, we have reduce it by
        // orders of magnitude due to a bug in pcap_file:
        // https://github.com/courvoif/pcap-file/pull/32
        let duration = std::time::Duration::from_nanos(duration.as_micros() as u64);

        let msg_bytes = msg.to_bytes()?;
        let ip_header = IpHeader {
            version_and_ihl: 0x45,
            dscp: 0,
            total_len: msg_bytes.len() as u16 + IP_HEADER_LEN + UDP_HEADER_LEN,
            identification: self.ip_id,
            flags_and_frag_offset: 0x40,
            idk: 0,
            ttl: 64,
            protocol: 0x11, // UDP
            checksum: 0xffff,
            src_addr: 0x7f000001,
            dst_addr: 0x7f000001, // TODO increment by radio_id
        };
        let udp_header = UdpHeader {
            src_port: 13337,
            dst_port: GSMTAP_PORT,
            length: msg_bytes.len() as u16 + UDP_HEADER_LEN,
            checksum: 0xffff,
        };
        let mut data: Vec<u8> = Vec::new();
        data.extend(&ip_header.to_bytes()?);
        data.extend(&udp_header.to_bytes()?);
        data.extend(&msg_bytes);

        match gps {
            None => {
                // Fast path: delegate to the library's standard EPB writer.
                let packet = EnhancedPacketBlock {
                    interface_id: 0,
                    timestamp: duration,
                    original_len: data.len() as u32,
                    data: Cow::Owned(data),
                    options: vec![],
                };
                self.writer.write_pcapng_block(packet).await?;
            }
            Some(gps_point) => {
                // GPS path: build a raw EPB body so we can append the Kismet
                // custom option directly.  The pcap-file-tokio crate does not
                // expose the inner option types publicly, so we must write the
                // option bytes manually and wrap them in a RawBlock.
                //
                // All standard pcapng multi-byte fields are big-endian (section
                // header declares Endianness::Big); the GPS custom payload uses
                // little-endian per the Kismet convention.
                let pad_len = (4 - (data.len() % 4)) % 4;
                let ts_nanos = duration.as_nanos();
                let ts_high = (ts_nanos >> 32) as u32;
                let ts_low = (ts_nanos & 0xFFFFFFFF) as u32;

                let gps_bytes = build_gps_option_bytes(gps_point);

                // Body = EPB fixed header (20 B) + data + padding + gps options
                let body_len = 20 + data.len() + pad_len + gps_bytes.len();
                let mut body = Vec::<u8>::with_capacity(body_len);
                body.extend_from_slice(&0u32.to_be_bytes());                    // interface_id
                body.extend_from_slice(&ts_high.to_be_bytes());                 // timestamp high
                body.extend_from_slice(&ts_low.to_be_bytes());                  // timestamp low
                body.extend_from_slice(&(data.len() as u32).to_be_bytes());     // captured_len
                body.extend_from_slice(&(data.len() as u32).to_be_bytes());     // original_len
                body.extend_from_slice(&data);
                body.extend_from_slice(&[0u8; 3][..pad_len]);                   // padding
                body.extend_from_slice(&gps_bytes);

                let block_total_len = (12 + body.len()) as u32;
                let raw = RawBlock {
                    type_: ENHANCED_PACKET_BLOCK,
                    initial_len: block_total_len,
                    body: Cow::Owned(body),
                    trailer_len: block_total_len,
                };
                self.writer.write_raw_block(&raw).await?;
            }
        }

        self.ip_id = self.ip_id.wrapping_add(1);
        Ok(())
    }
}
