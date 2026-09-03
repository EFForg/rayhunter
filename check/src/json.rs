use rayhunter::analysis::analyzer::ReportMetadata;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

use crate::Report;

/// Writes JSON output files incrementally. The reasoning here is that each QMDL
/// report file may contain a huge number of events, and each JSON file may
/// contain many reports, so instead of keeping them all in memory until the
/// end, we simply serialize them to JSON as we go along.
pub struct IncrementalJsonWriter<T> {
    writer: BufWriter<T>,
    reports_written: usize,
}

impl<T: AsyncWrite + Unpin> IncrementalJsonWriter<T> {
    pub async fn new(
        file: T,
        check_path: &str,
        metadata: &ReportMetadata,
    ) -> std::io::Result<Self> {
        // Writes the "static" part of the report, i.e. the initial curly brace
        // & fields that don't change depending on QMDL reports
        let mut writer = BufWriter::new(file);
        let path_str = serde_json::to_string(check_path)?;
        let metadata_str = serde_json::to_string(metadata)?;
        let s = format!(
            r#"{{ "path": {}, "metadata": {}, "reports": ["#,
            path_str, metadata_str
        );
        writer.write_all(s.as_bytes()).await?;
        Ok(Self {
            writer,
            reports_written: 0,
        })
    }

    pub async fn write_report(&mut self, report: &Report) -> std::io::Result<()> {
        // If we've already written a report, we need to separate it from the
        // next one w/ a comma
        if self.reports_written > 0 {
            self.writer.write_u8(b',').await?;
        }
        let report_str = serde_json::to_string(report)?;
        self.writer.write_all(report_str.as_bytes()).await?;
        self.reports_written += 1;
        Ok(())
    }

    pub async fn finish(mut self) -> std::io::Result<()> {
        // Close the list of reports, and then the JSON object itself
        self.writer.write_all("]}".as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use rayhunter::DeviceMetadata;
    use rayhunter::analysis::analyzer::{
        AnalyzerConfig, Event, EventType, Harness, ReportMetadata,
    };
    use serde::Deserialize;
    use tokio::io::{AsyncReadExt, DuplexStream, duplex};

    use crate::{EventWithTimestamp, Report, json::IncrementalJsonWriter};

    #[derive(Debug, Deserialize, PartialEq)]
    struct ExpectedJsonLayout {
        path: String,
        metadata: ReportMetadata,
        reports: Vec<Report>,
    }

    fn create_report(path: &str) -> Report {
        let mut report = Report::new(path);
        report.skipped = 13;
        report.warnings = 12;
        report.events.push(EventWithTimestamp {
            event: Event {
                event_type: EventType::High,
                message: "hi".into(),
            },
            timestamp: chrono::DateTime::default(),
        });
        report
    }

    async fn create_test_writer(path: &str) -> (IncrementalJsonWriter<DuplexStream>, DuplexStream) {
        let (writer, reader) = duplex(1_000_000);
        let harness =
            Harness::new_with_config(&AnalyzerConfig::default(), &DeviceMetadata::default());
        (
            IncrementalJsonWriter::new(writer, path, &harness.get_metadata())
                .await
                .unwrap(),
            reader,
        )
    }

    async fn parse_json(mut reader: DuplexStream) -> ExpectedJsonLayout {
        let mut received = String::new();
        reader.read_to_string(&mut received).await.unwrap();
        serde_json::from_str(&received).unwrap()
    }

    #[tokio::test]
    async fn test_empty() {
        let (writer, reader) = create_test_writer("test_path").await;
        writer.finish().await.unwrap();
        let value = parse_json(reader).await;
        let expected = ExpectedJsonLayout {
            path: "test_path".to_string(),
            metadata: Harness::new_with_config(
                &AnalyzerConfig::default(),
                &DeviceMetadata::default(),
            )
            .get_metadata(),
            reports: vec![],
        };
        assert_eq!(value, expected);
    }

    #[tokio::test]
    async fn test_with_one_report() {
        let (mut writer, reader) = create_test_writer("test_path").await;
        let report = create_report("foo.qmdl");
        writer.write_report(&report).await.unwrap();
        writer.finish().await.unwrap();
        let value = parse_json(reader).await;
        assert_eq!(value.reports, vec![report]);
    }

    #[tokio::test]
    async fn test_with_multiple_reports() {
        let (mut writer, reader) = create_test_writer("test_path").await;
        let report1 = create_report("foo.qmdl");
        writer.write_report(&report1).await.unwrap();
        let report2 = create_report("bar.qmdl");
        writer.write_report(&report2).await.unwrap();
        writer.finish().await.unwrap();
        let value = parse_json(reader).await;
        assert_eq!(value.reports, vec![report1, report2]);
    }
}
