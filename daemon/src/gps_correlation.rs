//! GPS data correlation with recording sessions
//! 
//! This module provides functionality to correlate GPS coordinates with
//! recording sessions based on timestamps, allowing users to download
//! GPS logs that correspond to specific recording time windows.

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use anyhow::{Result, Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsEntry {
    pub timestamp: DateTime<Local>,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GpsCorrelationResult {
    pub recording_id: String,
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
    pub gps_entries: Vec<GpsEntry>,
    pub total_entries: usize,
}

/// GPS correlation service for matching GPS data to recording sessions
#[allow(dead_code)]
pub struct GpsCorrelator {
    gps_data_path: PathBuf,
}

#[allow(dead_code)]
impl GpsCorrelator {
    pub fn new<P: AsRef<Path>>(gps_data_path: P) -> Self {
        Self {
            gps_data_path: gps_data_path.as_ref().to_path_buf(),
        }
    }

    /// Get GPS data that correlates with a specific recording session
    pub async fn get_gps_for_recording(
        &self,
        recording_id: &str,
        start_time: DateTime<Local>,
        end_time: Option<DateTime<Local>>,
    ) -> Result<GpsCorrelationResult> {
        let gps_entries = self.load_gps_for_recording(recording_id).await?;

        Ok(GpsCorrelationResult {
            recording_id: recording_id.to_string(),
            start_time,
            end_time,
            gps_entries: gps_entries.clone(),
            total_entries: gps_entries.len(),
        })
    }

    /// Load GPS entries for a specific recording session
    async fn load_gps_for_recording(&self, recording_id: &str) -> Result<Vec<GpsEntry>> {
        let gps_file_path = self.gps_data_path.join(format!("{}.gps", recording_id));
        
        if !gps_file_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&gps_file_path).await
            .context("Failed to read GPS file")?;

        let mut entries = Vec::new();
        for line in content.lines() {
            if let Some(entry) = self.parse_gps_line(line) {
                entries.push(entry);
            }
        }

        // Sort by timestamp to ensure chronological order
        entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        Ok(entries)
    }

    // Legacy CSV and JSON loading methods removed - all GPS data now uses per-scan files with UNIX timestamps

    /// Parse a GPS file line into a GpsEntry (format: "formatted_timestamp, latitude, longitude")
    fn parse_gps_line(&self, line: &str) -> Option<GpsEntry> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            return None;
        }

        // Parse formatted timestamp
        let timestamp_str = parts[0].trim();
        let timestamp = chrono::DateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S%.3f UTC")
            .or_else(|_| chrono::DateTime::parse_from_rfc3339(timestamp_str))
            .ok()?
            .with_timezone(&Local);
        
        let latitude: f64 = parts[1].trim().parse().ok()?;
        let longitude: f64 = parts[2].trim().parse().ok()?;

        Some(GpsEntry {
            timestamp,
            latitude,
            longitude,
        })
    }

    // CSV line parsing method removed - all GPS data now uses UNIX timestamps

    /// Filter GPS entries by timeframe with some tolerance
    fn filter_gps_by_timeframe(
        &self,
        entries: &[GpsEntry],
        start_time: DateTime<Local>,
        end_time: Option<DateTime<Local>>,
    ) -> Vec<GpsEntry> {
        // Add 5-minute buffer before start and after end to account for timing differences
        let buffer = chrono::Duration::minutes(5);
        let buffered_start = start_time - buffer;
        let buffered_end = end_time.map(|et| et + buffer).unwrap_or_else(|| Local::now() + buffer);

        entries.iter()
            .filter(|entry| {
                entry.timestamp >= buffered_start && entry.timestamp <= buffered_end
            })
            .cloned()
            .collect()
    }

    /// Generate a GPS file for a specific recording session
    pub async fn generate_gps_file(
        &self,
        correlation: &GpsCorrelationResult,
        format: GpsFileFormat,
    ) -> Result<String> {
        match format {
            GpsFileFormat::Csv => self.generate_csv_content(correlation),
            GpsFileFormat::Json => self.generate_json_content(correlation),
            GpsFileFormat::Gpx => self.generate_gpx_content(correlation),
        }
    }

    /// Generate CSV content for GPS data
    fn generate_csv_content(&self, correlation: &GpsCorrelationResult) -> Result<String> {
        let mut content = String::from("timestamp,latitude,longitude\n");
        
        for entry in &correlation.gps_entries {
            content.push_str(&format!(
                "{},{},{}\n",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S%.3f UTC"),
                entry.latitude,
                entry.longitude
            ));
        }

        Ok(content)
    }

    /// Generate JSON content for GPS data
    fn generate_json_content(&self, correlation: &GpsCorrelationResult) -> Result<String> {
        let json_data = serde_json::to_string_pretty(correlation)
            .context("Failed to serialize GPS correlation data")?;
        Ok(json_data)
    }

    /// Generate GPX content for GPS data (for use with mapping software)
    fn generate_gpx_content(&self, correlation: &GpsCorrelationResult) -> Result<String> {
        let mut gpx = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<gpx version="1.1" creator="Rayhunter" xmlns="http://www.topografix.com/GPX/1/1">
  <metadata>
    <name>Rayhunter GPS Track</name>
    <desc>GPS coordinates correlated with recording session</desc>
  </metadata>
  <trk>
    <name>"#);
        
        gpx.push_str(&format!("Recording {}", correlation.recording_id));
        gpx.push_str(r#"</name>
    <desc>GPS track for Rayhunter recording session</desc>
    <trkseg>
"#);

        for entry in &correlation.gps_entries {
            gpx.push_str(&format!(
                r#"      <trkpt lat="{}" lon="{}">
        <time>{}</time>
      </trkpt>
"#,
                entry.latitude,
                entry.longitude,
                entry.timestamp.to_rfc3339()
            ));
        }

        gpx.push_str(r#"    </trkseg>
  </trk>
</gpx>"#);

        Ok(gpx)
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum GpsFileFormat {
    Csv,
    Json,
    Gpx,
}

#[allow(dead_code)]
impl GpsFileFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "csv" => Some(Self::Csv),
            "json" => Some(Self::Json),
            "gpx" => Some(Self::Gpx),
            _ => None,
        }
    }

    pub fn content_type(&self) -> &'static str {
        match self {
            Self::Csv => "text/csv",
            Self::Json => "application/json",
            Self::Gpx => "application/gpx+xml",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        match self {
            Self::Csv => "csv",
            Self::Json => "json",
            Self::Gpx => "gpx",
        }
    }
}
