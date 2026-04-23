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
        help = "Output format (NDJSON is written to stdout)"
    )]
    format: OutputFormat,

    #[arg(
        short = 'o',
        long,
        help = "Optional directory for output files. With --format json, NDJSON is also written to <output>/<input>.ndjson alongside stdout. Required for --pcapify."
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

// Decide whether an AnalysisRow should appear in NDJSON output. Empty rows
// (no warnings, no skipped reason) are always omitted; rows that exist only
// because a message was skipped are omitted unless --show-skipped is set, so
// NDJSON consumers see warnings by default and can opt in to the verbose
// stream the same way the text-mode summary does.
fn should_emit_to_ndjson(row: &AnalysisRow, show_skipped: bool) -> bool {
    if row.is_empty() {
        return false;
    }
    show_skipped || row.skipped_message_reason.is_none()
}

fn output_path(output_dir: &Path, input_path: &str, extension: &str) -> PathBuf {
    // Append the new extension to the input's full file name (rather than
    // replacing it via Path::with_extension, which eats anything after the
    // last dot in the stem). This preserves dotted stems like
    // "2026-01-02_10.05.00_capture.qmdl" and keeps "session.qmdl" /
    // "session.pcap" outputs distinct in the same target directory.
    let input_name = Path::new(input_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    output_dir.join(format!("{input_name}.{extension}"))
}

// NDJSON sinks for one input file. Stdout is always set up (per the design
// in PR #941: ndjson always goes to stdout, --output is optional). When
// --output is provided, an extra file sink is created so the run produces a
// durable copy alongside the stream. Cross-directory collisions on the file
// sink are reported and the file is skipped, but stdout output continues so
// the user still sees the analysis.
struct NdjsonSinks {
    stdout: NdjsonWriter,
    file: Option<NdjsonFileSink>,
}

struct NdjsonFileSink {
    writer: NdjsonWriter,
    path: PathBuf,
}

impl NdjsonSinks {
    async fn for_input(
        output_dir: Option<&Path>,
        input_path: &str,
        harness: &Harness,
    ) -> NdjsonSinks {
        let stdout = NdjsonWriter::with_writer(tokio::io::stdout());
        let mut sinks = NdjsonSinks { stdout, file: None };

        if let Some(dir) = output_dir {
            let out_path = output_path(dir, input_path, "ndjson");
            if tokio::fs::try_exists(&out_path).await.unwrap_or(false) {
                error!(
                    "{input_path}: refusing to overwrite existing {}; skipping file copy (different inputs with the same file name collide in --output). NDJSON will still be written to stdout.",
                    out_path.display()
                );
            } else {
                let f = File::create(&out_path)
                    .await
                    .expect("failed to create ndjson file");
                let writer = NdjsonWriter::new(f);
                sinks.file = Some(NdjsonFileSink {
                    writer,
                    path: out_path,
                });
            }
        }

        sinks
            .write(&harness.get_metadata())
            .await
            .expect("failed to write metadata");
        sinks
    }

    async fn write<T: serde::Serialize>(&mut self, value: &T) -> Result<(), std::io::Error> {
        self.stdout.write(value).await?;
        if let Some(ref mut f) = self.file {
            f.writer.write(value).await?;
        }
        Ok(())
    }

    async fn close(self) -> Result<(), std::io::Error> {
        self.stdout.close().await?;
        if let Some(f) = self.file {
            f.writer.close().await?;
            info!("wrote {:?}", f.path);
        }
        Ok(())
    }
}

async fn analyze_pcap(
    pcap_path: &str,
    show_skipped: bool,
    format_json: bool,
    output_dir: Option<&Path>,
    config: &AnalyzerConfig,
) {
    let mut harness = Harness::new_with_config(config);
    let pcap_file = &mut File::open(pcap_path).await.expect("failed to open file");
    let mut pcap_reader = PcapNgReader::new(pcap_file)
        .await
        .expect("failed to read PCAP file");

    let (mut ndjson, mut report) = if format_json {
        (
            Some(NdjsonSinks::for_input(output_dir, pcap_path, &harness).await),
            None,
        )
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
            Some(sinks) => {
                if should_emit_to_ndjson(&row, show_skipped) {
                    sinks.write(&row).await.expect("write");
                }
            }
            None => report.as_mut().unwrap().process_row(&row),
        }
    }

    if let Some(sinks) = ndjson {
        sinks.close().await.expect("failed to flush");
    } else {
        report.unwrap().print_summary(show_skipped);
    }
}

async fn analyze_qmdl(
    qmdl_path: &str,
    show_skipped: bool,
    format_json: bool,
    output_dir: Option<&Path>,
    config: &AnalyzerConfig,
) {
    let mut harness = Harness::new_with_config(config);
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
        (
            Some(NdjsonSinks::for_input(output_dir, qmdl_path, &harness).await),
            None,
        )
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
                Some(sinks) => {
                    if should_emit_to_ndjson(&row, show_skipped) {
                        sinks.write(&row).await.expect("write");
                    }
                }
                None => report.as_mut().unwrap().process_row(&row),
            }
        }
    }

    if let Some(sinks) = ndjson {
        sinks.close().await.expect("failed to flush");
    } else {
        report.unwrap().print_summary(show_skipped);
    }
}

async fn pcapify(qmdl_path: &Path, output_dir: &Path) {
    let qmdl_path_str = qmdl_path.to_string_lossy();
    let pcap_path = output_path(output_dir, qmdl_path_str.as_ref(), "pcapng");
    if tokio::fs::try_exists(&pcap_path).await.unwrap_or(false) {
        error!(
            "{}: refusing to overwrite existing {}; skipping pcapify (different inputs with the same file name collide in --output)",
            qmdl_path_str,
            pcap_path.display()
        );
        return;
    }
    let qmdl_file = &mut File::open(qmdl_path)
        .await
        .expect("failed to open qmdl file");
    let qmdl_file_size = qmdl_file.metadata().await.unwrap().len();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(qmdl_file_size as usize));
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

    if args.pcapify && args.output.is_none() {
        error!("--output is required for --pcapify");
        std::process::exit(1);
    }

    let output_dir = args.output.as_deref();
    if let Some(dir) = output_dir {
        tokio::fs::create_dir_all(dir)
            .await
            .expect("failed to create output directory");
    }

    let analyzer_config = AnalyzerConfig::default();
    let metadata_harness = Harness::new_with_config(&analyzer_config);
    info!("Analyzers:");
    for analyzer in metadata_harness.get_metadata().analyzers {
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
                &analyzer_config,
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
                &analyzer_config,
            )
            .await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{output_path, should_emit_to_ndjson};
    use rayhunter::analysis::analyzer::{AnalysisRow, Event, EventType};
    use std::path::{Path, PathBuf};

    fn empty_row() -> AnalysisRow {
        AnalysisRow {
            packet_timestamp: None,
            skipped_message_reason: None,
            events: Vec::new(),
        }
    }

    fn skipped_row() -> AnalysisRow {
        AnalysisRow {
            packet_timestamp: None,
            skipped_message_reason: Some("test reason".into()),
            events: Vec::new(),
        }
    }

    fn warning_row() -> AnalysisRow {
        AnalysisRow {
            packet_timestamp: None,
            skipped_message_reason: None,
            events: vec![Some(Event {
                event_type: EventType::High,
                message: "test warning".into(),
            })],
        }
    }

    fn informational_only_row() -> AnalysisRow {
        AnalysisRow {
            packet_timestamp: None,
            skipped_message_reason: None,
            events: vec![Some(Event {
                event_type: EventType::Informational,
                message: "fyi".into(),
            })],
        }
    }

    #[test]
    fn empty_row_never_emitted() {
        assert!(!should_emit_to_ndjson(&empty_row(), false));
        assert!(!should_emit_to_ndjson(&empty_row(), true));
    }

    #[test]
    fn informational_only_row_never_emitted() {
        // Informational events don't count as warnings; AnalysisRow::is_empty
        // treats this row as empty, so it stays out of NDJSON.
        assert!(!should_emit_to_ndjson(&informational_only_row(), false));
        assert!(!should_emit_to_ndjson(&informational_only_row(), true));
    }

    #[test]
    fn warning_row_always_emitted() {
        assert!(should_emit_to_ndjson(&warning_row(), false));
        assert!(should_emit_to_ndjson(&warning_row(), true));
    }

    #[test]
    fn skipped_row_only_emitted_with_show_skipped() {
        assert!(!should_emit_to_ndjson(&skipped_row(), false));
        assert!(should_emit_to_ndjson(&skipped_row(), true));
    }

    #[test]
    fn appends_extension_to_basic_name() {
        let got = output_path(Path::new("/out"), "/in/capture.qmdl", "ndjson");
        assert_eq!(got, PathBuf::from("/out/capture.qmdl.ndjson"));
    }

    #[test]
    fn preserves_dotted_stem() {
        let got = output_path(Path::new("/out"), "/in/recording.v2.qmdl", "ndjson");
        assert_eq!(got, PathBuf::from("/out/recording.v2.qmdl.ndjson"));
    }

    #[test]
    fn preserves_timestamped_name_with_multiple_dots() {
        let got = output_path(
            Path::new("/out"),
            "/in/2026-01-02_10.05.00_capture.qmdl",
            "ndjson",
        );
        assert_eq!(
            got,
            PathBuf::from("/out/2026-01-02_10.05.00_capture.qmdl.ndjson")
        );
    }

    #[test]
    fn same_stem_with_different_extensions_stays_distinct() {
        let qmdl = output_path(Path::new("/out"), "/in/session.qmdl", "ndjson");
        let pcap = output_path(Path::new("/out"), "/in/session.pcap", "ndjson");
        assert_eq!(qmdl, PathBuf::from("/out/session.qmdl.ndjson"));
        assert_eq!(pcap, PathBuf::from("/out/session.pcap.ndjson"));
        assert_ne!(qmdl, pcap);
    }

    #[test]
    fn pcapng_extension_uses_same_template() {
        let got = output_path(Path::new("/out"), "/in/capture.qmdl", "pcapng");
        assert_eq!(got, PathBuf::from("/out/capture.qmdl.pcapng"));
    }

    #[test]
    fn input_without_file_name_falls_back_to_output_literal() {
        // Path::file_name() returns None for "/" and similar; we should still
        // produce a deterministic, non-panicking output path.
        let got = output_path(Path::new("/out"), "/", "ndjson");
        assert_eq!(got, PathBuf::from("/out/output.ndjson"));
    }

    #[test]
    fn ignores_input_directory_components() {
        // Outputs are flat under output_dir; only the input file name matters.
        let a = output_path(Path::new("/out"), "/captures/a/capture.qmdl", "ndjson");
        let b = output_path(Path::new("/out"), "/captures/b/capture.qmdl", "ndjson");
        // Both resolve to the same path; collision detection in the caller is
        // what prevents the second from clobbering the first.
        assert_eq!(a, PathBuf::from("/out/capture.qmdl.ndjson"));
        assert_eq!(a, b);
    }
}
