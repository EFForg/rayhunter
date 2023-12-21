use crate::diag_reader::DiagReader;
use crate::diag_device::DiagResult;
use crate::diag::*;

use deku::prelude::*;
use std::fs::File;
use std::io::Read;
use log::warn;

#[derive(Debug, DekuRead, DekuWrite)]
#[deku(endian = "little")]
pub struct DebugFileBlock<'a> {
    pub size: u32,
    #[deku(count = "size")]
    pub data: &'a [u8],
}

pub struct DebugFileReader {
    file: File,
}

impl DebugFileReader {
    pub fn new<P>(path: P) -> DiagResult<Self> where P: AsRef<std::path::Path> {
        let file = std::fs::File::options()
            .read(true)
            .open(path)?;
        Ok(DebugFileReader { file })
    }
}

impl DiagReader for DebugFileReader {
    fn get_next_messages_container(&mut self) -> DiagResult<MessagesContainer> {
        let mut bytes_read_buf = [0; 4];
        self.file.read_exact(&mut bytes_read_buf)?;
        let bytes_read = u32::from_le_bytes(bytes_read_buf) as usize;
        let mut data = vec![0; bytes_read as usize];
        self.file.read_exact(&mut data)?;
        let ((leftover_bytes, _), container) = MessagesContainer::from_bytes((&data, 0))?;
        if leftover_bytes.len() > 0 {
            warn!("warning: {} leftover bytes when parsing MessagesContainer", leftover_bytes.len());
        }
        Ok(container)
    }
}
