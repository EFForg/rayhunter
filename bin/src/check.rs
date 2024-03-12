use std::{future, path::PathBuf, pin::pin};
use chrono::{DateTime, FixedOffset};
use rayhunter::{analysis::{analyzer::{Event, EventType, Harness}, information_element::InformationElement, lte_downgrade::LteSib6And7DowngradeAnalyzer}, diag::DataType, gsmtap_parser::GsmtapParser, qmdl::QmdlReader};
use tokio::fs::File;
use serde::Serialize;
use clap::Parser;
use futures::TryStreamExt;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    qmdl_path: PathBuf,
}

#[derive(Serialize, Debug)]
struct AnalyzerMetadata {
    name: String,
    description: String,
}

#[derive(Serialize, Debug)]
struct ReportMetadata {
    num_packets_analyzed: usize,
    num_packets_skipped: usize,
    num_warnings: usize,
    first_packet_time: DateTime<FixedOffset>,
    last_packet_time: DateTime<FixedOffset>,
    analyzers: Vec<AnalyzerMetadata>,
}

#[derive(Serialize, Debug)]
struct PacketAnalysis {
    timestamp: DateTime<FixedOffset>,
    events: Vec<Option<Event>>,
}

#[derive(Serialize, Debug)]
struct AnalysisReport {
    metadata: ReportMetadata,
    analysis: Vec<PacketAnalysis>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut harness = Harness::new();
    harness.add_analyzer(Box::new(LteSib6And7DowngradeAnalyzer{}));

    let mut num_packets_analyzed = 0;
    let mut num_warnings = 0;
    let mut first_packet_time: Option<DateTime<FixedOffset>> = None;
    let mut last_packet_time: Option<DateTime<FixedOffset>> = None;
    let mut skipped_message_reasons: Vec<String> = Vec::new();
    let mut analysis: Vec<PacketAnalysis> = Vec::new();
    let mut analyzers: Vec<AnalyzerMetadata> = Vec::new();

    let names = harness.get_names();
    let descriptions = harness.get_names();
    for (name, description) in names.iter().zip(descriptions.iter()) {
        analyzers.push(AnalyzerMetadata {
            name: name.to_string(),
            description: description.to_string(),
        });
    }

    let qmdl_file = File::open(args.qmdl_path).await.expect("failed to open QMDL file");
    let file_size = qmdl_file.metadata().await.expect("failed to get QMDL file metadata").len();
    let mut gsmtap_parser = GsmtapParser::new();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(file_size as usize));
    let mut qmdl_stream = pin!(qmdl_reader.as_stream()
        .try_filter(|container| future::ready(container.data_type == DataType::UserSpace)));
    while let Some(container) = qmdl_stream.try_next().await.expect("failed getting QMDL container") {
        for maybe_qmdl_message in container.into_messages() {
            let qmdl_message = match maybe_qmdl_message {
                Ok(msg) => msg,
                Err(err) => {
                    skipped_message_reasons.push(format!("{:?}", err));
                    continue;
                }
            };
            let gsmtap_message = match gsmtap_parser.parse(qmdl_message) {
                Ok(msg) => msg,
                Err(err) => {
                    skipped_message_reasons.push(format!("{:?}", err));
                    continue;
                }
            };
            let Some((timestamp, gsmtap_msg)) = gsmtap_message else {
                continue;
            };
            let element = match InformationElement::try_from(&gsmtap_msg) {
                Ok(element) => element,
                Err(err) => {
                    skipped_message_reasons.push(format!("{:?}", err));
                    continue;
                }
            };
            if first_packet_time.is_none() {
                first_packet_time = Some(timestamp.to_datetime());
            }
            last_packet_time = Some(timestamp.to_datetime());
            num_packets_analyzed += 1;
            let analysis_result = harness.analyze_information_element(&element);
            if analysis_result.iter().any(Option::is_some) {
                num_warnings += analysis_result.iter()
                    .filter(|maybe_event| matches!(maybe_event, Some(Event { event_type: EventType::QualitativeWarning { .. }, .. })))
                    .count();
                analysis.push(PacketAnalysis {
                    timestamp: timestamp.to_datetime(),
                    events: analysis_result,
                });
            }
        }
    }

    let report = AnalysisReport {
        metadata: ReportMetadata {
            num_packets_analyzed,
            num_packets_skipped: skipped_message_reasons.len(),
            num_warnings,
            first_packet_time: first_packet_time.expect("no packet times set"),
            last_packet_time: last_packet_time.expect("no packet times set"),
            analyzers,
        },
        analysis,
    };

    println!("{}", serde_json::to_string(&report).expect("failed to serialize report"));
}
