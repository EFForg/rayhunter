use clap::{Parser, ValueEnum};
use log::{debug, error, info, warn};
use pcap_file_tokio::pcapng::{Block, PcapNgReader};
use rayhunter::{
    analysis::analyzer::{AnalysisRow, AnalyzerConfig, EventType, Harness, ReportMetadata},
    gsmtap::parser as gsmtap_parser,
    pcap::GsmtapPcapWriter,
    qmdl::QmdlMessageReader,
};
use serde::Serialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'p', long, help = "A file or directory of packet captures")]
    path: PathBuf,

    #[arg(
        short = 'P',
        long,
        help = "Convert each .qmdl to PCAPNG beside the file (or under --output if set)"
    )]
    pcapify: bool,

    #[arg(long, help = "Show why some packets were skipped during analysis")]
    show_skipped: bool,

    #[arg(
        long,
        value_enum,
        default_value_t = OutputFormat::Text,
        value_name = "FORMAT",
        help = "Output format. 'json' writes a JSON array of per-file reports to stdout"
    )]
    format: OutputFormat,

    #[arg(
        short = 'o',
        long,
        help = "Optional directory for output files. With --format json, each input's report is also written to <output>/<input>.json. With --pcapify, PCAPNG files go here instead of next to each .qmdl."
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

// One analyzed input file's machine-readable report. This is the JSON analogue
// of the daemon's on-device NDJSON (metadata header line + one row per event),
// reshaped into a single well-formed JSON object so standard tooling (jq,
// JSON.parse) can consume it directly.
#[derive(Serialize)]
struct FileReport {
    path: String,
    metadata: ReportMetadata,
    rows: Vec<AnalysisRow>,
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

// Decide whether an AnalysisRow should appear in JSON output. Empty rows (no
// warnings, no skipped reason) are always omitted; rows that exist only because
// a message was skipped are omitted unless --show-skipped is set, so JSON
// consumers see warnings by default and can opt in to the verbose stream the
// same way the text-mode summary does.
fn should_emit(row: &AnalysisRow, show_skipped: bool) -> bool {
    if row.is_empty() {
        return false;
    }
    show_skipped || row.skipped_message_reason.is_none()
}

// Per-file output collector. Owning the mode-specific state here lets the
// analyze_* functions stay format-agnostic: they read rows and hand each to
// `accept`, then `finish` either prints the text summary or returns the JSON
// report. Only the active mode's state is allocated.
enum Sink {
    Text(Report),
    Json(Vec<AnalysisRow>),
}

impl Sink {
    fn new(format: OutputFormat, file_path: &str) -> Self {
        match format {
            OutputFormat::Text => Sink::Text(Report::new(file_path)),
            OutputFormat::Json => Sink::Json(Vec::new()),
        }
    }

    fn accept(&mut self, row: AnalysisRow, show_skipped: bool) {
        match self {
            Sink::Text(report) => report.process_row(row),
            Sink::Json(rows) => {
                if should_emit(&row, show_skipped) {
                    rows.push(row);
                }
            }
        }
    }

    fn finish(self, file_path: &str, harness: &Harness, show_skipped: bool) -> Option<FileReport> {
        match self {
            Sink::Text(report) => {
                report.print_summary(show_skipped);
                None
            }
            Sink::Json(rows) => Some(FileReport {
                path: file_path.to_string(),
                metadata: harness.get_metadata(),
                rows,
            }),
        }
    }
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

/// Directory to write `--pcapify` output when `--output` is omitted: the QMDL's
/// parent folder, or `.` when the path has no directory component (e.g.
/// `foo.qmdl` yields parent `""` on Unix).
fn sidecar_output_dir(qmdl_path: &Path) -> &Path {
    match qmdl_path.parent() {
        Some(p) if !p.as_os_str().is_empty() => p,
        _ => Path::new("."),
    }
}

async fn analyze_pcap(
    pcap_path: &str,
    show_skipped: bool,
    format: OutputFormat,
    config: &AnalyzerConfig,
) -> Option<FileReport> {
    let mut harness = Harness::new_with_config(config);
    let pcap_file = &mut File::open(pcap_path).await.expect("failed to open file");
    let mut pcap_reader = PcapNgReader::new(pcap_file)
        .await
        .expect("failed to read PCAP file");
    let mut sink = Sink::new(format, pcap_path);
    while let Some(Ok(block)) = pcap_reader.next_block().await {
        let row = match block {
            Block::EnhancedPacket(packet) => harness.analyze_pcap_packet(packet),
            other => {
                debug!("{pcap_path}: skipping pcap packet {other:?}");
                continue;
            }
        };
        sink.accept(row, show_skipped);
    }
    sink.finish(pcap_path, &harness, show_skipped)
}

async fn analyze_qmdl(
    qmdl_path: &str,
    show_skipped: bool,
    format: OutputFormat,
    config: &AnalyzerConfig,
) -> Option<FileReport> {
    let mut harness = Harness::new_with_config(config);
    let qmdl_file = &mut File::open(qmdl_path).await.expect("failed to open file");
    let mut qmdl_reader = QmdlMessageReader::new(qmdl_file)
        .await
        .expect("failed to open QmdlReader");
    let mut sink = Sink::new(format, qmdl_path);
    while let Some(maybe_message) = qmdl_reader
        .get_next_message()
        .await
        .expect("failed to get message")
    {
        sink.accept(harness.analyze_qmdl_message(maybe_message), show_skipped);
    }
    sink.finish(qmdl_path, &harness, show_skipped)
}

async fn pcapify(qmdl_path: &Path, output_dir: &Path) {
    let qmdl_path_str = qmdl_path.to_string_lossy();
    let pcap_path = output_path(output_dir, qmdl_path_str.as_ref(), "pcapng");
    if tokio::fs::try_exists(&pcap_path).await.unwrap_or(false) {
        error!(
            "{}: refusing to overwrite existing {}; skipping pcapify (different inputs with the same file name collide in the output directory)",
            qmdl_path_str,
            pcap_path.display()
        );
        return;
    }
    let qmdl_file = &mut File::open(qmdl_path)
        .await
        .expect("failed to open qmdl file");
    let mut qmdl_reader = QmdlMessageReader::new(qmdl_file)
        .await
        .expect("failed to open QmdlReader");
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
    info!("wrote pcap to {:?}", &pcap_path);
}

// Emit the collected JSON reports. The whole run is serialized to stdout as a
// single JSON array (always an array, even for one input, so consumers get a
// stable shape regardless of how many files matched). When --output is set,
// each report is additionally written to its own <output>/<input>.json file.
// Cross-directory collisions on the per-file copy are reported and skipped, but
// stdout still receives every report.
async fn write_json_reports(reports: &[FileReport], output_dir: Option<&Path>) {
    if let Some(dir) = output_dir {
        for report in reports {
            let out_path = output_path(dir, &report.path, "json");
            if tokio::fs::try_exists(&out_path).await.unwrap_or(false) {
                error!(
                    "{}: refusing to overwrite existing {}; skipping file copy (different inputs with the same file name collide in --output). JSON will still be written to stdout.",
                    report.path,
                    out_path.display()
                );
                continue;
            }
            let json = serde_json::to_string_pretty(report).expect("failed to serialize report");
            tokio::fs::write(&out_path, json)
                .await
                .expect("failed to write json file");
            info!("wrote {:?}", out_path);
        }
    }

    let json = serde_json::to_string_pretty(reports).expect("failed to serialize reports");
    let mut stdout = tokio::io::stdout();
    stdout
        .write_all(json.as_bytes())
        .await
        .expect("failed to write stdout");
    stdout
        .write_all(b"\n")
        .await
        .expect("failed to write stdout");
    stdout.flush().await.expect("failed to flush stdout");
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
    // This keeps stdout clean so JSON output can be redirected independently.
    rayhunter::init_logging(level);

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

    let mut reports: Vec<FileReport> = Vec::new();
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
            if let Some(report) =
                analyze_qmdl(path_str, args.show_skipped, args.format, &analyzer_config).await
            {
                reports.push(report);
            }
            if args.pcapify {
                let pcap_dir = output_dir.unwrap_or_else(|| sidecar_output_dir(path));
                pcapify(path, pcap_dir).await;
            }
        } else if name_str.ends_with(".pcap") || name_str.ends_with(".pcapng") {
            // TODO: if we've already analyzed a QMDL, skip its corresponding pcap
            info!("**** Beginning analysis of {name_str}");
            if let Some(report) =
                analyze_pcap(path_str, args.show_skipped, args.format, &analyzer_config).await
            {
                reports.push(report);
            }
        }
    }

    if args.format == OutputFormat::Json {
        write_json_reports(&reports, output_dir).await;
    }
}

#[cfg(test)]
mod tests {
    use super::{output_path, should_emit, sidecar_output_dir};
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
        assert!(!should_emit(&empty_row(), false));
        assert!(!should_emit(&empty_row(), true));
    }

    #[test]
    fn informational_only_row_never_emitted() {
        // Informational events don't count as warnings; AnalysisRow::is_empty
        // treats this row as empty, so it stays out of the JSON output.
        assert!(!should_emit(&informational_only_row(), false));
        assert!(!should_emit(&informational_only_row(), true));
    }

    #[test]
    fn warning_row_always_emitted() {
        assert!(should_emit(&warning_row(), false));
        assert!(should_emit(&warning_row(), true));
    }

    #[test]
    fn skipped_row_only_emitted_with_show_skipped() {
        assert!(!should_emit(&skipped_row(), false));
        assert!(should_emit(&skipped_row(), true));
    }

    #[test]
    fn appends_extension_to_basic_name() {
        let got = output_path(Path::new("/out"), "/in/capture.qmdl", "json");
        assert_eq!(got, PathBuf::from("/out/capture.qmdl.json"));
    }

    #[test]
    fn preserves_dotted_stem() {
        let got = output_path(Path::new("/out"), "/in/recording.v2.qmdl", "json");
        assert_eq!(got, PathBuf::from("/out/recording.v2.qmdl.json"));
    }

    #[test]
    fn preserves_timestamped_name_with_multiple_dots() {
        let got = output_path(
            Path::new("/out"),
            "/in/2026-01-02_10.05.00_capture.qmdl",
            "json",
        );
        assert_eq!(
            got,
            PathBuf::from("/out/2026-01-02_10.05.00_capture.qmdl.json")
        );
    }

    #[test]
    fn same_stem_with_different_extensions_stays_distinct() {
        let qmdl = output_path(Path::new("/out"), "/in/session.qmdl", "json");
        let pcap = output_path(Path::new("/out"), "/in/session.pcap", "json");
        assert_eq!(qmdl, PathBuf::from("/out/session.qmdl.json"));
        assert_eq!(pcap, PathBuf::from("/out/session.pcap.json"));
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
        let got = output_path(Path::new("/out"), "/", "json");
        assert_eq!(got, PathBuf::from("/out/output.json"));
    }

    #[test]
    fn ignores_input_directory_components() {
        // Outputs are flat under output_dir; only the input file name matters.
        let a = output_path(Path::new("/out"), "/captures/a/capture.qmdl", "json");
        let b = output_path(Path::new("/out"), "/captures/b/capture.qmdl", "json");
        // Both resolve to the same path; collision detection in the caller is
        // what prevents the second from clobbering the first.
        assert_eq!(a, PathBuf::from("/out/capture.qmdl.json"));
        assert_eq!(a, b);
    }

    #[test]
    fn sidecar_dir_is_parent_of_input() {
        assert_eq!(
            sidecar_output_dir(Path::new("captures/sub/file.qmdl")),
            Path::new("captures/sub")
        );
    }

    #[test]
    fn sidecar_dir_single_component_relative_is_dot() {
        assert_eq!(sidecar_output_dir(Path::new("file.qmdl")), Path::new("."));
    }

    #[cfg(unix)]
    #[test]
    fn sidecar_dir_absolute_unix_style() {
        assert_eq!(
            sidecar_output_dir(Path::new("/var/recordings/session.qmdl")),
            Path::new("/var/recordings")
        );
    }

    #[cfg(unix)]
    #[test]
    fn sidecar_dir_file_at_filesystem_root() {
        assert_eq!(sidecar_output_dir(Path::new("/solo.qmdl")), Path::new("/"));
    }

    #[cfg(windows)]
    #[test]
    fn sidecar_dir_absolute_windows_style() {
        assert_eq!(
            sidecar_output_dir(Path::new(r"C:\captures\session.qmdl")),
            Path::new(r"C:\captures")
        );
    }
}
