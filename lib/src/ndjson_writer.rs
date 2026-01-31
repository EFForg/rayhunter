//! NDJSON (Newline Delimited JSON) writer for analysis reports
//!
//! We write analysis results in NDJSON format to minimize the amount of state
//! Rayhunter has to keep track of in memory. The NDJSON format lets us simply
//! append new rows to the end without parsing the entire JSON object beforehand.
//!
//! See: [Newline Delimited JSON](https://docs.mulesoft.com/dataweave/latest/dataweave-formats-ndjson)

use serde::Serialize;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};

pub struct NdjsonWriter {
    writer: BufWriter<File>,
}

impl NdjsonWriter {
    /// Create a new NDJSON writer from a file handle
    pub fn new(file: File) -> Self {
        Self {
            writer: BufWriter::new(file),
        }
    }

    /// Write a serializable value as a line of NDJSON
    pub async fn write<T: Serialize>(&mut self, value: &T) -> Result<(), std::io::Error> {
        let mut value_str = serde_json::to_string(value).unwrap();
        value_str.push('\n');
        self.writer.write_all(value_str.as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }

    /// Flush any pending I/O to disk
    pub async fn flush(&mut self) -> Result<(), std::io::Error> {
        self.writer.flush().await
    }

    /// Flush and close the writer
    pub async fn close(mut self) -> Result<(), std::io::Error> {
        self.writer.flush().await?;
        Ok(())
    }
}
