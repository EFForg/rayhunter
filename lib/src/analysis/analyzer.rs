use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::util::RuntimeMetadata;
use crate::{diag::MessagesContainer, gsmtap_parser};

use super::{
    cellular_data::CellularData,
    connection_redirect_downgrade::ConnectionRedirect2GDowngradeAnalyzer,
    imsi_requested::ImsiRequestedAnalyzer, information_element::InformationElement,
    null_cipher::NullCipherAnalyzer, priority_2g_downgrade::LteSib6And7DowngradeAnalyzer,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct AnalyzerConfig {
    pub imsi_requested: bool,
    pub connection_redirect_2g_downgrade: bool,
    pub lte_sib6_and_7_downgrade: bool,
    pub null_cipher: bool,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        AnalyzerConfig {
            imsi_requested: true,
            connection_redirect_2g_downgrade: true,
            lte_sib6_and_7_downgrade: true,
            null_cipher: true,
        }
    }
}

/// Qualitative measure of how severe a Warning event type is.
/// The levels should break down like this:
///   * Low: if combined with a large number of other Warnings, user should investigate
///   * Medium: if combined with a few other Warnings, user should investigate
///   * High: user should investigate
#[derive(Serialize, Debug, Clone)]
pub enum Severity {
    Low,
    Medium,
    High,
}

/// `QualitativeWarning` events will always be shown to the user in some manner,
/// while `Informational` ones may be hidden based on user settings.
#[derive(Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum EventType {
    Informational,
    QualitativeWarning { severity: Severity },
}

/// Events are user-facing signals that can be emitted by an [Analyzer] upon a
/// message being received. They can be used to signifiy an IC detection
/// warning, or just to display some relevant information to the user.
#[derive(Serialize, Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub message: String,
}

/// An [Analyzer] represents one type of heuristic for detecting an IMSI Catcher
/// (IC). While maintaining some amount of state is useful, be mindful of how
/// much memory your [Analyzer] uses at runtime, since rayhunter may run for
/// many hours at a time with dozens of [Analyzers](Analyzer) working in parallel.
pub trait Analyzer {
    /// Returns a user-friendly, concise name for your heuristic.
    fn get_name(&self) -> Cow<str>;

    /// Returns a user-friendly description of what your heuristic looks for,
    /// the types of [Events](Event) it may return, as well as possible false-positive
    /// conditions that may trigger an [Event]. If different [Events](Event) have
    /// different false-positive conditions, consider including them in its
    /// `message` field.
    fn get_description(&self) -> Cow<str>;

    /// Analyze a single [InformationElement], possibly returning an [Event] if your
    /// heuristic deems it relevant. Again, be mindful of any state your
    /// [Analyzer] updates per message, since it may be run over hundreds or
    /// thousands of them alongside many other [Analyzers](Analyzer).
    fn analyze_information_element(&mut self, ie: &InformationElement) -> Option<Event>;
}

#[derive(Serialize, Debug)]
pub struct AnalyzerMetadata {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Debug)]
pub struct ReportMetadata {
    pub analyzers: Vec<AnalyzerMetadata>,
    pub rayhunter: RuntimeMetadata,
}

#[derive(Serialize, Debug, Clone)]
pub struct PacketAnalysis {
    pub timestamp: DateTime<FixedOffset>,
    pub events: Vec<Option<Event>>,
    pub cellular_data: Option<CellularData>,
}

#[derive(Serialize, Debug)]
pub struct AnalysisRow {
    pub timestamp: DateTime<FixedOffset>,
    pub skipped_message_reasons: Vec<String>,
    pub analysis: Vec<PacketAnalysis>,
}

impl AnalysisRow {
    pub fn is_empty(&self) -> bool {
        self.skipped_message_reasons.is_empty() && self.analysis.is_empty()
    }

    pub fn contains_warnings(&self) -> bool {
        for analysis in &self.analysis {
            for event in analysis.events.iter().flatten() {
                if matches!(event.event_type, EventType::QualitativeWarning { .. }) {
                    return true;
                }
            }
        }
        false
    }
}

pub struct Harness {
    analyzers: Vec<Box<dyn Analyzer + Send>>,
}

impl Default for Harness {
    fn default() -> Self {
        Self::new()
    }
}

impl Harness {
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
        }
    }

    pub fn new_with_config(analyzer_config: &AnalyzerConfig) -> Self {
        let mut harness = Harness::new();

        if analyzer_config.imsi_requested {
            harness.add_analyzer(Box::new(ImsiRequestedAnalyzer::new()));
        }
        if analyzer_config.connection_redirect_2g_downgrade {
            harness.add_analyzer(Box::new(ConnectionRedirect2GDowngradeAnalyzer {}));
        }
        if analyzer_config.lte_sib6_and_7_downgrade {
            harness.add_analyzer(Box::new(LteSib6And7DowngradeAnalyzer {}));
        }
        if analyzer_config.null_cipher {
            harness.add_analyzer(Box::new(NullCipherAnalyzer {}));
        }

        harness
    }

    pub fn add_analyzer(&mut self, analyzer: Box<dyn Analyzer + Send>) {
        self.analyzers.push(analyzer);
    }

    pub fn analyze_qmdl_messages(&mut self, container: MessagesContainer) -> AnalysisRow {
        let mut row = AnalysisRow {
            timestamp: chrono::Local::now().fixed_offset(),
            skipped_message_reasons: Vec::new(),
            analysis: Vec::new(),
        };
        for maybe_qmdl_message in container.into_messages() {
            let qmdl_message = match maybe_qmdl_message {
                Ok(msg) => msg,
                Err(err) => {
                    row.skipped_message_reasons.push(format!("{err:?}"));
                    continue;
                }
            };

            let gsmtap_message = match gsmtap_parser::parse(qmdl_message) {
                Ok(msg) => msg,
                Err(err) => {
                    row.skipped_message_reasons.push(format!("{err:?}"));
                    continue;
                }
            };

            let Some((timestamp, gsmtap_msg)) = gsmtap_message else {
                continue;
            };

            let element = match InformationElement::try_from(&gsmtap_msg) {
                Ok(element) => element,
                Err(err) => {
                    row.skipped_message_reasons.push(format!("{err:?}"));
                    continue;
                }
            };

            let analysis_result = self.analyze_information_element(&element);
            let mut cellular_data = CellularData::from_gsmtap_and_ie(&gsmtap_msg, &element);
            
            // Add GPS data if available (placeholder for GPS integration)
            // TODO: Integrate with actual GPS data source
            if let Some(lat) = self.get_gps_latitude() {
                if let Some(lon) = self.get_gps_longitude() {
                    cellular_data.add_gps_location(lat, lon, None, None);
                }
            }
            
            // Perform security analysis
            cellular_data.analyze_security();
            
            // Include packet analysis if we have events OR cellular data OR security threats
            let has_security_threat = cellular_data.security_analysis.as_ref()
                .map(|sa| sa.threat_level != super::cellular_data::ThreatLevel::None)
                .unwrap_or(false);
                
            if analysis_result.iter().any(Option::is_some) || 
               cellular_data.has_cell_identification() || 
               has_security_threat {
                row.analysis.push(PacketAnalysis {
                    timestamp: timestamp.to_datetime(),
                    events: analysis_result,
                    cellular_data: if cellular_data.has_cell_identification() || has_security_threat {
                        Some(cellular_data)
                    } else {
                        None
                    },
                });
            }
        }
        row
    }

    fn analyze_information_element(&mut self, ie: &InformationElement) -> Vec<Option<Event>> {
        self.analyzers
            .iter_mut()
            .map(|analyzer| analyzer.analyze_information_element(ie))
            .collect()
    }

    pub fn get_names(&self) -> Vec<Cow<'_, str>> {
        self.analyzers
            .iter()
            .map(|analyzer| analyzer.get_name())
            .collect()
    }

    pub fn get_descriptions(&self) -> Vec<Cow<'_, str>> {
        self.analyzers
            .iter()
            .map(|analyzer| analyzer.get_description())
            .collect()
    }

    pub fn get_metadata(&self) -> ReportMetadata {
        let names = self.get_names();
        let descriptions = self.get_descriptions();
        let mut analyzers = Vec::new();
        for (name, description) in names.iter().zip(descriptions.iter()) {
            analyzers.push(AnalyzerMetadata {
                name: name.to_string(),
                description: description.to_string(),
            });
        }

        let rayhunter = RuntimeMetadata::new();

        ReportMetadata {
            analyzers,
            rayhunter,
        }
    }

    /// Placeholder for GPS latitude - replace with actual GPS integration
    fn get_gps_latitude(&self) -> Option<f64> {
        // TODO: Integrate with actual GPS data source
        // For now, return None to indicate no GPS data
        None
    }

    /// Placeholder for GPS longitude - replace with actual GPS integration
    fn get_gps_longitude(&self) -> Option<f64> {
        // TODO: Integrate with actual GPS data source
        // For now, return None to indicate no GPS data
        None
    }

    /// Export analysis results to NDJSON file with Unix timestamp
    pub fn export_to_ndjson(&self, analysis_rows: &[AnalysisRow], output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = chrono::Utc::now().timestamp();
        let filename = format!("{}/cellular_analysis_{}.ndjson", output_dir, timestamp);
        let path = Path::new(&filename);
        
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        
        for row in analysis_rows {
            for analysis in &row.analysis {
                if let Some(cellular_data) = &analysis.cellular_data {
                    let json_value = cellular_data.to_ndjson_format();
                    writeln!(writer, "{}", serde_json::to_string(&json_value)?)?;
                }
            }
        }
        
        writer.flush()?;
        println!("NDJSON export completed: {}", filename);
        Ok(())
    }

    /// Export security threats to separate NDJSON file
    pub fn export_security_threats(&self, analysis_rows: &[AnalysisRow], output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let timestamp = chrono::Utc::now().timestamp();
        let filename = format!("{}/security_threats_{}.ndjson", output_dir, timestamp);
        let path = Path::new(&filename);
        
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        
        for row in analysis_rows {
            for analysis in &row.analysis {
                if let Some(cellular_data) = &analysis.cellular_data {
                    if let Some(security) = &cellular_data.security_analysis {
                        if security.threat_level != super::cellular_data::ThreatLevel::None {
                            let mut threat_json = serde_json::Map::new();
                            threat_json.insert("timestamp".to_string(), serde_json::Value::Number(
                                serde_json::Number::from(analysis.timestamp.timestamp())
                            ));
                            threat_json.insert("cell_id".to_string(), serde_json::Value::String(cellular_data.get_cell_id()));
                            threat_json.insert("threat_level".to_string(), serde_json::Value::String(format!("{:?}", security.threat_level)));
                            if let Some(attack_type) = &security.attack_type {
                                threat_json.insert("attack_type".to_string(), serde_json::Value::String(format!("{:?}", attack_type)));
                            }
                            threat_json.insert("confidence".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(security.confidence as f64).unwrap()));
                            threat_json.insert("indicators".to_string(), serde_json::Value::Array(
                                security.indicators.iter().map(|i| serde_json::Value::String(i.clone())).collect()
                            ));
                            
                            // Add GPS location if available
                            if let Some(gps) = &cellular_data.gps_location {
                                let mut gps_json = serde_json::Map::new();
                                if let Some(lat) = gps.latitude {
                                    gps_json.insert("latitude".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(lat).unwrap()));
                                }
                                if let Some(lon) = gps.longitude {
                                    gps_json.insert("longitude".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(lon).unwrap()));
                                }
                                threat_json.insert("gps_location".to_string(), serde_json::Value::Object(gps_json));
                            }
                            
                            writeln!(writer, "{}", serde_json::to_string(&serde_json::Value::Object(threat_json))?)?;
                        }
                    }
                }
            }
        }
        
        writer.flush()?;
        println!("Security threats export completed: {}", filename);
        Ok(())
    }
}
