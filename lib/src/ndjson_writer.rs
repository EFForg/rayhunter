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
use tokio::io::AsyncWriteExt;

pub struct NdjsonWriter {
    file: File,
}

impl NdjsonWriter {
    pub fn new(file: File) -> Self {
        Self { file }
    }

    pub async fn write<T: Serialize>(&mut self, value: &T) -> Result<(), std::io::Error> {
        let mut line = serde_json::to_string(value).unwrap();
        line.push('\n');
        self.file.write_all(line.as_bytes()).await?;
        self.file.flush().await?;
        Ok(())
    }

    pub async fn close(mut self) -> Result<(), std::io::Error> {
        self.file.flush().await
    }
}
