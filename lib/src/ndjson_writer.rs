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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::collections::HashMap;
    use tokio::fs;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestRecord {
        id: u32,
        name: String,
        data: HashMap<String, String>,
    }

    #[tokio::test]
    async fn test_write_single_record() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.ndjson");

        // Write a single record
        let file = File::create(&file_path).await.unwrap();
        let mut writer = NdjsonWriter::new(file);
        let record = TestRecord {
            id: 1,
            name: "test".to_string(),
            data: HashMap::from([("key".to_string(), "value".to_string())]),
        };
        writer.write(&record).await.unwrap();
        writer.close().await.unwrap();

        // Read and verify
        let content = fs::read_to_string(&file_path).await.unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 1);
        let parsed: TestRecord = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(parsed, record);
    }

    #[tokio::test]
    async fn test_write_multiple_records() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_multiple.ndjson");

        // Write multiple records
        let file = File::create(&file_path).await.unwrap();
        let mut writer = NdjsonWriter::new(file);

        let records = vec![
            TestRecord {
                id: 1,
                name: "first".to_string(),
                data: HashMap::new(),
            },
            TestRecord {
                id: 2,
                name: "second".to_string(),
                data: HashMap::from([("foo".to_string(), "bar".to_string())]),
            },
            TestRecord {
                id: 3,
                name: "third".to_string(),
                data: HashMap::from([
                    ("a".to_string(), "b".to_string()),
                    ("c".to_string(), "d".to_string()),
                ]),
            },
        ];

        for record in &records {
            writer.write(record).await.unwrap();
        }
        writer.close().await.unwrap();

        // Read and verify
        let content = fs::read_to_string(&file_path).await.unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 3);

        for (i, line) in lines.iter().enumerate() {
            let parsed: TestRecord = serde_json::from_str(line).unwrap();
            assert_eq!(parsed, records[i]);
        }
    }

    #[tokio::test]
    async fn test_flush() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_flush.ndjson");

        let file = File::create(&file_path).await.unwrap();
        let mut writer = NdjsonWriter::new(file);

        let record = TestRecord {
            id: 1,
            name: "test".to_string(),
            data: HashMap::new(),
        };

        writer.write(&record).await.unwrap();
        writer.flush().await.unwrap();

        // Verify data was flushed
        let content = fs::read_to_string(&file_path).await.unwrap();
        assert!(!content.is_empty());
        assert!(content.ends_with('\n'));
    }

    #[tokio::test]
    async fn test_write_different_types() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_types.ndjson");

        let file = File::create(&file_path).await.unwrap();
        let mut writer = NdjsonWriter::new(file);

        // Write different serializable types
        writer.write(&42u32).await.unwrap();
        writer.write(&"hello").await.unwrap();
        writer
            .write(&vec!["a", "b", "c"])
            .await
            .unwrap();
        writer.close().await.unwrap();

        // Read and verify
        let content = fs::read_to_string(&file_path).await.unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "42");
        assert_eq!(lines[1], r#""hello""#);
        assert_eq!(lines[2], r#"["a","b","c"]"#);
    }

    #[tokio::test]
    async fn test_empty_file_after_close() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test_empty.ndjson");

        let file = File::create(&file_path).await.unwrap();
        let writer = NdjsonWriter::new(file);
        writer.close().await.unwrap();

        // Verify empty file
        let metadata = fs::metadata(&file_path).await.unwrap();
        assert_eq!(metadata.len(), 0);
    }
}
