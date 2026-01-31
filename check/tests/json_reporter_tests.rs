#[cfg(test)]
mod tests {
    use rayhunter::analysis::analyzer::{
        AnalysisRow, AnalyzerConfig, Event, EventType, Harness, ReportMetadata,
    };
    use std::io::{BufRead, BufReader};
    use tempfile::TempDir;

    #[test]
    fn test_json_reporter_basic() {
        // Create a mock reporter with some test data
        let harness = Harness::new_with_config(&AnalyzerConfig::default());
        let metadata = harness.get_metadata();

        let mut rows = Vec::new();
        rows.push(AnalysisRow {
            packet_timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z").unwrap(),
            ),
            skipped_message_reason: None,
            events: vec![
                Some(Event {
                    event_type: EventType::High,
                    message: "Test warning".to_string(),
                }),
                None,
            ],
        });

        // Verify metadata can be serialized
        let metadata_json = serde_json::to_string(&metadata).unwrap();
        assert!(metadata_json.contains("analyzers"));
        assert!(metadata_json.contains("rayhunter"));

        // Verify rows can be serialized
        let row_json = serde_json::to_string(&rows[0]).unwrap();
        assert!(row_json.contains("packet_timestamp"));
        assert!(row_json.contains("Test warning"));
        assert!(row_json.contains("High"));
    }

    #[test]
    fn test_json_reporter_skipped_messages() {
        let mut rows = Vec::new();
        rows.push(AnalysisRow {
            packet_timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z").unwrap(),
            ),
            skipped_message_reason: Some("Failed to parse GSMTAP".to_string()),
            events: vec![],
        });

        let row_json = serde_json::to_string(&rows[0]).unwrap();
        assert!(row_json.contains("skipped_message_reason"));
        assert!(row_json.contains("Failed to parse GSMTAP"));
    }

    #[test]
    fn test_json_reporter_multiple_events() {
        let mut rows = Vec::new();
        rows.push(AnalysisRow {
            packet_timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z").unwrap(),
            ),
            skipped_message_reason: None,
            events: vec![
                Some(Event {
                    event_type: EventType::Low,
                    message: "Low severity event".to_string(),
                }),
                Some(Event {
                    event_type: EventType::Medium,
                    message: "Medium severity event".to_string(),
                }),
                Some(Event {
                    event_type: EventType::High,
                    message: "High severity event".to_string(),
                }),
                Some(Event {
                    event_type: EventType::Informational,
                    message: "Info message".to_string(),
                }),
            ],
        });

        let row_json = serde_json::to_string(&rows[0]).unwrap();
        assert!(row_json.contains("Low severity event"));
        assert!(row_json.contains("Medium severity event"));
        assert!(row_json.contains("High severity event"));
        assert!(row_json.contains("Info message"));
    }

    #[test]
    fn test_ndjson_format_structure() {
        // Test that we can create NDJSON format (newline delimited JSON)
        let harness = Harness::new_with_config(&AnalyzerConfig::default());
        let metadata = harness.get_metadata();

        let mut output = String::new();

        // First line: metadata
        let metadata_json = serde_json::to_string(&metadata).unwrap();
        output.push_str(&metadata_json);
        output.push('\n');

        // Subsequent lines: rows
        let row = AnalysisRow {
            packet_timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z").unwrap(),
            ),
            skipped_message_reason: None,
            events: vec![Some(Event {
                event_type: EventType::High,
                message: "Test warning".to_string(),
            })],
        };
        let row_json = serde_json::to_string(&row).unwrap();
        output.push_str(&row_json);
        output.push('\n');

        // Verify format
        let lines: Vec<&str> = output.lines().collect();
        assert_eq!(lines.len(), 2);

        // Parse metadata line
        let parsed_metadata: ReportMetadata = serde_json::from_str(lines[0]).unwrap();
        assert!(!parsed_metadata.analyzers.is_empty());

        // Parse row line
        let parsed_row: AnalysisRow = serde_json::from_str(lines[1]).unwrap();
        assert_eq!(parsed_row.events.len(), 1);
        assert_eq!(
            parsed_row.events[0].as_ref().unwrap().event_type,
            EventType::High
        );
    }

    #[test]
    fn test_json_reporter_file_writing() {
        // Create a temporary directory for test files
        let temp_dir = TempDir::new().unwrap();
        let test_file_path = temp_dir.path().join("test_output.ndjson");

        let harness = Harness::new_with_config(&AnalyzerConfig::default());
        let metadata = harness.get_metadata();

        // Write test data
        let mut file = std::fs::File::create(&test_file_path).unwrap();
        use std::io::Write;

        let metadata_json = serde_json::to_string(&metadata).unwrap();
        writeln!(file, "{}", metadata_json).unwrap();

        let row = AnalysisRow {
            packet_timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z").unwrap(),
            ),
            skipped_message_reason: None,
            events: vec![Some(Event {
                event_type: EventType::High,
                message: "Test warning".to_string(),
            })],
        };
        let row_json = serde_json::to_string(&row).unwrap();
        writeln!(file, "{}", row_json).unwrap();

        drop(file);

        // Read and verify
        let file = std::fs::File::open(&test_file_path).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        assert_eq!(lines.len(), 2);

        // Verify metadata
        let parsed_metadata: ReportMetadata = serde_json::from_str(&lines[0]).unwrap();
        assert!(!parsed_metadata.analyzers.is_empty());

        // Verify row
        let parsed_row: AnalysisRow = serde_json::from_str(&lines[1]).unwrap();
        assert_eq!(parsed_row.events.len(), 1);
        assert_eq!(
            parsed_row.events[0].as_ref().unwrap().message,
            "Test warning"
        );
    }

    #[test]
    fn test_json_reporter_empty_events() {
        let row = AnalysisRow {
            packet_timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z").unwrap(),
            ),
            skipped_message_reason: None,
            events: vec![],
        };

        let row_json = serde_json::to_string(&row).unwrap();
        let parsed: AnalysisRow = serde_json::from_str(&row_json).unwrap();
        assert_eq!(parsed.events.len(), 0);
    }

    #[test]
    fn test_json_reporter_null_events_in_array() {
        // Test that we can have null values in the events array
        let row = AnalysisRow {
            packet_timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z").unwrap(),
            ),
            skipped_message_reason: None,
            events: vec![
                Some(Event {
                    event_type: EventType::High,
                    message: "Test".to_string(),
                }),
                None,
                Some(Event {
                    event_type: EventType::Low,
                    message: "Another test".to_string(),
                }),
            ],
        };

        let row_json = serde_json::to_string(&row).unwrap();
        let parsed: AnalysisRow = serde_json::from_str(&row_json).unwrap();
        assert_eq!(parsed.events.len(), 3);
        assert!(parsed.events[0].is_some());
        assert!(parsed.events[1].is_none());
        assert!(parsed.events[2].is_some());
    }

    #[test]
    fn test_metadata_contains_all_required_fields() {
        let harness = Harness::new_with_config(&AnalyzerConfig::default());
        let metadata = harness.get_metadata();

        // Verify required fields exist
        assert!(!metadata.analyzers.is_empty());
        assert!(!metadata.rayhunter.rayhunter_version.is_empty());
        assert!(!metadata.rayhunter.system_os.is_empty());
        assert!(!metadata.rayhunter.arch.is_empty());

        // Verify each analyzer has required fields
        for analyzer in &metadata.analyzers {
            assert!(!analyzer.name.is_empty());
            assert!(!analyzer.description.is_empty());
            assert!(analyzer.version > 0);
        }
    }

    #[test]
    fn test_json_serialization_roundtrip() {
        // Test that we can serialize and deserialize without data loss
        let harness = Harness::new_with_config(&AnalyzerConfig::default());
        let original_metadata = harness.get_metadata();

        let json = serde_json::to_string(&original_metadata).unwrap();
        let parsed: ReportMetadata = serde_json::from_str(&json).unwrap();

        assert_eq!(original_metadata.analyzers.len(), parsed.analyzers.len());
        assert_eq!(
            original_metadata.rayhunter.rayhunter_version,
            parsed.rayhunter.rayhunter_version
        );
    }

    #[test]
    fn test_multiple_rows_ndjson() {
        // Test writing multiple rows in NDJSON format
        let harness = Harness::new_with_config(&AnalyzerConfig::default());
        let metadata = harness.get_metadata();

        let temp_dir = TempDir::new().unwrap();
        let test_file_path = temp_dir.path().join("multi_rows.ndjson");
        let mut file = std::fs::File::create(&test_file_path).unwrap();

        use std::io::Write;

        // Write metadata
        writeln!(file, "{}", serde_json::to_string(&metadata).unwrap()).unwrap();

        // Write multiple rows
        for i in 0..10 {
            let row = AnalysisRow {
                packet_timestamp: Some(
                    chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z").unwrap(),
                ),
                skipped_message_reason: None,
                events: vec![Some(Event {
                    event_type: EventType::Low,
                    message: format!("Event {}", i),
                })],
            };
            writeln!(file, "{}", serde_json::to_string(&row).unwrap()).unwrap();
        }

        drop(file);

        // Read and verify
        let file = std::fs::File::open(&test_file_path).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        assert_eq!(lines.len(), 11); // 1 metadata + 10 rows

        // Verify first line is metadata
        let _metadata: ReportMetadata = serde_json::from_str(&lines[0]).unwrap();

        // Verify remaining lines are rows
        for i in 1..=10 {
            let row: AnalysisRow = serde_json::from_str(&lines[i]).unwrap();
            assert_eq!(row.events.len(), 1);
            assert_eq!(
                row.events[0].as_ref().unwrap().message,
                format!("Event {}", i - 1)
            );
        }
    }
}
