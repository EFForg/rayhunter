//! Qualcomm Mobile Diagnostic Log (QMDL) files have a very simple format: just
//! a series of of concatenated HDLC encapsulated diag::Message structs.
//! QmdlReader and QmdlWriter can read and write MessagesContainers to and from
//! QMDL files.

use std::io::{Cursor, ErrorKind};
use std::pin::Pin;
use std::task::Poll;

use crate::diag::{DataType, HdlcEncapsulatedMessage, MESSAGE_TERMINATOR, MessagesContainer};

use futures::TryStream;
use log::error;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use async_compression::tokio::bufread::GzipDecoder;
use async_compression::tokio::write::GzipEncoder;

pub struct QmdlWriter<T>
where
    T: AsyncWrite + Unpin,
{
    writer: GzipEncoder<T>,
    pub total_uncompressed_bytes: usize,
}

impl<T> QmdlWriter<T>
where
    T: AsyncWrite + Unpin,
{
    pub fn new(writer: T) -> Self {
        let gzip_writer = GzipEncoder::new(writer);
        QmdlWriter {
            writer: gzip_writer,
            total_uncompressed_bytes: 0,
        }
    }

    pub async fn write_container(&mut self, container: &MessagesContainer) -> std::io::Result<()> {
        for msg in &container.messages {
            // for a gzipped file, we can't use `msg.data.len()` to
            // determine the number of bytes written, so we have to
            // manually do a `write_all()` type loop
            let mut buf = Cursor::new(&msg.data);
            loop {
                let bytes_written = self.writer.write_buf(&mut buf).await?;
                self.writer.flush().await?;
                if bytes_written == 0 {
                    break;
                }
                self.total_uncompressed_bytes += bytes_written;
            }
        }
        Ok(())
    }

    pub async fn close(mut self) -> std::io::Result<()> {
        self.writer.shutdown().await?;
        Ok(())
    }
}

#[derive(Debug)]
enum QmdlReaderSource<T> {
    Compressed {
        reader: GzipDecoder<BufReader<T>>,
        eof: bool,
    },
    Uncompressed {
        reader: T,
    },
}

#[derive(Debug)]
struct QmdlAsyncReader<T> {
    source: QmdlReaderSource<T>,
    uncompressed_bytes_read: usize,
    max_uncompressed_bytes: Option<usize>,
}

impl<T> QmdlAsyncReader<T>
where
    T: AsyncRead
{
    pub fn new(reader: T, compressed: bool, max_uncompressed_bytes: Option<usize>) -> Self {
        let source = if compressed {
            QmdlReaderSource::Compressed {
                reader: GzipDecoder::new(BufReader::new(reader)),
                eof: false,
            }
        } else {
            QmdlReaderSource::Uncompressed { reader }
        };
        Self {
            source,
            uncompressed_bytes_read: 0,
            max_uncompressed_bytes,
        }
    }
}

impl<T> AsyncRead for QmdlAsyncReader<T>
where
    T: AsyncRead + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        // if we've already read beyond the byte limit, return without reading
        // into the buffer, essentially signalling EOF
        if let Some(max_bytes) = self.max_uncompressed_bytes
            && self.uncompressed_bytes_read >= max_bytes
        {
            if self.uncompressed_bytes_read > max_bytes {
                error!(
                    "warning: {} bytes read, but max_bytes was {}",
                    self.uncompressed_bytes_read, max_bytes
                );
            }
            return Poll::Ready(Ok(()));
        }

        let before = buf.filled().len();
        let this = self.get_mut();
        let res = match &mut this.source {
            QmdlReaderSource::Compressed { reader, eof } => {
                // if we already determined we've reached the Gzip EOF, don't read more
                if *eof {
                    return Poll::Ready(Ok(()));
                }

                match Pin::new(reader).poll_read(cx, buf) {
                    // if we hit an unexpected EOF in a Gzip file, it shouldn't
                    // be considered fatal, just a truncated file. mark that
                    // we're done and return the result as usual
                    Poll::Ready(Err(err)) if err.kind() == ErrorKind::UnexpectedEof => {
                        *eof = true;
                        Poll::Ready(Ok(()))
                    },
                    res => res,
                }
            },
            QmdlReaderSource::Uncompressed { reader } => {
                Pin::new(reader).poll_read(cx, buf)
            },
        };

        // if we read more bytes than is allowed, cap the buffer by
        // our max bytes
        let after = buf.filled().len();
        let read = after - before;
        if let Some(max_bytes) = this.max_uncompressed_bytes
            && this.uncompressed_bytes_read + read > max_bytes
        {
            let overread = this.uncompressed_bytes_read + read - max_bytes;
            buf.set_filled(after - overread);
        }
        res
    }
}

#[derive(Debug)]
pub struct QmdlReader<T>
where
    T: AsyncRead,
{
    buf_reader: BufReader<QmdlAsyncReader<T>>,
}

impl<T> QmdlReader<T>
where
    T: AsyncRead + Unpin,
{
    pub fn new(reader: T, compressed: bool, max_uncompressed_bytes: Option<usize>) -> Self {
        QmdlReader {
            buf_reader: BufReader::new(QmdlAsyncReader::new(
                reader, compressed, max_uncompressed_bytes
            )),
        }
    }

    pub fn as_stream(
        self,
    ) -> impl TryStream<Ok = MessagesContainer, Error = std::io::Error> {
        futures::stream::try_unfold(self, |mut reader| async {
            let maybe_container = reader.get_next_messages_container().await?;
            match maybe_container {
                Some(container) => Ok(Some((container, reader))),
                None => Ok(None),
            }
        })
    }

    pub async fn get_next_messages_container(
        &mut self,
    ) -> Result<Option<MessagesContainer>, std::io::Error> {
        let mut buf = Vec::new();
        if self.buf_reader.read_until(MESSAGE_TERMINATOR, &mut buf).await? == 0 {
            return Ok(None);
        }

        // Since QMDL is just a flat list of messages, we can't actually
        // reproduce the container structure they came from in the original
        // read. So we'll just pretend that all containers had exactly one
        // message. As far as I know, the number of messages per container
        // doesn't actually affect anything, so this should be fine.
        Ok(Some(MessagesContainer {
            data_type: DataType::UserSpace,
            num_messages: 1,
            messages: vec![HdlcEncapsulatedMessage {
                len: buf.len() as u32,
                data: buf,
            }],
        }))
    }
}

impl<T> AsyncRead for QmdlReader<T>
where
    T: AsyncRead + Unpin
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        Pin::new(&mut self.get_mut().buf_reader).poll_read(cx, buf)
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use crate::diag::CRC_CCITT;
    use crate::hdlc::hdlc_encapsulate;

    use super::*;

    fn get_test_messages() -> Vec<HdlcEncapsulatedMessage> {
        let messages: Vec<HdlcEncapsulatedMessage> = (10..20)
            .map(|i| {
                let data = hdlc_encapsulate(&vec![i as u8; i], &CRC_CCITT);
                HdlcEncapsulatedMessage {
                    len: data.len() as u32,
                    data,
                }
            })
            .collect();
        messages
    }

    // returns a byte array consisting of concatenated HDLC encapsulated
    // test messages
    fn get_test_message_bytes() -> Vec<u8> {
        get_test_messages()
            .iter()
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
                messages: messages2.to_vec(),
            },
        ]
    }

    #[tokio::test]
    async fn test_unbounded_qmdl_reader() {
        let mut buf = Cursor::new(get_test_message_bytes());
        let mut reader = QmdlReader::new(&mut buf, false, None);
        let expected_messages = get_test_messages();
        for message in expected_messages {
            let expected_container = MessagesContainer {
                data_type: DataType::UserSpace,
                num_messages: 1,
                messages: vec![message],
            };
            assert_eq!(
                expected_container,
                reader.get_next_messages_container().await.unwrap().unwrap()
            );
        }
    }

    #[tokio::test]
    async fn test_bounded_qmdl_reader() {
        let mut buf = Cursor::new(get_test_message_bytes());

        // bound the reader to the first two messages
        let mut expected_messages = get_test_messages();
        let limit = expected_messages[0].len + expected_messages[1].len;

        let mut reader = QmdlReader::new(&mut buf, false, Some(limit as usize));
        for message in expected_messages.drain(0..2) {
            let expected_container = MessagesContainer {
                data_type: DataType::UserSpace,
                num_messages: 1,
                messages: vec![message],
            };
            assert_eq!(
                expected_container,
                reader.get_next_messages_container().await.unwrap().unwrap()
            );
        }
        assert!(matches!(
            reader.get_next_messages_container().await,
            Ok(None)
        ));
    }

    /// Writes the test containers to a QmdlWriter, optionally finishing the
    /// gzip stream with a footer. Then, attempts to decompress the buffer with
    /// a QmdlWriter, asserting that the containers match what's expected.
    async fn run_compressed_reading_and_writing_tests(do_close: bool) {
        let containers = get_test_containers();
        let mut buf = Vec::new();
        {
            let mut writer = QmdlWriter::new(&mut buf);
            for container in &containers {
                writer.write_container(&container).await.unwrap();
            }
            if do_close {
                writer.close().await.unwrap();
            }
        }
        let mut reader = QmdlReader::new(Cursor::new(buf), true, None);
        let expected_messages = get_test_messages();
        for message in expected_messages {
            let expected_container = MessagesContainer {
                data_type: DataType::UserSpace,
                num_messages: 1,
                messages: vec![message],
            };
            assert_eq!(
                expected_container,
                reader.get_next_messages_container().await.unwrap().unwrap()
            );
        }
        assert!(matches!(
            reader.get_next_messages_container().await,
            Ok(None)
        ));
    }

    #[tokio::test]
    async fn test_compressed_reading_and_writing() {
        run_compressed_reading_and_writing_tests(true).await;
        run_compressed_reading_and_writing_tests(false).await;
    }
}
