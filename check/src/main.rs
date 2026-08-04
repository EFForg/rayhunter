use chrono::{DateTime, FixedOffset};
use clap::Parser;
use log::{debug, error, info, warn};
use pcap_file_tokio::pcapng::{Block, PcapNgReader};
use rayhunter::{
    DeviceMetadata,
    analysis::analyzer::{AnalysisRow, AnalyzerConfig, Event, EventType, Harness},
    gsmtap::parser as gsmtap_parser,
    pcap::GsmtapPcapWriter,
    qmdl::QmdlMessageReader,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};
use tokio::fs::File;
use walkdir::WalkDir;

use crate::json::IncrementalJsonWriter;

mod json;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'p', long, help = "A file or directory of packet captures")]
    path: PathBuf,

    #[arg(short = 'P', long, help = "Convert qmdl files to pcap after analysis")]
    pcapify: bool,

    #[arg(long, help = "Show why some packets were skipped during analysis")]
    show_skipped: bool,

    #[arg(short = 'j', long, help = "Output report to a JSON file")]
    json: Option<PathBuf>,

    #[arg(short, long, help = "Only print warnings/errors to stdout")]
    quiet: bool,

    #[arg(short, long, help = "Show debug messages")]
    debug: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct EventWithTimestamp {
    timestamp: DateTime<FixedOffset>,
    event: Event,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
struct Report {
    skipped_reasons: HashMap<String, u32>,
    events: Vec<EventWithTimestamp>,
    total_messages: u32,
    warnings: u32,
    skipped: u32,
    file_path: String,
}

impl Report {
    fn new(file_path: &str) -> Self {
        Report {
            file_path: file_path.to_string(),
            ..Default::default()
        }
    }

    fn process_row(&mut self, row: AnalysisRow) {
        self.total_messages += 1;
        if let Some(reason) = row.skipped_message_reason {
            *self.skipped_reasons.entry(reason).or_insert(0) += 1;
            self.skipped += 1;
            return;
        }
        for maybe_event in row.events {
            let Some(event) = maybe_event else { continue };
            let Some(timestamp) = row.packet_timestamp else {
                continue;
            };
            match event.event_type {
                EventType::Informational => {
                    info!("{}: INFO - {} {}", self.file_path, timestamp, event.message,);
                }
                EventType::Low | EventType::Medium | EventType::High => {
                    warn!(
                        "{}: WARNING (Severity: {:?}) - {} {}",
                        self.file_path, event.event_type, timestamp, event.message,
                    );
                    self.warnings += 1;
                }
            }
            self.events.push(EventWithTimestamp { timestamp, event });
        }
    }

    fn print_summary(&self, show_skipped: bool) {
        if show_skipped && self.skipped > 0 {
            info!("{}: messages skipped:", self.file_path);
            for (reason, count) in self.skipped_reasons.iter() {
                info!("    - {count}: \"{reason}\"");
            }
        }
        info!(
            "{}: {} messages analyzed, {} warnings, {} messages skipped",
            self.file_path, self.total_messages, self.warnings, self.skipped
        );
    }
}

async fn analyze_pcap(
    pcap_path: &str,
    show_skipped: bool,
    json_writer: Option<&mut IncrementalJsonWriter<File>>,
) {
    let mut harness =
        Harness::new_with_config(&AnalyzerConfig::default(), &DeviceMetadata::default());
    let pcap_file = &mut File::open(&pcap_path).await.expect("failed to open file");
    let mut pcap_reader = PcapNgReader::new(pcap_file)
        .await
        .expect("failed to read PCAP file");
    let mut report = Report::new(pcap_path);
    while let Some(Ok(block)) = pcap_reader.next_block().await {
        let row = match block {
            Block::EnhancedPacket(packet) => harness.analyze_pcap_packet(packet),
            other => {
                debug!("{pcap_path}: skipping pcap packet {other:?}");
                continue;
            }
        };
        report.process_row(row);
    }
    report.print_summary(show_skipped);
    if let Some(writer) = json_writer {
        writer
            .write_report(&report)
            .await
            .expect("failed to write report to JSON");
    }
}

async fn analyze_qmdl(
    qmdl_path: &str,
    show_skipped: bool,
    json_writer: Option<&mut IncrementalJsonWriter<File>>,
) {
    let mut harness =
        Harness::new_with_config(&AnalyzerConfig::default(), &DeviceMetadata::default());
    let qmdl_file = &mut File::open(&qmdl_path).await.expect("failed to open file");
    let mut qmdl_reader = QmdlMessageReader::new(qmdl_file)
        .await
        .expect("failed to open QmdlReader");
    let mut report = Report::new(qmdl_path);
    while let Some(maybe_message) = qmdl_reader
        .get_next_message()
        .await
        .expect("failed to get message")
    {
        report.process_row(harness.analyze_qmdl_message(maybe_message));
    }
    report.print_summary(show_skipped);
    if let Some(writer) = json_writer {
        writer
            .write_report(&report)
            .await
            .expect("failed to write report to JSON");
    }
}

async fn pcapify(qmdl_path: &PathBuf) {
    let qmdl_file = &mut File::open(&qmdl_path)
        .await
        .expect("failed to open qmdl file");
    let mut qmdl_reader = QmdlMessageReader::new(qmdl_file)
        .await
        .expect("failed to open QmdlReader");
    let mut pcap_path = qmdl_path.clone();
    pcap_path.set_extension("pcapng");
    let pcap_file = &mut File::create(&pcap_path)
        .await
        .expect("failed to open pcap file");
    let mut pcap_writer = GsmtapPcapWriter::new(pcap_file).await.unwrap();
    pcap_writer.write_iface_header().await.unwrap();
    while let Some(maybe_message) = qmdl_reader
        .get_next_message()
        .await
        .expect("failed to get message")
    {
        if let Ok(msg) = maybe_message
            && let Ok(Some((timestamp, parsed))) = gsmtap_parser::parse(msg)
        {
            pcap_writer
                .write_gsmtap_message(parsed, timestamp, None)
                .await
                .expect("failed to write");
        }
    }
    info!("wrote pcap to {:?}", pcap_path);
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let level = if args.debug {
        log::LevelFilter::Debug
    } else if args.quiet {
        log::LevelFilter::Warn
    } else {
        log::LevelFilter::Info
    };
    rayhunter::init_logging(level);

    let harness = Harness::new_with_config(&AnalyzerConfig::default(), &DeviceMetadata::default());
    let metadata = harness.get_metadata();
    info!("Analyzers:");
    for analyzer in &metadata.analyzers {
        info!(
            "    - {} (v{}): {}",
            analyzer.name, analyzer.version, analyzer.description
        );
    }

    let mut json_writer = None;
    if let Some(json_path) = &args.json {
        let json_file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(json_path)
            .await
            .expect("failed to create JSON output file");
        let check_path_str = args.path.to_string_lossy();
        json_writer = Some(
            IncrementalJsonWriter::new(json_file, &check_path_str, &metadata)
                .await
                .expect("failed to create JSON writer"),
        );
    }

    for maybe_entry in WalkDir::new(&args.path) {
        let Ok(entry) = maybe_entry else {
            error!("failed to open dir entry {maybe_entry:?}");
            continue;
        };
        let name = entry.file_name();
        let name_str = name.to_str().unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        if name_str.ends_with(".qmdl") || name_str.ends_with(".qmdl.gz") {
            info!("**** Beginning analysis of {name_str}");
            analyze_qmdl(path_str, args.show_skipped, json_writer.as_mut()).await;
            if args.pcapify {
                pcapify(&path.to_path_buf()).await;
            }
        } else if name_str.ends_with(".pcap") || name_str.ends_with(".pcapng") {
            // TODO: if we've already analyzed a QMDL, skip its corresponding pcap
            info!("**** Beginning analysis of {name_str}");
            analyze_pcap(path_str, args.show_skipped, json_writer.as_mut()).await;
        }
    }

    if let Some(writer) = json_writer {
        // if we have a json_writer, we also have args.json
        info!("Writing report to {:?}", args.json.unwrap());
        writer
            .finish()
            .await
            .expect("failed to finish writing to JSON file");
    }
}
