//! Enhanced analysis with GPS correlation
//! 
//! This module provides enhanced analysis functionality that correlates
//! analysis results with GPS coordinates based on timestamps.

use std::sync::Arc;
use std::{future, pin};

use axum::Json;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use futures::TryStreamExt;
use log::{debug, error, info, warn};
use rayhunter::analysis::analyzer::{AnalyzerConfig, Harness, AnalysisRow, PacketAnalysis};
use rayhunter::diag::{DataType, MessagesContainer};
use rayhunter::qmdl::QmdlReader;
use serde::{Serialize, Deserialize};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::mpsc::Receiver;
use tokio::sync::{RwLock, RwLockWriteGuard};
use tokio_util::task::TaskTracker;
use chrono::{DateTime, Local};

use crate::dummy_analyzer::TestAnalyzer;
use crate::qmdl_store::RecordingStore;
use crate::server::ServerState;
use crate::gps_correlation::GpsCorrelator;

/// Enhanced GPS correlation data for analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedGpsCorrelation {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy_meters: Option<f64>,
    pub correlation_method: String,
    pub time_difference_seconds: Option<i64>,
}

impl EnhancedGpsCorrelation {
    pub fn new() -> Self {
        Self {
            latitude: None,
            longitude: None,
            accuracy_meters: None,
            correlation_method: "none".to_string(),
            time_difference_seconds: None,
        }
    }

    pub fn with_coordinates(lat: f64, lon: f64, method: &str, time_diff: i64) -> Self {
        Self {
            latitude: Some(lat),
            longitude: Some(lon),
            accuracy_meters: None,
            correlation_method: method.to_string(),
            time_difference_seconds: Some(time_diff),
        }
    }

    pub fn with_accuracy(lat: f64, lon: f64, accuracy: f64, method: &str, time_diff: i64) -> Self {
        Self {
            latitude: Some(lat),
            longitude: Some(lon),
            accuracy_meters: Some(accuracy),
            correlation_method: method.to_string(),
            time_difference_seconds: Some(time_diff),
        }
    }
}

/// Enhanced packet analysis with GPS correlation
#[derive(Serialize, Debug, Clone)]
pub struct EnhancedPacketAnalysis {
    pub timestamp: DateTime<chrono::FixedOffset>,
    pub events: Vec<Option<rayhunter::analysis::analyzer::Event>>,
    pub gps_correlation: EnhancedGpsCorrelation,
}

/// Enhanced analysis row with GPS correlation
#[derive(Serialize, Debug)]
pub struct EnhancedAnalysisRow {
    pub timestamp: DateTime<chrono::FixedOffset>,
    pub skipped_message_reasons: Vec<String>,
    pub analysis: Vec<EnhancedPacketAnalysis>,
    pub gps_stats: Option<GpsAnalysisStats>,
}

#[derive(Serialize, Debug)]
pub struct GpsAnalysisStats {
    pub total_gps_entries: usize,
    pub correlated_events: usize,
    pub correlation_rate: f64,
    pub gps_time_span: Option<String>,
}

/// Enhanced analysis writer with GPS correlation
pub struct EnhancedAnalysisWriter {
    writer: BufWriter<File>,
    harness: Harness,
    bytes_written: usize,
    gps_correlator: Option<GpsCorrelator>,
    recording_name: Option<String>,
    gps_stats: Option<GpsAnalysisStats>,
}

impl EnhancedAnalysisWriter {
    pub async fn new(
        file: File,
        enable_dummy_analyzer: bool,
        analyzer_config: &AnalyzerConfig,
        recording_name: Option<String>,
        gps_file_path: Option<String>,
    ) -> Result<Self, std::io::Error> {
        let mut harness = Harness::new_with_config(analyzer_config);
        if enable_dummy_analyzer {
            harness.add_analyzer(Box::new(TestAnalyzer { count: 0 }));
        }

        // Initialize GPS correlator if GPS file path is provided
        let gps_correlator = if let Some(gps_path) = gps_file_path {
            let correlator = GpsCorrelator::new(gps_path);
            info!("GPS correlator initialized for recording: {}", 
                  recording_name.as_ref().unwrap_or(&"unknown".to_string()));
            Some(correlator)
        } else {
            None
        };

        let mut result = Self {
            writer: BufWriter::new(file),
            bytes_written: 0,
            harness,
            gps_correlator,
            recording_name,
            gps_stats: None,
        };

        let metadata = result.harness.get_metadata();
        result.write(&metadata).await?;
        Ok(result)
    }

    /// Analyze QMDL messages with GPS correlation
    pub async fn analyze_with_gps(
        &mut self,
        container: MessagesContainer,
    ) -> Result<(usize, bool), std::io::Error> {
        let mut row = self.harness.analyze_qmdl_messages(container);
        
        // Convert to enhanced analysis with GPS correlation
        let enhanced_row = self.enhance_analysis_with_gps(row).await;
        
        if !enhanced_row.analysis.is_empty() {
            self.write(&enhanced_row).await?;
        }
        
        Ok((self.bytes_written, enhanced_row.analysis.iter().any(|analysis| {
            analysis.events.iter().any(|event| {
                event.as_ref().map(|e| matches!(e.event_type, rayhunter::analysis::analyzer::EventType::QualitativeWarning { .. })).unwrap_or(false)
            })
        })))
    }

    /// Enhance analysis results with GPS correlation
    async fn enhance_analysis_with_gps(&mut self, row: AnalysisRow) -> EnhancedAnalysisRow {
        let mut enhanced_analysis = Vec::new();
        let mut correlated_events = 0;
        let total_events = row.analysis.len();

        for packet_analysis in row.analysis {
            let gps_correlation = if let Some(ref correlator) = self.gps_correlator {
                // Convert timestamp to Local timezone for GPS correlation
                let local_timestamp = packet_analysis.timestamp.with_timezone(&Local);
                
                // Find closest GPS coordinate
                if let Some(gps_entries) = correlator.get_gps_for_recording(
                    &self.recording_name.as_ref().unwrap_or(&"unknown".to_string()),
                    local_timestamp,
                    None
                ).await.ok() {
                    if !gps_entries.gps_entries.is_empty() {
                        // Find closest GPS entry by timestamp
                        let mut closest_entry = &gps_entries.gps_entries[0];
                        let mut min_diff = (local_timestamp - closest_entry.timestamp).abs();

                        for entry in &gps_entries.gps_entries[1..] {
                            let diff = (local_timestamp - entry.timestamp).abs();
                            if diff < min_diff {
                                min_diff = diff;
                                closest_entry = entry;
                            }
                        }

                        // Only correlate if GPS data is within 30 seconds
                        let max_tolerance = chrono::Duration::seconds(30);
                        if min_diff <= max_tolerance {
                            correlated_events += 1;
                            let accuracy = if min_diff <= chrono::Duration::seconds(5) {
                                10.0
                            } else if min_diff <= chrono::Duration::seconds(15) {
                                50.0
                            } else {
                                100.0
                            };

                            EnhancedGpsCorrelation::with_accuracy(
                                closest_entry.latitude,
                                closest_entry.longitude,
                                accuracy,
                                "closest_timestamp",
                                min_diff.num_seconds()
                            )
                        } else {
                            EnhancedGpsCorrelation::new()
                        }
                    } else {
                        EnhancedGpsCorrelation::new()
                    }
                } else {
                    EnhancedGpsCorrelation::new()
                }
            } else {
                EnhancedGpsCorrelation::new()
            };

            enhanced_analysis.push(EnhancedPacketAnalysis {
                timestamp: packet_analysis.timestamp,
                events: packet_analysis.events,
                gps_correlation,
            });
        }

        // Calculate GPS statistics
        let gps_stats = if let Some(ref correlator) = self.gps_correlator {
            if let Ok(gps_data) = correlator.get_gps_for_recording(
                &self.recording_name.as_ref().unwrap_or(&"unknown".to_string()),
                Local::now(),
                None
            ).await {
                let correlation_rate = if total_events > 0 {
                    (correlated_events as f64 / total_events as f64) * 100.0
                } else {
                    0.0
                };

                let time_span = if let (Some(first), Some(last)) = (gps_data.gps_entries.first(), gps_data.gps_entries.last()) {
                    let span = last.timestamp - first.timestamp;
                    Some(format!("{} minutes", span.num_minutes()))
                } else {
                    None
                };

                Some(GpsAnalysisStats {
                    total_gps_entries: gps_data.total_entries,
                    correlated_events,
                    correlation_rate,
                    gps_time_span: time_span,
                })
            } else {
                None
            }
        } else {
            None
        };

        EnhancedAnalysisRow {
            timestamp: row.timestamp,
            skipped_message_reasons: row.skipped_message_reasons,
            analysis: enhanced_analysis,
            gps_stats,
        }
    }

    async fn write<T: Serialize>(&mut self, value: &T) -> Result<(), std::io::Error> {
        let mut value_str = serde_json::to_string(value).unwrap();
        value_str.push('\n');
        self.bytes_written += value_str.len();
        self.writer.write_all(value_str.as_bytes()).await?;
        self.writer.flush().await?;
        Ok(())
    }

    pub async fn close(mut self) -> Result<(), std::io::Error> {
        self.writer.flush().await?;
        Ok(())
    }
} 