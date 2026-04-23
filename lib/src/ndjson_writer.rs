// Shared NDJSON (Newline Delimited JSON) writer used by both the daemon
// (real-time analysis) and rayhunter-check (offline analysis) to ensure
// consistent output format.
//
// We write analysis results in NDJSON format to minimize in-memory state.
// Each line is a self-contained JSON object, so we can append without
// parsing the entire file.
//
// We flush after every line so each record is durable.

use serde::Serialize;
use tokio::fs::File;
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub struct NdjsonWriter {
    inner: Box<dyn AsyncWrite + Send + Unpin>,
}

impl NdjsonWriter {
    /// Convenience constructor that matches the daemon's existing usage:
    /// build a writer backed by an open `tokio::fs::File`.
    pub fn new(file: File) -> Self {
        Self::with_writer(file)
    }

    /// Build an NDJSON writer over any async writer (file, stdout, etc.).
    /// Lets `rayhunter-check` fan output to both a file and stdout without
    /// duplicating serialization logic.
    pub fn with_writer<W: AsyncWrite + Send + Unpin + 'static>(writer: W) -> Self {
        Self {
            inner: Box::new(writer),
        }
    }

    pub async fn write<T: Serialize>(&mut self, value: &T) -> Result<(), std::io::Error> {
        let mut line = serde_json::to_string(value).unwrap();
        line.push('\n');
        self.inner.write_all(line.as_bytes()).await?;
        self.inner.flush().await?;
        Ok(())
    }

    pub async fn close(mut self) -> Result<(), std::io::Error> {
        self.inner.flush().await
    }
}
