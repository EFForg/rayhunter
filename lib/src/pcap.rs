//! Parse QMDL files and create a pcap file. 
//! Creates a plausible IP header and [GSMtap](https://osmocom.org/projects/baseband/wiki/GSMTAP) header and then puts the rest of the data under that for wireshark to parse. 
use crate::gsmtap::GsmtapMessage;
use crate::diag::Timestamp;

use tokio::io::AsyncWrite;
use std::borrow::Cow;
use chrono::prelude::*;
use deku::prelude::*;
use pcap_file_tokio::pcapng::blocks::enhanced_packet::EnhancedPacketBlock;
use pcap_file_tokio::pcapng::blocks::interface_description::InterfaceDescriptionBlock;
use pcap_file_tokio::pcapng::blocks::section_header::{SectionHeaderBlock, SectionHeaderOption};
use pcap_file_tokio::pcapng::PcapNgWriter;
use pcap_file_tokio::{Endianness, PcapError};
use thiserror::Error;

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

pub struct GsmtapPcapWriter<T> where T: AsyncWrite {
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

impl<T> GsmtapPcapWriter<T> where T: AsyncWrite + Unpin + Send {
    pub async fn new(writer: T) -> Result<Self, GsmtapPcapError> {
        let metadata = crate::util::RuntimeMetadata::new();
        let package = format!("{} {}", env!("CARGO_PKG_NAME").to_owned(), metadata.rayhunter_version);
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

    pub async fn write_gsmtap_message(&mut self, msg: GsmtapMessage, timestamp: Timestamp) -> Result<(), GsmtapPcapError> {
        let duration = timestamp.to_datetime()
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
        let packet = EnhancedPacketBlock {
            interface_id: 0,
            timestamp: duration,
            original_len: data.len() as u32,
            data: Cow::Owned(data),
            options: vec![],
        };
        self.writer.write_pcapng_block(packet).await?;
        self.ip_id = self.ip_id.wrapping_add(1);
        Ok(())
    }
}
