//! QMDL files are Qualcomm Mobile Diagnostic Logs. Their format is very simple,
//! just a series of of concatenated HDLC encapsulated diag::Message structs.

use crate::diag_reader::DiagReader;
use crate::diag::{MessagesContainer, MESSAGE_TERMINATOR, HdlcEncapsulatedMessage, DataType};

use std::fs::File;
use std::io::{Write, BufReader, BufRead};

pub struct QmdlFileWriter {
    file: File,
    pub total_written: usize,
}

impl QmdlFileWriter {
    pub fn new<P>(path: P) -> std::io::Result<Self> where P: AsRef<std::path::Path> {
        let file = std::fs::File::options()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(QmdlFileWriter {
            file,
            total_written: 0,
        })
    }

    pub fn write_container(&mut self, container: &MessagesContainer) -> std::io::Result<()> {
        for msg in &container.messages {
            self.file.write_all(&msg.data)?;
            self.total_written += msg.data.len();
        }
        Ok(())
    }
}

pub struct QmdlFileReader {
    file: BufReader<File>,
    buf: Vec<u8>
}

impl QmdlFileReader {
    pub fn new<P>(path: P) -> std::io::Result<Self> where P: AsRef<std::path::Path> {
        let file = std::fs::File::options()
            .read(true)
            .open(path)?;
        Ok(QmdlFileReader {
            file: BufReader::new(file),
            buf: Vec::new(),
        })
    }
}

impl DiagReader for QmdlFileReader {
    type Err = std::io::Error;

    fn get_next_messages_container(&mut self) -> std::io::Result<MessagesContainer> {
        let bytes_read = self.file.read_until(MESSAGE_TERMINATOR, &mut self.buf)?;

        // Since QMDL is just a flat list of messages, we can't actually
        // reproduce the container structure they came from in the original
        // read. So we'll just pretend that all containers had exactly one
        // message. As far as I know, the number of messages per container
        // doesn't actually affect anything, so this should be fine.
        Ok(MessagesContainer {
            data_type: DataType::UserSpace,
            num_messages: 1,
            messages: vec![
                HdlcEncapsulatedMessage {
                    len: 1,
                    data: self.buf[0..bytes_read].to_vec(),
                },
            ]
        })
    }
}
