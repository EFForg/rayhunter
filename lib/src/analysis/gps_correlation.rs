//! GPS correlation for analysis results
//! 
//! This module provides functionality to correlate GPS coordinates with
//! analysis results based on timestamps, allowing analysis events to
//! include location information.

use chrono::{DateTime, Local, Duration};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsCoordinate {
    pub timestamp: DateTime<Local>,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsCorrelation {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy_meters: Option<f64>,
    pub correlation_method: String,
}

impl GpsCorrelation {
    pub fn new() -> Self {
        Self {
            latitude: None,
            longitude: None,
            accuracy_meters: None,
            correlation_method: "none".to_string(),
        }
    }

    pub fn with_coordinates(lat: f64, lon: f64, method: &str) -> Self {
        Self {
            latitude: Some(lat),
            longitude: Some(lon),
            accuracy_meters: None,
            correlation_method: method.to_string(),
        }
    }

    pub fn with_accuracy(lat: f64, lon: f64, accuracy: f64, method: &str) -> Self {
        Self {
            latitude: Some(lat),
            longitude: Some(lon),
            accuracy_meters: Some(accuracy),
            correlation_method: method.to_string(),
        }
    }
}

/// GPS correlation service for analysis results
pub struct AnalysisGpsCorrelator {
    gps_entries: Vec<GpsCoordinate>,
    loaded: bool,
}

impl AnalysisGpsCorrelator {
    pub fn new() -> Self {
        Self {
            gps_entries: Vec::new(),
            loaded: false,
        }
    }

    /// Load GPS data from a GPS file for correlation with analysis
    pub async fn load_gps_data<P: AsRef<Path>>(&mut self, gps_file_path: P) -> Result<usize, io::Error> {
        let path = gps_file_path.as_ref();
        
        if !path.exists() {
            self.loaded = false;
            return Ok(0);
        }

        let content = fs::read_to_string(path).await?;

        let mut entries = Vec::new();
        for line in content.lines() {
            if let Some(coord) = self.parse_gps_line(line) {
                entries.push(coord);
            }
        }

        // Sort by timestamp to ensure chronological order
        entries.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        self.gps_entries = entries;
        self.loaded = true;
        
        Ok(self.gps_entries.len())
    }

    /// Parse a GPS file line into a GpsCoordinate
    fn parse_gps_line(&self, line: &str) -> Option<GpsCoordinate> {
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

        Some(GpsCoordinate {
            timestamp,
            latitude,
            longitude,
        })
    }

    /// Find the closest GPS coordinate for a given timestamp
    pub fn find_closest_gps(&self, target_timestamp: &DateTime<Local>) -> Option<GpsCorrelation> {
        if !self.loaded || self.gps_entries.is_empty() {
            return Some(GpsCorrelation::new());
        }

        // Find the closest GPS entry by timestamp
        let mut closest_entry = &self.gps_entries[0];
        let mut min_diff = (*target_timestamp - closest_entry.timestamp).abs();

        for entry in &self.gps_entries[1..] {
            let diff = (*target_timestamp - entry.timestamp).abs();
            if diff < min_diff {
                min_diff = diff;
                closest_entry = entry;
            }
        }

        // Only correlate if the GPS data is within 30 seconds of the analysis timestamp
        let max_tolerance = Duration::seconds(30);
        if min_diff <= max_tolerance {
            let accuracy = if min_diff <= Duration::seconds(5) {
                Some(10.0) // High accuracy for very close timestamps
            } else if min_diff <= Duration::seconds(15) {
                Some(50.0) // Medium accuracy
            } else {
                Some(100.0) // Lower accuracy for distant timestamps
            };

            Some(GpsCorrelation::with_accuracy(
                closest_entry.latitude,
                closest_entry.longitude,
                accuracy.unwrap_or(100.0),
                "closest_timestamp"
            ))
        } else {
            Some(GpsCorrelation::new())
        }
    }

    /// Find GPS coordinates within a time window
    pub fn find_gps_in_window(
        &self,
        target_timestamp: &DateTime<Local>,
        window_seconds: i64,
    ) -> Vec<GpsCoordinate> {
        if !self.loaded {
            return Vec::new();
        }

        let window = Duration::seconds(window_seconds);
        let start_time = *target_timestamp - window;
        let end_time = *target_timestamp + window;

        self.gps_entries
            .iter()
            .filter(|entry| entry.timestamp >= start_time && entry.timestamp <= end_time)
            .cloned()
            .collect()
    }

    /// Get statistics about the loaded GPS data
    pub fn get_stats(&self) -> GpsCorrelationStats {
        if !self.loaded || self.gps_entries.is_empty() {
            return GpsCorrelationStats {
                total_entries: 0,
                time_span: None,
                coverage_percentage: 0.0,
            };
        }

        let first_timestamp = self.gps_entries.first().unwrap().timestamp;
        let last_timestamp = self.gps_entries.last().unwrap().timestamp;
        let time_span = last_timestamp - first_timestamp;

        GpsCorrelationStats {
            total_entries: self.gps_entries.len(),
            time_span: Some(time_span),
            coverage_percentage: 100.0, // Simplified for now
        }
    }

    /// Check if GPS data is loaded and available
    pub fn is_loaded(&self) -> bool {
        self.loaded && !self.gps_entries.is_empty()
    }
}

#[derive(Debug, Serialize)]
pub struct GpsCorrelationStats {
    pub total_entries: usize,
    pub time_span: Option<Duration>,
    pub coverage_percentage: f64,
} 