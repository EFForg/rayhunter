use clap::Parser;
use futures::TryStreamExt;
use log::{debug, error, info, warn};
use pcap_file_tokio::pcapng::{Block, PcapNgReader};
use rayhunter::{
    analysis::analyzer::{AnalysisRow, AnalyzerConfig, EventType, Harness, ReportMetadata},
    diag::DataType,
    gsmtap_parser,
    pcap::GsmtapPcapWriter,
    qmdl::QmdlReader,
};
use std::{collections::HashMap, future, io::Write, path::PathBuf, pin::pin};
use tokio::fs::File;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'p', long, help = "A file or directory of packet captures")]
    path: PathBuf,

    #[arg(short = 'P', long, help = "Convert qmdl files to pcap before analysis")]
    pcapify: bool,

    #[arg(long, help = "Show why some packets were skipped during analysis")]
    show_skipped: bool,

    #[arg(short, long, help = "Only print warnings/errors to stdout")]
    quiet: bool,

    #[arg(short, long, help = "Show debug messages")]
    debug: bool,

    #[arg(long, value_enum, default_value = "text", help = "Output format")]
    format: OutputFormat,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

trait Reporter {
    fn process_row(&mut self, row: AnalysisRow);
    fn finish(&mut self);
}

struct TextReporter {
    skipped_reasons: HashMap<String, u32>,
    total_messages: u32,
    warnings: u32,
    skipped: u32,
    file_path: String,
    show_skipped: bool,
}

impl TextReporter {
    fn new(file_path: &str, show_skipped: bool) -> Self {
        TextReporter {
            file_path: file_path.to_string(),
            skipped_reasons: HashMap::new(),
            total_messages: 0,
            warnings: 0,
            skipped: 0,
            show_skipped,
        }
    }
}

impl Reporter for TextReporter {
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
        }
    }

    fn finish(&mut self) {
        if self.show_skipped && self.skipped > 0 {
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

struct JsonReporter {
    rows: Vec<AnalysisRow>,
    metadata: ReportMetadata,
}

impl JsonReporter {
    fn new(_file_path: &str, metadata: ReportMetadata) -> Self {
        JsonReporter {
            rows: Vec::new(),
            metadata,
        }
    }

    fn write_to_file(&self, output_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::create(output_path)?;

        // Write metadata as first line
        let metadata_str = serde_json::to_string(&self.metadata)?;
        writeln!(file, "{}", metadata_str)?;

        // Write each row as a separate line
        for row in &self.rows {
            let row_str = serde_json::to_string(row)?;
            writeln!(file, "{}", row_str)?;
        }

        Ok(())
    }
}

impl Reporter for JsonReporter {
    fn process_row(&mut self, row: AnalysisRow) {
        self.rows.push(row);
    }

    fn finish(&mut self) {
        // For JSON to stdout, output entire NDJSON
        let metadata_str = serde_json::to_string(&self.metadata).unwrap();
        println!("{}", metadata_str);

        for row in &self.rows {
            let row_str = serde_json::to_string(row).unwrap();
            println!("{}", row_str);
        }
    }
}

async fn analyze_pcap(
    pcap_path: &str,
    show_skipped: bool,
    format: OutputFormat,
    harness_metadata: &ReportMetadata,
) {
    let mut harness = Harness::new_with_config(&AnalyzerConfig::default());
    let pcap_file = &mut File::open(&pcap_path).await.expect("failed to open file");
    let mut pcap_reader = PcapNgReader::new(pcap_file)
        .await
        .expect("failed to read PCAP file");

    let mut reporter: Box<dyn Reporter> = match format {
        OutputFormat::Text => Box::new(TextReporter::new(pcap_path, show_skipped)),
        OutputFormat::Json => Box::new(JsonReporter::new(pcap_path, harness_metadata.clone())),
    };

    while let Some(Ok(block)) = pcap_reader.next_block().await {
        let row = match block {
            Block::EnhancedPacket(packet) => harness.analyze_pcap_packet(packet),
            other => {
                debug!("{pcap_path}: skipping pcap packet {other:?}");
                continue;
            }
        };
        reporter.process_row(row);
    }
    reporter.finish();
}

async fn analyze_qmdl(
    qmdl_path: &str,
    show_skipped: bool,
    format: OutputFormat,
    harness_metadata: &ReportMetadata,
) {
    let mut harness = Harness::new_with_config(&AnalyzerConfig::default());
    let qmdl_file = &mut File::open(&qmdl_path).await.expect("failed to open file");
    let file_size = qmdl_file
        .metadata()
        .await
        .expect("failed to get QMDL file metadata")
        .len();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(file_size as usize));
    let mut qmdl_stream = pin!(
        qmdl_reader
            .as_stream()
            .try_filter(|container| future::ready(container.data_type == DataType::UserSpace))
    );

    let mut reporter: Box<dyn Reporter> = match format {
        OutputFormat::Text => Box::new(TextReporter::new(qmdl_path, show_skipped)),
        OutputFormat::Json => Box::new(JsonReporter::new(qmdl_path, harness_metadata.clone())),
    };

    while let Some(container) = qmdl_stream
        .try_next()
        .await
        .expect("failed getting QMDL container")
    {
        for row in harness.analyze_qmdl_messages(container) {
            reporter.process_row(row);
        }
    }
    reporter.finish();
}

async fn analyze_qmdl_to_json_file(qmdl_path: &PathBuf, harness_metadata: &ReportMetadata) {
    let mut harness = Harness::new_with_config(&AnalyzerConfig::default());
    let qmdl_file = &mut File::open(&qmdl_path)
        .await
        .expect("failed to open qmdl file");
    let file_size = qmdl_file
        .metadata()
        .await
        .expect("failed to get QMDL file metadata")
        .len();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(file_size as usize));
    let mut qmdl_stream = pin!(
        qmdl_reader
            .as_stream()
            .try_filter(|container| future::ready(container.data_type == DataType::UserSpace))
    );

    let mut json_reporter = JsonReporter::new(
        qmdl_path.to_str().unwrap(),
        harness_metadata.clone(),
    );

    while let Some(container) = qmdl_stream
        .try_next()
        .await
        .expect("failed getting QMDL container")
    {
        for row in harness.analyze_qmdl_messages(container) {
            json_reporter.process_row(row);
        }
    }

    // Write to .ndjson file instead of stdout
    let mut output_path = qmdl_path.clone();
    output_path.set_extension("ndjson");

    match json_reporter.write_to_file(&output_path) {
        Ok(_) => info!("Wrote analysis to {:?}", output_path),
        Err(e) => error!("Failed to write analysis file: {}", e),
    }
}

async fn analyze_pcap_to_json_file(pcap_path: &PathBuf, harness_metadata: &ReportMetadata) {
    let mut harness = Harness::new_with_config(&AnalyzerConfig::default());
    let pcap_file = &mut File::open(&pcap_path)
        .await
        .expect("failed to open pcap file");
    let mut pcap_reader = PcapNgReader::new(pcap_file)
        .await
        .expect("failed to read PCAP file");

    let mut json_reporter = JsonReporter::new(
        pcap_path.to_str().unwrap(),
        harness_metadata.clone(),
    );

    while let Some(Ok(block)) = pcap_reader.next_block().await {
        let row = match block {
            Block::EnhancedPacket(packet) => harness.analyze_pcap_packet(packet),
            other => {
                debug!("{:?}: skipping pcap packet {other:?}", pcap_path);
                continue;
            }
        };
        json_reporter.process_row(row);
    }

    // Write to .ndjson file instead of stdout
    let mut output_path = pcap_path.clone();
    output_path.set_extension("ndjson");

    match json_reporter.write_to_file(&output_path) {
        Ok(_) => info!("Wrote analysis to {:?}", output_path),
        Err(e) => error!("Failed to write analysis file: {}", e),
    }
}

async fn pcapify(qmdl_path: &PathBuf) {
    let qmdl_file = &mut File::open(&qmdl_path)
        .await
        .expect("failed to open qmdl file");
    let qmdl_file_size = qmdl_file.metadata().await.unwrap().len();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(qmdl_file_size as usize));
    let mut pcap_path = qmdl_path.clone();
    pcap_path.set_extension("pcapng");
    let pcap_file = &mut File::create(&pcap_path)
        .await
        .expect("failed to open pcap file");
    let mut pcap_writer = GsmtapPcapWriter::new(pcap_file).await.unwrap();
    pcap_writer.write_iface_header().await.unwrap();
    while let Some(container) = qmdl_reader
        .get_next_messages_container()
        .await
        .expect("failed to get container")
    {
        for msg in container.into_messages().into_iter().flatten() {
            if let Ok(Some((timestamp, parsed))) = gsmtap_parser::parse(msg) {
                pcap_writer
                    .write_gsmtap_message(parsed, timestamp)
                    .await
                    .expect("failed to write");
            }
        }
    }
    info!("wrote pcap to {:?}", &pcap_path);
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
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .with_level(level)
        //Filter out a stupid massive amount of uneccesary warnings from hampi about undecoded extensions
        .with_module_level("asn1_codecs", log::LevelFilter::Error)
        .init()
        .unwrap();

    let harness = Harness::new_with_config(&AnalyzerConfig::default());
    let harness_metadata = harness.get_metadata();

    info!("Analyzers:");
    for analyzer in &harness_metadata.analyzers {
        info!(
            "    - {} (v{}): {}",
            analyzer.name, analyzer.version, analyzer.description
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
        // instead of relying on the QMDL extension, can we check if a file is
        // QMDL by inspecting the contents?
        if name_str.ends_with(".qmdl") {
            info!("**** Beginning analysis of {name_str}");

            // In JSON format with -p mode, write to individual .ndjson files
            if matches!(args.format, OutputFormat::Json) {
                analyze_qmdl_to_json_file(&path.to_path_buf(), &harness_metadata).await;
            } else {
                analyze_qmdl(path_str, args.show_skipped, args.format, &harness_metadata).await;
            }

            if args.pcapify {
                pcapify(&path.to_path_buf()).await;
            }
        } else if name_str.ends_with(".pcap") || name_str.ends_with(".pcapng") {
            // TODO: if we've already analyzed a QMDL, skip its corresponding pcap
            info!("**** Beginning analysis of {name_str}");

            // In JSON format with -p mode, write to individual .ndjson files
            if matches!(args.format, OutputFormat::Json) {
                analyze_pcap_to_json_file(&path.to_path_buf(), &harness_metadata).await;
            } else {
                analyze_pcap(path_str, args.show_skipped, args.format, &harness_metadata).await;
            }
        }
    }
}
