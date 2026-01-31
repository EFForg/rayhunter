use clap::{Parser, ValueEnum};
use futures::TryStreamExt;
use log::{debug, error, info, warn};
use pcap_file_tokio::pcapng::{Block, PcapNgReader};
use rayhunter::{
    analysis::analyzer::{
        AnalysisRow, AnalyzerConfig, DetectionRow, EventType, Harness, ReportMetadata,
    },
    diag::DataType,
    gsmtap_parser,
    pcap::GsmtapPcapWriter,
    qmdl::QmdlReader,
};
use serde::Serialize;
use std::{collections::HashMap, future, path::PathBuf, pin::pin};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncWriteExt, BufWriter};
use walkdir::WalkDir;

#[derive(ValueEnum, Copy, Clone, Debug, Default)]
enum ReportFormat {
    /// Log detections to stdout
    #[default]
    Log,
    /// Write a newline-delimited JSON file for each report
    Ndjson,
}

impl std::fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportFormat::Log => write!(f, "log"),
            ReportFormat::Ndjson => write!(f, "ndjson"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'p', long, help = "A file or directory of packet captures")]
    path: PathBuf,

    #[arg(short = 'P', long, help = "Convert qmdl files to pcap before analysis")]
    pcapify: bool,

    #[arg(
        short = 'r',
        long,
        help = "Generate a report for each capture analyzed",
        default_value_t = ReportFormat::default()
    )]
    report: ReportFormat,

    #[arg(long, help = "Show why some packets were skipped during analysis")]
    show_skipped: bool,

    #[arg(short, long, help = "Only print warnings/errors to stdout")]
    quiet: bool,

    #[arg(short, long, help = "Show debug messages")]
    debug: bool,
}

#[derive(Default, Debug, Clone, Serialize)]
struct Summary {
    skipped_reasons: HashMap<String, u32>,
    total_messages: u32,
    skipped: u32,
}

#[derive(Default)]
struct LogReport {
    show_skipped: bool,
    warnings: u32,
    file_path: String,
}

impl LogReport {
    fn new(file_path: &str, show_skipped: bool) -> Self {
        LogReport {
            file_path: file_path.to_string(),
            show_skipped,
            ..Default::default()
        }
    }

    fn process_row(&mut self, row: DetectionRow) {
        for event in row.events {
            match event.event_type {
                EventType::Informational => {
                    info!(
                        "{}: INFO - {} {}",
                        self.file_path, row.packet_timestamp, event.message,
                    );
                }
                EventType::Low | EventType::Medium | EventType::High => {
                    warn!(
                        "{}: WARNING (Severity: {:?}) - {} {}",
                        self.file_path, event.event_type, row.packet_timestamp, event.message,
                    );
                    self.warnings += 1;
                }
            }
        }
    }

    fn finish(&self, summary: &Summary) {
        if self.show_skipped && summary.skipped > 0 {
            info!("{}: messages skipped:", self.file_path);
            for (reason, count) in summary.skipped_reasons.iter() {
                info!("    - {count}: \"{reason}\"");
            }
        }
        info!(
            "{}: {} messages analyzed, {} warnings, {} messages skipped",
            self.file_path, summary.total_messages, self.warnings, summary.skipped
        );
    }
}

struct NdjsonReport {
    writer: BufWriter<File>,
}

// The `njson` report has the same output format as the daemon analysis report.
// See also: [Newline Delimited JSON](https://docs.mulesoft.com/dataweave/latest/dataweave-formats-ndjson)
impl NdjsonReport {
    async fn new(file_path: &str, metadata: &ReportMetadata) -> std::io::Result<Self> {
        let mut report_path = PathBuf::from(file_path);
        report_path.set_extension("ndjson");
        let writer = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&report_path)
            .await
            .map(BufWriter::new)?;

        let mut r = NdjsonReport { writer };

        // Analysis metadata is written to the first line of the ndjson report format
        r.write(metadata).await?;

        Ok(r)
    }

    async fn write<T: Serialize>(&mut self, value: &T) -> std::io::Result<()> {
        let mut value_str = serde_json::to_string(value).unwrap();
        value_str.push('\n');
        self.writer.write_all(value_str.as_bytes()).await
    }

    async fn process_row(&mut self, row: DetectionRow) {
        self.write(&row).await.expect("failed to write ndjson row");
    }

    async fn finish(&mut self, _summary: &Summary) {
        self.writer
            .flush()
            .await
            .expect("failed to flush ndjson report");
    }
}

enum ReportDest {
    Log(LogReport),
    Ndjson(NdjsonReport),
}

struct Report {
    show_skipped: bool,
    summary: Summary,
    dest: ReportDest,
}

impl Report {
    async fn build(
        format: ReportFormat,
        harness: &Harness,
        show_skipped: bool,
        path_str: &str,
    ) -> Self {
        let dest = match format {
            ReportFormat::Log => {
                let r = LogReport::new(path_str, show_skipped);
                ReportDest::Log(r)
            }
            ReportFormat::Ndjson => {
                let metadata = harness.get_metadata();
                let ndjson_report = NdjsonReport::new(path_str, &metadata)
                    .await
                    .expect("failed to create ndjson report");
                ReportDest::Ndjson(ndjson_report)
            }
        };

        Report::new_with_dest(show_skipped, dest)
    }
    fn new_with_dest(show_skipped: bool, dest: ReportDest) -> Self {
        Report {
            show_skipped,
            summary: Summary::default(),
            dest,
        }
    }

    async fn process_row(&mut self, row: AnalysisRow) {
        self.summary.total_messages += 1;
        if let Some(ref reason) = row.skipped_message_reason {
            *self
                .summary
                .skipped_reasons
                .entry(reason.clone())
                .or_insert(0) += 1;
            self.summary.skipped += 1;

            if !self.show_skipped {
                return;
            }
        }

        let det = DetectionRow::try_from(row).ok();
        if let Some(detection) = det {
            match &mut self.dest {
                ReportDest::Log(r) => r.process_row(detection),
                ReportDest::Ndjson(r) => r.process_row(detection).await,
            }
        }
    }

    async fn finish(&mut self) {
        match &mut self.dest {
            ReportDest::Log(r) => r.finish(&self.summary),
            ReportDest::Ndjson(r) => r.finish(&self.summary).await,
        }
    }
}

async fn analyze_pcap(pcap_path: &str, args: &Args) {
    let mut harness = Harness::new_with_config(&AnalyzerConfig::default());
    let pcap_file = &mut File::open(&pcap_path).await.expect("failed to open file");
    let mut pcap_reader = PcapNgReader::new(pcap_file)
        .await
        .expect("failed to read PCAP file");

    let mut report = Report::build(args.report, &harness, args.show_skipped, pcap_path).await;

    while let Some(Ok(block)) = pcap_reader.next_block().await {
        let row = match block {
            Block::EnhancedPacket(packet) => harness.analyze_pcap_packet(packet),
            other => {
                debug!("{pcap_path}: skipping pcap packet {other:?}");
                continue;
            }
        };
        report.process_row(row).await;
    }
    report.finish().await;
}

async fn analyze_qmdl(qmdl_path: &str, args: &Args) {
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
    let mut report = Report::build(args.report, &harness, args.show_skipped, qmdl_path).await;
    while let Some(container) = qmdl_stream
        .try_next()
        .await
        .expect("failed getting QMDL container")
    {
        for row in harness.analyze_qmdl_messages(container) {
            report.process_row(row).await;
        }
    }
    report.finish().await;
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
        .env()
        .init()
        .unwrap();

    let harness = Harness::new_with_config(&AnalyzerConfig::default());
    info!("Analyzers:");
    for analyzer in harness.get_metadata().analyzers {
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
            analyze_qmdl(path_str, &args).await;
            if args.pcapify {
                pcapify(&path.to_path_buf()).await;
            }
        } else if name_str.ends_with(".pcap") || name_str.ends_with(".pcapng") {
            // TODO: if we've already analyzed a QMDL, skip its corresponding pcap
            info!("**** Beginning analysis of {name_str}");
            analyze_pcap(path_str, &args).await;
        }
    }
}
