use crate::gsmtap::GsmtapMessage;
use crate::diag::Timestamp;

use std::fs::File;
use std::borrow::Cow;
use std::path::Path;
use chrono::prelude::*;
use deku::prelude::*;
use pcap_file::pcapng::blocks::enhanced_packet::EnhancedPacketBlock;
use pcap_file::pcapng::blocks::interface_description::InterfaceDescriptionBlock;
use pcap_file::pcapng::PcapNgWriter;
use pcap_file::PcapError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PcapFileError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Pcap error: {0}")]
    Pcap(#[from] PcapError),
    #[error("Deku error: {0}")]
    Deku(#[from] DekuError),
}

pub struct PcapFile {
    writer: PcapNgWriter<File>,
    ip_id: u16,
}

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

#[derive(DekuWrite)]
#[deku(endian = "big")]
struct UdpHeader {
    src_port: u16,
    dst_port: u16,
    length: u16,
    checksum: u16,
}

impl PcapFile {
    pub fn new<P>(path: P) -> Result<Self, PcapFileError> where P: AsRef<Path> {
        let file = std::fs::File::options()
            .create(true)
            .write(true)
            .open(path)?;
        let writer = PcapNgWriter::new(file)?;
        Ok(PcapFile { writer, ip_id: 0 })
    }

    pub fn write_iface_header(&mut self) -> Result<(), PcapFileError> {
        let interface = InterfaceDescriptionBlock {
            linktype: pcap_file::DataLink::IPV4,
            snaplen: 0xffff,
            options: vec![],
        };
        self.writer.write_pcapng_block(interface)?;
        Ok(())
    }

    pub fn write_gsmtap_message(&mut self, msg: GsmtapMessage, timestamp: Timestamp) -> Result<(), PcapFileError> {
        let time_since_epoch = timestamp.to_datetime().signed_duration_since(DateTime::UNIX_EPOCH);
        let duration = std::time::Duration::new(time_since_epoch.num_seconds() as u64, time_since_epoch.num_nanoseconds().unwrap() as u32);
        let msg_bytes = msg.to_bytes()?;
        let ip_header = IpHeader {
            version_and_ihl: 0x45,
            dscp: 0,
            total_len: msg_bytes.len() as u16 + 20 + 8,
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
            dst_port: 4729,
            length: msg_bytes.len() as u16 + 8,
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
        self.writer.write_pcapng_block(packet)?;
        self.ip_id = self.ip_id.wrapping_add(1);
        Ok(())
    }
}
