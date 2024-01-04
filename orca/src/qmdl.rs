//! Qualcomm Mobile Diagnostic Log (QMDL) files have a very simple format: just
//! a series of of concatenated HDLC encapsulated diag::Message structs.
//! QmdlReader and QmdlWriter can read and write MessagesContainers to and from
//! QMDL files.

use crate::diag_reader::DiagReader;
use crate::diag::{MessagesContainer, MESSAGE_TERMINATOR, HdlcEncapsulatedMessage, DataType};

use std::io::{Write, BufReader, BufRead, Read};
use thiserror::Error;
use log::error;

pub struct QmdlWriter<T> where T: Write {
    writer: T,
    pub total_written: usize,
}

impl<T> QmdlWriter<T> where T: Write {
    pub fn new(writer: T) -> Self {
        QmdlWriter::new_with_existing_size(writer, 0)
    }

    pub fn new_with_existing_size(writer: T, existing_size: usize) -> Self {
        QmdlWriter {
            writer,
            total_written: existing_size,
        }
    }

    pub fn write_container(&mut self, container: &MessagesContainer) -> std::io::Result<()> {
        for msg in &container.messages {
            self.writer.write_all(&msg.data)?;
            self.total_written += msg.data.len();
        }
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum QmdlReaderError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Reached max_bytes count {0}")]
    MaxBytesReached(usize),
}

pub struct QmdlReader<T> where T: Read {
    reader: BufReader<T>,
    bytes_read: usize,
    max_bytes: Option<usize>,
}

impl<T> QmdlReader<T> where T: Read {
    pub fn new(reader: T, max_bytes: Option<usize>) -> Self {
        QmdlReader {
            reader: BufReader::new(reader),
            bytes_read: 0,
            max_bytes,
        }
    }
}

impl<T> DiagReader for QmdlReader<T> where T: Read {
    type Err = QmdlReaderError;

    fn get_next_messages_container(&mut self) -> Result<MessagesContainer, Self::Err> {
        if let Some(max_bytes) = self.max_bytes {
            if self.bytes_read >= max_bytes {
                if self.bytes_read > max_bytes {
                    error!("warning: {} bytes read, but max_bytes was {}", self.bytes_read, max_bytes);
                }
                return Err(QmdlReaderError::MaxBytesReached(max_bytes));
            }
        }

        let mut buf = Vec::new();
        let bytes_read = self.reader.read_until(MESSAGE_TERMINATOR, &mut buf)?;
        self.bytes_read += bytes_read;

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
                    len: bytes_read as u32,
                    data: buf,
                },
            ]
        })
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::hdlc::hdlc_encapsulate;
    use crate::diag_reader::CRC_CCITT;

    use super::*;

    fn get_test_messages() -> Vec<HdlcEncapsulatedMessage> {
        let messages: Vec<HdlcEncapsulatedMessage> = (10..20).map(|i| {
            let data = hdlc_encapsulate(&vec![i as u8; i], &CRC_CCITT);
            HdlcEncapsulatedMessage {
                len: data.len() as u32,
                data,
            }
        }).collect();
        messages
    }

    // returns a byte array consisting of concatenated HDLC encapsulated
    // test messages
    fn get_test_message_bytes() -> Vec<u8> {
        get_test_messages().iter()
            .flat_map(|msg| msg.data.clone())
            .collect()
    }

    fn get_test_containers() -> Vec<MessagesContainer> {
        let messages = get_test_messages();
        let (messages1, messages2) = messages.split_at(5);
        vec![
            MessagesContainer {
                data_type: DataType::UserSpace,
                num_messages: messages1.len() as u32,
                messages: messages1.to_vec(),
            },
            MessagesContainer {
                data_type: DataType::UserSpace,
                num_messages: messages2.len() as u32,
                messages: messages2.to_vec()
            },
        ]
    }

    #[test]
    fn test_unbounded_qmdl_reader() {
        let mut buf = Cursor::new(get_test_message_bytes());
        let mut reader = QmdlReader::new(&mut buf, None);
        let expected_messages = get_test_messages();
        for message in expected_messages {
            let expected_container = MessagesContainer {
                data_type: DataType::UserSpace,
                num_messages: 1,
                messages: vec![message],
            };
            assert_eq!(expected_container, reader.get_next_messages_container().unwrap());
        }
    }

    #[test]
    fn test_bounded_qmdl_reader() {
        let mut buf = Cursor::new(get_test_message_bytes());

        // bound the reader to the first two messages
        let mut expected_messages = get_test_messages();
        let limit = expected_messages[0].len + expected_messages[1].len;

        let mut reader = QmdlReader::new(&mut buf, Some(limit as usize));
        for message in expected_messages.drain(0..2) {
            let expected_container = MessagesContainer {
                data_type: DataType::UserSpace,
                num_messages: 1,
                messages: vec![message],
            };
            assert_eq!(expected_container, reader.get_next_messages_container().unwrap());
        }
        assert!(matches!(reader.get_next_messages_container(), Err(QmdlReaderError::MaxBytesReached(_))));
    }

    #[test]
    fn test_qmdl_writer() {
        let mut buf = Vec::new();
        let mut writer = QmdlWriter::new(&mut buf);
        let expected_containers = get_test_containers();
        for container in &expected_containers {
            writer.write_container(container).unwrap();
        }
        assert_eq!(writer.total_written, buf.len());
        assert_eq!(buf, get_test_message_bytes());
    }

    #[test]
    fn test_writing_and_reading() {
        let mut buf = Vec::new();
        let mut writer = QmdlWriter::new(&mut buf);
        let expected_containers = get_test_containers();
        for container in &expected_containers {
            writer.write_container(container).unwrap();
        }

        let limit = Some(buf.len());
        let mut reader = QmdlReader::new(Cursor::new(&mut buf), limit);
        let expected_messages = get_test_messages();
        for message in expected_messages {
            let expected_container = MessagesContainer {
                data_type: DataType::UserSpace,
                num_messages: 1,
                messages: vec![message],
            };
            assert_eq!(expected_container, reader.get_next_messages_container().unwrap());
        }
        assert!(matches!(reader.get_next_messages_container(), Err(QmdlReaderError::MaxBytesReached(_))));
    }
}
