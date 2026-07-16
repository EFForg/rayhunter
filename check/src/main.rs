use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use log::{debug, error, info, warn};
use pcap_file_tokio::DataLink;
use pcap_file_tokio::pcap::PcapReader;
use pcap_file_tokio::pcapng::blocks::enhanced_packet::EnhancedPacketBlock;
use pcap_file_tokio::pcapng::{Block, PcapNgReader};
use rayhunter::{
    analysis::analyzer::{AnalysisRow, AnalyzerConfig, EventType, Harness, ReportMetadata},
    gsmtap::parser as gsmtap_parser,
    pcap::GsmtapPcapWriter,
    qmdl::QmdlMessageReader,
};
use serde::Serialize;
use std::{
    borrow::Cow,
    collections::HashMap,
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
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

    #[arg(short, long, help = "Only log warnings and errors")]
    quiet: bool,

    #[arg(short, long, help = "Show debug messages")]
    debug: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

// one input file's analysis as a single json object. same metadata + rows as
// the daemon's on-device ndjson, just wrapped up so jq and friends can read it.
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

// whether a row belongs in the json output. empty rows are always dropped, and
// skipped-message rows only show up with --show-skipped, same as text mode.
fn should_emit(row: &AnalysisRow, show_skipped: bool) -> bool {
    if row.is_empty() {
        return false;
    }
    show_skipped || row.skipped_message_reason.is_none()
}

// holds a file's rows in whatever form the current output mode needs, so the
// analyze_* fns don't have to know which mode they're feeding.
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
    // append the extension instead of using with_extension, which would eat
    // everything after the last dot. keeps dotted names like
    // 2026-01-02_10.05.00_capture.qmdl whole, and session.qmdl / session.pcap
    // from landing on the same output name.
    let input_name = Path::new(input_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    output_dir.join(format!("{input_name}.{extension}"))
}

// where --pcapify writes when there's no --output: next to the qmdl, falling
// back to the current dir when the path has no parent (e.g. a bare foo.qmdl).
fn sidecar_output_dir(qmdl_path: &Path) -> &Path {
    match qmdl_path.parent() {
        Some(p) if !p.as_os_str().is_empty() => p,
        _ => Path::new("."),
    }
}

// pcapng starts with a Section Header Block carrying this block type
const PCAPNG_MAGIC: [u8; 4] = [0x0A, 0x0D, 0x0D, 0x0A];
// legacy pcap magics: micro/nanosecond resolution, either byte order
const PCAP_MAGICS: [[u8; 4]; 4] = [
    [0xA1, 0xB2, 0xC3, 0xD4],
    [0xD4, 0xC3, 0xB2, 0xA1],
    [0xA1, 0xB2, 0x3C, 0x4D],
    [0x4D, 0x3C, 0xB2, 0xA1],
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum CaptureFormat {
    PcapNg,
    Pcap,
}

fn detect_capture_format(magic: [u8; 4]) -> Option<CaptureFormat> {
    if magic == PCAPNG_MAGIC {
        Some(CaptureFormat::PcapNg)
    } else if PCAP_MAGICS.contains(&magic) {
        Some(CaptureFormat::Pcap)
    } else {
        None
    }
}

// 1970-01-01 to 1980-01-06. legacy pcap timestamps are unix-epoch, but
// analyze_pcap_packet reads them as gps-epoch, so shift by this (~10 years).
const UNIX_TO_GPS_EPOCH_OFFSET: Duration = Duration::from_secs(315_964_800);

// how many bytes of link-layer header to skip so the data starts at the IP
// header. None for link types we don't handle.
fn link_layer_offset(datalink: DataLink) -> Option<usize> {
    match datalink {
        DataLink::RAW | DataLink::IPV4 => Some(0),
        DataLink::ETHERNET => Some(14),
        DataLink::NULL | DataLink::LOOP => Some(4),
        DataLink::LINUX_SLL => Some(16),
        DataLink::LINUX_SLL2 => Some(20),
        _ => None,
    }
}

async fn analyze_pcap(
    pcap_path: &str,
    show_skipped: bool,
    format: OutputFormat,
    config: &AnalyzerConfig,
) -> Result<Option<FileReport>> {
    let mut harness = Harness::new_with_config(config);
    let mut pcap_file = File::open(pcap_path).await.context("failed to open file")?;

    // accept both legacy pcap (tcpdump's default) and pcapng; peek at the
    // magic number to pick the right reader.
    let mut magic = [0u8; 4];
    let capture_format = match pcap_file.read_exact(&mut magic).await {
        Ok(_) => detect_capture_format(magic),
        Err(err) if err.kind() == std::io::ErrorKind::UnexpectedEof => None,
        Err(err) => return Err(err).context("failed to read file"),
    };
    let Some(capture_format) = capture_format else {
        bail!("unrecognized capture format (expected pcap or pcapng magic)");
    };
    pcap_file.rewind().await.context("failed to rewind file")?;

    let mut sink = Sink::new(format, pcap_path);
    match capture_format {
        CaptureFormat::PcapNg => {
            let mut pcap_reader = PcapNgReader::new(pcap_file)
                .await
                .context("failed to read pcapng file")?;
            loop {
                match pcap_reader.next_block().await {
                    Some(Ok(Block::EnhancedPacket(packet))) => {
                        sink.accept(harness.analyze_pcap_packet(packet), show_skipped);
                    }
                    Some(Ok(other)) => debug!("{pcap_path}: skipping pcap packet {other:?}"),
                    Some(Err(err)) => {
                        error!(
                            "{pcap_path}: stopping analysis early, failed to read next block: {err}"
                        );
                        break;
                    }
                    None => break,
                }
            }
        }
        CaptureFormat::Pcap => {
            let mut pcap_reader = PcapReader::new(pcap_file)
                .await
                .context("failed to read pcap file")?;
            let datalink = pcap_reader.header().datalink;
            let Some(link_offset) = link_layer_offset(datalink) else {
                bail!("unsupported pcap link type {datalink:?}");
            };
            loop {
                match pcap_reader.next_packet().await {
                    Some(Ok(packet)) => {
                        // analyze_pcap_packet only wants the timestamp and
                        // data, so fake up a pcapng block. it expects the IP
                        // header first, so drop the link-layer framing (a
                        // too-short packet just becomes empty and gets skipped).
                        let payload_start = link_offset.min(packet.data.len());
                        // it also reads the timestamp as gps-epoch (like our
                        // pcapng writer), but legacy pcap is unix-epoch, so
                        // shift it back or the times come out ~10 years off.
                        let timestamp = packet
                            .timestamp
                            .checked_sub(UNIX_TO_GPS_EPOCH_OFFSET)
                            .unwrap_or(Duration::ZERO);
                        let block = EnhancedPacketBlock {
                            interface_id: 0,
                            timestamp,
                            original_len: packet.orig_len,
                            // borrow the reader's buffer; analyze_pcap_packet
                            // copies out only the bytes it keeps, so there's no
                            // point cloning the whole payload here.
                            data: Cow::Borrowed(&packet.data[payload_start..]),
                            options: Vec::new(),
                        };
                        sink.accept(harness.analyze_pcap_packet(block), show_skipped);
                    }
                    Some(Err(err)) => {
                        error!(
                            "{pcap_path}: stopping analysis early, failed to read next packet: {err}"
                        );
                        break;
                    }
                    None => break,
                }
            }
        }
    }
    Ok(sink.finish(pcap_path, &harness, show_skipped))
}

async fn analyze_qmdl(
    qmdl_path: &str,
    show_skipped: bool,
    format: OutputFormat,
    config: &AnalyzerConfig,
) -> Result<Option<FileReport>> {
    let mut harness = Harness::new_with_config(config);
    let qmdl_file = &mut File::open(qmdl_path).await.context("failed to open file")?;
    let mut qmdl_reader = QmdlMessageReader::new(qmdl_file)
        .await
        .context("failed to open QmdlReader")?;
    let mut sink = Sink::new(format, qmdl_path);
    loop {
        match qmdl_reader.get_next_message().await {
            Ok(Some(maybe_message)) => {
                sink.accept(harness.analyze_qmdl_message(maybe_message), show_skipped);
            }
            Ok(None) => break,
            // a read error partway through (truncated/corrupt file) keeps what
            // we already analyzed instead of tossing the whole file.
            Err(err) => {
                error!("{qmdl_path}: stopping analysis early, failed to read next message: {err}");
                break;
            }
        }
    }
    Ok(sink.finish(qmdl_path, &harness, show_skipped))
}

async fn pcapify(qmdl_path: &Path, output_dir: &Path) -> Result<()> {
    let qmdl_path_str = qmdl_path.to_string_lossy();
    let pcap_path = output_path(output_dir, qmdl_path_str.as_ref(), "pcapng");
    if tokio::fs::try_exists(&pcap_path).await.unwrap_or(false) {
        error!(
            "{}: refusing to overwrite existing {}; skipping pcapify (different inputs with the same file name collide in the output directory)",
            qmdl_path_str,
            pcap_path.display()
        );
        return Ok(());
    }
    let qmdl_file = &mut File::open(qmdl_path)
        .await
        .context("failed to open qmdl file")?;
    let mut qmdl_reader = QmdlMessageReader::new(qmdl_file)
        .await
        .context("failed to open QmdlReader")?;
    let pcap_file = &mut File::create(&pcap_path)
        .await
        .context("failed to create pcap file")?;
    // false if the read stopped early, so we log it as a partial pcap
    let mut completed = true;
    let write_result = async {
        let mut pcap_writer = GsmtapPcapWriter::new(pcap_file).await?;
        pcap_writer.write_iface_header().await?;
        loop {
            match qmdl_reader.get_next_message().await {
                Ok(Some(maybe_message)) => {
                    if let Ok(msg) = maybe_message
                        && let Ok(Some((timestamp, parsed))) = gsmtap_parser::parse(msg)
                    {
                        pcap_writer
                            .write_gsmtap_message(parsed, timestamp, None)
                            .await
                            .context("failed to write gsmtap message")?;
                    }
                }
                Ok(None) => break,
                // keep the packets we already converted if the read stops early
                Err(err) => {
                    error!(
                        "{qmdl_path_str}: stopping pcapify early, failed to read next message: {err}"
                    );
                    completed = false;
                    break;
                }
            }
        }
        anyhow::Ok(())
    }
    .await;
    // a failed write leaves a corrupt file that the overwrite guard above would
    // then refuse to replace, so delete it and let a re-run rebuild it. (a read
    // error is different: we keep that partial pcap.)
    if let Err(err) = write_result {
        let _ = tokio::fs::remove_file(&pcap_path).await;
        return Err(err);
    }
    if completed {
        info!("wrote pcap to {:?}", pcap_path);
    } else {
        info!("wrote partial pcap to {:?}", pcap_path);
    }
    Ok(())
}

// write every report to stdout as one json array (always an array, even for a
// single file, so the shape is predictable). with --output, also drop a copy
// per file; if two inputs map to the same name we skip the copy but still print
// it to stdout.
async fn write_json_reports(reports: &[FileReport], output_dir: Option<&Path>) -> Result<()> {
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
            // serializing our own struct can't fail; the write can
            let json = serde_json::to_string_pretty(report).expect("failed to serialize report");
            tokio::fs::write(&out_path, json)
                .await
                .with_context(|| format!("failed to write {}", out_path.display()))?;
            info!("wrote {:?}", out_path);
        }
    }

    let json = serde_json::to_string_pretty(reports).expect("failed to serialize reports");
    let mut stdout = tokio::io::stdout();
    stdout
        .write_all(json.as_bytes())
        .await
        .context("failed to write stdout")?;
    stdout
        .write_all(b"\n")
        .await
        .context("failed to write stdout")?;
    stdout.flush().await.context("failed to flush stdout")?;
    Ok(())
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
    // logs go to stderr (env_logger), so stdout stays clean for the json
    rayhunter::init_logging(level);

    if let Err(err) = run(args).await {
        error!("{err:#}");
        std::process::exit(1);
    }
}

async fn run(args: Args) -> Result<()> {
    let output_dir = args.output.as_deref();
    if let Some(dir) = output_dir {
        tokio::fs::create_dir_all(dir)
            .await
            .context("failed to create output directory")?;
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
    let mut failed_files = 0u32;
    for maybe_entry in WalkDir::new(&args.path) {
        let Ok(entry) = maybe_entry else {
            error!("failed to open dir entry {maybe_entry:?}");
            failed_files += 1;
            continue;
        };
        let name = entry.file_name();
        let name_str = name.to_str().unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();
        let is_qmdl = name_str.ends_with(".qmdl") || name_str.ends_with(".qmdl.gz");
        let is_pcap = name_str.ends_with(".pcap") || name_str.ends_with(".pcapng");
        if !is_qmdl && !is_pcap {
            continue;
        }
        // skip anything that isn't a regular file. a dir named foo.qmdl is
        // fine to skip quietly, but warn on a broken symlink/fifo so a matching
        // name doesn't just vanish from the output.
        if !path.is_file() {
            if !entry.file_type().is_dir() {
                warn!("{path_str}: skipping (not a regular file)");
            }
            continue;
        }
        // TODO: if we've already analyzed a QMDL, skip its corresponding pcap
        info!("**** Beginning analysis of {name_str}");
        let outcome = if is_qmdl {
            analyze_qmdl(path_str, args.show_skipped, args.format, &analyzer_config).await
        } else {
            analyze_pcap(path_str, args.show_skipped, args.format, &analyzer_config).await
        };
        match outcome {
            Ok(Some(report)) => reports.push(report),
            Ok(None) => {}
            // one bad file shouldn't sink the rest of the run
            Err(err) => {
                error!("{path_str}: skipping file, analysis failed: {err:#}");
                failed_files += 1;
            }
        }
        if is_qmdl && args.pcapify {
            let pcap_dir = output_dir.unwrap_or_else(|| sidecar_output_dir(path));
            if let Err(err) = pcapify(path, pcap_dir).await {
                error!("{path_str}: pcapify failed: {err:#}");
                failed_files += 1;
            }
        }
    }

    if args.format == OutputFormat::Json {
        write_json_reports(&reports, output_dir).await?;
    }

    // every readable file was still emitted above; exit nonzero just to flag
    // that something didn't process cleanly.
    if failed_files > 0 {
        error!("{failed_files} input file(s) could not be processed");
        std::process::exit(1);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        CaptureFormat, DataLink, detect_capture_format, link_layer_offset, output_path,
        should_emit, sidecar_output_dir,
    };
    use rayhunter::analysis::analyzer::{AnalysisRow, Event, EventType};
    use std::path::{Path, PathBuf};

    #[test]
    fn link_layer_offsets() {
        // Raw-IP link types need no stripping; framed ones strip their header.
        assert_eq!(link_layer_offset(DataLink::RAW), Some(0));
        assert_eq!(link_layer_offset(DataLink::IPV4), Some(0));
        assert_eq!(link_layer_offset(DataLink::ETHERNET), Some(14));
        assert_eq!(link_layer_offset(DataLink::NULL), Some(4));
        assert_eq!(link_layer_offset(DataLink::LINUX_SLL), Some(16));
        assert_eq!(link_layer_offset(DataLink::LINUX_SLL2), Some(20));
        assert_eq!(link_layer_offset(DataLink::IEEE802_11), None);
        assert_eq!(link_layer_offset(DataLink::Unknown(999)), None);
    }

    #[test]
    fn detects_pcapng_magic() {
        assert_eq!(
            detect_capture_format([0x0A, 0x0D, 0x0D, 0x0A]),
            Some(CaptureFormat::PcapNg)
        );
    }

    #[test]
    fn detects_legacy_pcap_magics() {
        // Micro- and nanosecond resolution, both byte orders.
        for magic in [
            [0xA1, 0xB2, 0xC3, 0xD4],
            [0xD4, 0xC3, 0xB2, 0xA1],
            [0xA1, 0xB2, 0x3C, 0x4D],
            [0x4D, 0x3C, 0xB2, 0xA1],
        ] {
            assert_eq!(detect_capture_format(magic), Some(CaptureFormat::Pcap));
        }
    }

    #[test]
    fn rejects_unknown_magic() {
        assert_eq!(detect_capture_format([0x00, 0x00, 0x00, 0x00]), None);
        assert_eq!(detect_capture_format([0x7F, 0x45, 0x4C, 0x46]), None);
    }

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
