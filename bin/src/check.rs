use std::{collections::HashMap, future, path::PathBuf, pin::pin};
use log::{info, warn};
use rayhunter::{analysis::analyzer::{EventType, Harness}, diag::DataType, gsmtap_parser, pcap::GsmtapPcapWriter, qmdl::QmdlReader};
use tokio::fs::{metadata, read_dir, File};
use clap::Parser;
use futures::TryStreamExt;

mod dummy_analyzer;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'p', long)]
    qmdl_path: PathBuf,

    #[arg(short = 'c', long)]
    pcapify: bool,

    #[arg(long)]
    show_skipped: bool,

    #[arg(long)]
    enable_dummy_analyzer: bool,

    #[arg(short, long)]
    verbose: bool,
}

async fn analyze_file(harness: &mut Harness, qmdl_path: &str, show_skipped: bool) {
    let qmdl_file = &mut File::open(&qmdl_path).await.expect("failed to open file");
    let file_size = qmdl_file.metadata().await.expect("failed to get QMDL file metadata").len();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(file_size as usize));
    let mut qmdl_stream = pin!(qmdl_reader.as_stream()
        .try_filter(|container| future::ready(container.data_type == DataType::UserSpace)));
    let mut skipped_reasons: HashMap<String, i32> = HashMap::new();
    let mut total_messages = 0;
    let mut warnings = 0;
    let mut skipped = 0;
    while let Some(container) = qmdl_stream.try_next().await.expect("failed getting QMDL container") {
        let row = harness.analyze_qmdl_messages(container);
        total_messages += 1;
        for reason in row.skipped_message_reasons {
            *skipped_reasons.entry(reason).or_insert(0) += 1;
            skipped += 1;
        }
        for analysis in row.analysis {
            for maybe_event in analysis.events {
                let Some(event) = maybe_event else { continue };
                match event.event_type {
                    EventType::Informational => {
                        info!(
                            "{}: INFO - {} {}",
                            qmdl_path,
                            analysis.timestamp,
                            event.message,
                        );
                    }
                    EventType::QualitativeWarning { severity } => {
                        warn!(
                            "{}: WARNING (Severity: {:?}) - {} {}",
                            qmdl_path,
                            severity,
                            analysis.timestamp,
                            event.message,
                        );
                        warnings += 1;
                    }
                }
            }
        }
    }
    if show_skipped && skipped > 0 {
        info!("{}: messages skipped:", qmdl_path);
        for (reason, count) in skipped_reasons.iter() {
            info!("    - {}: \"{}\"", count, reason);
        }
    }
    info!("{}: {} messages analyzed, {} warnings, {} messages skipped", qmdl_path, total_messages, warnings, skipped);
}

async fn pcapify(qmdl_path: &PathBuf) {
    let qmdl_file = &mut File::open(&qmdl_path).await.expect("failed to open qmdl file");
    let qmdl_file_size = qmdl_file.metadata().await.unwrap().len();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(qmdl_file_size as usize));
    let mut pcap_path = qmdl_path.clone();
    pcap_path.set_extension("pcap");
    let pcap_file = &mut File::create(&pcap_path).await.expect("failed to open pcap file");
    let mut pcap_writer = GsmtapPcapWriter::new(pcap_file).await.unwrap();
    pcap_writer.write_iface_header().await.unwrap();
    while let Some(container) = qmdl_reader.get_next_messages_container().await.expect("failed to get container") {
        for msg in container.into_messages().into_iter().flatten() {
            if let Ok(Some((timestamp, parsed))) = gsmtap_parser::parse(msg) {
                pcap_writer.write_gsmtap_message(parsed, timestamp).await.expect("failed to write");
            }
        }
    }
    info!("wrote pcap to {:?}", &pcap_path);
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let level = if args.verbose {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Warn
    };
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .with_level(level)
        .init().unwrap();

    let mut harness = Harness::new_with_all_analyzers();
    if args.enable_dummy_analyzer {
        harness.add_analyzer(Box::new(dummy_analyzer::TestAnalyzer { count: 0 }));
    }
    info!("Analyzers:");
    for analyzer in harness.get_metadata().analyzers {
        info!("    - {}: {}", analyzer.name, analyzer.description);
    }

    let metadata = metadata(&args.qmdl_path).await.expect("failed to get metadata");
    if metadata.is_dir() {
        let mut dir = read_dir(&args.qmdl_path).await.expect("failed to read dir");
        while let Some(entry) = dir.next_entry().await.expect("failed to get entry") {
            let name = entry.file_name();
            let name_str = name.to_str().unwrap();
            if name_str.ends_with(".qmdl") {
                let path = entry.path();
                let path_str = path.to_str().unwrap();
                analyze_file(&mut harness, path_str, args.show_skipped).await;
                if args.pcapify {
                    pcapify(&path).await;
                }
            }
        }
    } else {
        let path = args.qmdl_path.to_str().unwrap();
        analyze_file(&mut harness, path, args.show_skipped).await;
        if args.pcapify {
            pcapify(&args.qmdl_path).await;
        }
    }
}
