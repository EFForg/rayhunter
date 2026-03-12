use clap::{Parser, ValueEnum};
use futures::TryStreamExt;
use log::{debug, error, info, warn};
use pcap_file_tokio::pcapng::{Block, PcapNgReader};
use rayhunter::{
    analysis::analyzer::{AnalysisRow, AnalyzerConfig, EventType, Harness},
    diag::DataType,
    gsmtap_parser,
    ndjson_writer::NdjsonWriter,
    pcap::GsmtapPcapWriter,
    qmdl::QmdlReader,
};
use std::{
    collections::HashMap,
    future,
    path::{Path, PathBuf},
    pin::pin,
};
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

    #[arg(
        long,
        value_enum,
        default_value_t = OutputFormat::Text,
        value_name = "FORMAT",
        help = "Output format (NDJSON requires --output)"
    )]
    format: OutputFormat,

    #[arg(
        short = 'o',
        long,
        help = "Write output files to this directory (required for --format json and --pcapify)"
    )]
    output: Option<PathBuf>,

    #[arg(short, long, help = "Only print warnings/errors to stdout")]
    quiet: bool,

    #[arg(short, long, help = "Show debug messages")]
    debug: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Default)]
struct Report {
    skipped_reasons: HashMap<String, u32>,
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

    fn process_row(&mut self, row: &AnalysisRow) {
        self.total_messages += 1;
        if let Some(ref reason) = row.skipped_message_reason {
            *self.skipped_reasons.entry(reason.clone()).or_insert(0) += 1;
            self.skipped += 1;
            return;
        }
        for maybe_event in &row.events {
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

fn output_path(output_dir: &Path, input_path: &str, extension: &str) -> PathBuf {
    let stem = Path::new(input_path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    output_dir.join(stem).with_extension(extension)
}

async fn open_ndjson_file(
    output_dir: &Path,
    input_path: &str,
    harness: &Harness,
) -> (NdjsonWriter, PathBuf) {
    let out_path = output_path(output_dir, input_path, "ndjson");
    let f = File::create(&out_path)
        .await
        .expect("failed to create ndjson file");
    let mut writer = NdjsonWriter::new(f);
    writer
        .write(&harness.get_metadata())
        .await
        .expect("failed to write metadata");
    (writer, out_path)
}

async fn analyze_pcap(
    pcap_path: &str,
    show_skipped: bool,
    format_json: bool,
    output_dir: Option<&Path>,
    harness: &mut Harness,
) {
    let pcap_file = &mut File::open(pcap_path).await.expect("failed to open file");
    let mut pcap_reader = PcapNgReader::new(pcap_file)
        .await
        .expect("failed to read PCAP file");

    let (mut ndjson, mut report) = if format_json {
        let d = output_dir.expect("--output required for json");
        (Some(open_ndjson_file(d, pcap_path, harness).await), None)
    } else {
        (None, Some(Report::new(pcap_path)))
    };

    while let Some(Ok(block)) = pcap_reader.next_block().await {
        let row = match block {
            Block::EnhancedPacket(packet) => harness.analyze_pcap_packet(packet),
            other => {
                debug!("{pcap_path}: skipping pcap packet {other:?}");
                continue;
            }
        };
        match &mut ndjson {
            Some((w, _)) => {
                if !row.is_empty() {
                    w.write(&row).await.expect("write");
                }
            }
            None => report.as_mut().unwrap().process_row(&row),
        }
    }

    if let Some((w, out_path)) = ndjson {
        w.close().await.expect("failed to flush");
        info!("wrote {:?}", out_path);
    } else {
        report.unwrap().print_summary(show_skipped);
    }
}

async fn analyze_qmdl(
    qmdl_path: &str,
    show_skipped: bool,
    format_json: bool,
    output_dir: Option<&Path>,
    harness: &mut Harness,
) {
    let qmdl_file = &mut File::open(qmdl_path).await.expect("failed to open file");
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

    let (mut ndjson, mut report) = if format_json {
        let d = output_dir.expect("--output required for json");
        (Some(open_ndjson_file(d, qmdl_path, harness).await), None)
    } else {
        (None, Some(Report::new(qmdl_path)))
    };

    while let Some(container) = qmdl_stream
        .try_next()
        .await
        .expect("failed getting QMDL container")
    {
        for row in harness.analyze_qmdl_messages(container) {
            match &mut ndjson {
                Some((w, _)) => {
                    if !row.is_empty() {
                        w.write(&row).await.expect("write");
                    }
                }
                None => report.as_mut().unwrap().process_row(&row),
            }
        }
    }

    if let Some((w, out_path)) = ndjson {
        w.close().await.expect("failed to flush");
        info!("wrote {:?}", out_path);
    } else {
        report.unwrap().print_summary(show_skipped);
    }
}

async fn pcapify(qmdl_path: &Path, output_dir: &Path) {
    let qmdl_file = &mut File::open(qmdl_path)
        .await
        .expect("failed to open qmdl file");
    let qmdl_file_size = qmdl_file.metadata().await.unwrap().len();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(qmdl_file_size as usize));
    let qmdl_path_str = qmdl_path.to_string_lossy();
    let pcap_path = output_path(output_dir, qmdl_path_str.as_ref(), "pcapng");
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
    // All log output (info, warnings, errors) goes to stderr via env_logger.
    // This keeps stdout clean so NDJSON output can be redirected independently.
    rayhunter::init_logging(level);

    let needs_output_dir = args.format == OutputFormat::Json || args.pcapify;
    if needs_output_dir && args.output.is_none() {
        error!("--output is required for --format json and for --pcapify");
        std::process::exit(1);
    }

    let output_dir = args.output.as_deref();
    if let Some(dir) = output_dir {
        tokio::fs::create_dir_all(dir)
            .await
            .expect("failed to create output directory");
    }

    let mut harness = Harness::new_with_config(&AnalyzerConfig::default());
    info!("Analyzers:");
    for analyzer in harness.get_metadata().analyzers {
        info!(
            "    - {} (v{}): {}",
            analyzer.name, analyzer.version, analyzer.description
        );
    }

    let format_json = args.format == OutputFormat::Json;
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
            analyze_qmdl(
                path_str,
                args.show_skipped,
                format_json,
                output_dir,
                &mut harness,
            )
            .await;
            if args.pcapify {
                let dir = output_dir.expect("--output required for --pcapify");
                pcapify(path, dir).await;
            }
        } else if name_str.ends_with(".pcap") || name_str.ends_with(".pcapng") {
            // TODO: if we've already analyzed a QMDL, skip its corresponding pcap
            info!("**** Beginning analysis of {name_str}");
            analyze_pcap(
                path_str,
                args.show_skipped,
                format_json,
                output_dir,
                &mut harness,
            )
            .await;
        }
    }
}
