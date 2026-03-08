use clap::Parser;
use futures::TryStreamExt;
use log::{debug, error, info, warn};
use pcap_file_tokio::pcapng::{Block, PcapNgReader};
use rayhunter::{
    analysis::analyzer::{AnalysisRow, AnalyzerConfig, EventType, Harness},
    diag::DataType,
    gsmtap_parser,
    pcap::GsmtapPcapWriter,
    qmdl::QmdlReader,
    serde_json,
};
use std::{
    collections::HashMap,
    future,
    path::{Path, PathBuf},
    pin::pin,
};
use tokio::fs::File;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};
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
        default_value = "text",
        value_name = "FORMAT",
        help = "Output format: 'text' or 'json' (NDJSON; requires --output)"
    )]
    format: String,

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

/// Writes one NDJSON line for a row when it is non-empty. Uses `buf` as scratch to avoid allocating per row.
async fn write_ndjson_row<W: AsyncWrite + Unpin>(
    writer: &mut W,
    row: &AnalysisRow,
    buf: &mut Vec<u8>,
) -> std::io::Result<()> {
    if !row.is_empty() {
        buf.clear();
        serde_json::to_writer(&mut *buf, row)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        buf.push(b'\n');
        AsyncWriteExt::write_all(writer, buf).await?;
    }
    Ok(())
}

/// Creates an NDJSON file in output_dir named from input_path, writes metadata line, returns (writer, path).
async fn open_ndjson_file(
    output_dir: &Path,
    input_path: &str,
    harness: &Harness,
) -> (BufWriter<File>, PathBuf) {
    let out_path = output_dir
        .join(Path::new(input_path).file_stem().unwrap())
        .with_extension("ndjson");
    let f = File::create(&out_path)
        .await
        .expect("failed to create ndjson file");
    let mut writer = BufWriter::new(f);
    let line = serde_json::to_string(&harness.get_metadata()).unwrap() + "\n";
    AsyncWriteExt::write_all(&mut writer, line.as_bytes())
        .await
        .expect("failed to write metadata");
    (writer, out_path)
}

async fn analyze_pcap(
    pcap_path: &str,
    show_skipped: bool,
    format_json: bool,
    output_dir: Option<&Path>,
) {
    let mut harness = Harness::new_with_config(&AnalyzerConfig::default());
    let pcap_file = &mut File::open(pcap_path).await.expect("failed to open file");
    let mut pcap_reader = PcapNgReader::new(pcap_file)
        .await
        .expect("failed to read PCAP file");

    let mut ndjson = if format_json {
        let d = output_dir.expect("--output required for json");
        Some(open_ndjson_file(d, pcap_path, &harness).await)
    } else {
        None
    };
    let mut report = if format_json {
        None
    } else {
        Some(Report::new(pcap_path))
    };
    let mut json_buf = Vec::with_capacity(1024);

    while let Some(Ok(block)) = pcap_reader.next_block().await {
        let row = match block {
            Block::EnhancedPacket(packet) => harness.analyze_pcap_packet(packet),
            other => {
                debug!("{pcap_path}: skipping pcap packet {other:?}");
                continue;
            }
        };
        if let Some((ref mut w, _)) = ndjson {
            write_ndjson_row(w, &row, &mut json_buf)
                .await
                .expect("write");
        } else {
            report.as_mut().unwrap().process_row(&row);
        }
    }

    if let Some((mut f, out_path)) = ndjson {
        AsyncWriteExt::flush(&mut f).await.expect("failed to flush");
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
) {
    let mut harness = Harness::new_with_config(&AnalyzerConfig::default());
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
        (Some(open_ndjson_file(d, qmdl_path, &harness).await), None)
    } else {
        (None, Some(Report::new(qmdl_path)))
    };
    let mut json_buf = Vec::with_capacity(1024);

    while let Some(container) = qmdl_stream
        .try_next()
        .await
        .expect("failed getting QMDL container")
    {
        for row in harness.analyze_qmdl_messages(container) {
            if let Some((ref mut w, _)) = ndjson {
                write_ndjson_row(w, &row, &mut json_buf)
                    .await
                    .expect("write");
            } else {
                report.as_mut().unwrap().process_row(&row);
            }
        }
    }

    if let Some((mut f, out_path)) = ndjson {
        AsyncWriteExt::flush(&mut f).await.expect("failed to flush");
        info!("wrote {:?}", out_path);
    } else {
        report.unwrap().print_summary(show_skipped);
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
    rayhunter::init_logging(level);

    let needs_output_dir = args.format == "json" || args.pcapify;
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
        let format_json = args.format == "json";
        // instead of relying on the QMDL extension, can we check if a file is
        // QMDL by inspecting the contents?
        if name_str.ends_with(".qmdl") {
            info!("**** Beginning analysis of {name_str}");
            analyze_qmdl(path_str, args.show_skipped, format_json, output_dir).await;
            if args.pcapify {
                pcapify(&path.to_path_buf()).await;
            }
        } else if name_str.ends_with(".pcap") || name_str.ends_with(".pcapng") {
            // TODO: if we've already analyzed a QMDL, skip its corresponding pcap
            info!("**** Beginning analysis of {name_str}");
            analyze_pcap(path_str, args.show_skipped, format_json, output_dir).await;
        }
    }
}
