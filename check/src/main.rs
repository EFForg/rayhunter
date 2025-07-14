use clap::Parser;
use futures::TryStreamExt;
use log::{error, info, warn};
use rayhunter::{
    analysis::analyzer::{AnalyzerConfig, EventType, Harness},
    diag::DataType,
    gsmtap_parser,
    pcap::GsmtapPcapWriter,
    qmdl::QmdlReader,
};
use std::{collections::HashMap, future, path::PathBuf, pin::pin};
use tokio::fs::File;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short = 'p', long)]
    path: PathBuf,

    #[arg(long)]
    pcapify: bool,

    #[arg(long)]
    show_skipped: bool,

    #[arg(short, long)]
    verbose: bool,
}

async fn analyze_file(qmdl_path: &str, show_skipped: bool) {
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
    let mut skipped_reasons: HashMap<String, i32> = HashMap::new();
    let mut total_messages = 0;
    let mut warnings = 0;
    let mut skipped = 0;
    while let Some(container) = qmdl_stream
        .try_next()
        .await
        .expect("failed getting QMDL container")
    {
        for row in harness.analyze_qmdl_messages(container) {
            total_messages += 1;
            if let Some(reason) = row.skipped_message_reason {
                *skipped_reasons.entry(reason).or_insert(0) += 1;
                skipped += 1;
                continue;
            }
            for maybe_event in row.events {
                let Some(event) = maybe_event else { continue };
                let Some(timestamp) = row.packet_timestamp else { continue };
                match event.event_type {
                    EventType::Informational => {
                        info!(
                            "{}: INFO - {} {}",
                            qmdl_path, timestamp, event.message,
                        );
                    }
                    EventType::QualitativeWarning { severity } => {
                        warn!(
                            "{}: WARNING (Severity: {:?}) - {} {}",
                            qmdl_path, severity, timestamp, event.message,
                        );
                        warnings += 1;
                    }
                }
            }
        }
    }
    if show_skipped && skipped > 0 {
        info!("{qmdl_path}: messages skipped:");
        for (reason, count) in skipped_reasons.iter() {
            info!("    - {count}: \"{reason}\"");
        }
    }
    info!(
        "{qmdl_path}: {total_messages} messages analyzed, {warnings} warnings, {skipped} messages skipped"
    );
}

async fn pcapify(qmdl_path: &PathBuf) {
    let qmdl_file = &mut File::open(&qmdl_path)
        .await
        .expect("failed to open qmdl file");
    let qmdl_file_size = qmdl_file.metadata().await.unwrap().len();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(qmdl_file_size as usize));
    let mut pcap_path = qmdl_path.clone();
    pcap_path.set_extension("pcap");
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
    let level = if args.verbose {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Warn
    };
    simple_logger::SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .with_level(level)
        //Filter out a stupid massive amount of uneccesary warnings from hampi about undecoded extensions
        .with_module_level("asn1_codecs", log::LevelFilter::Error)
        .init()
        .unwrap();
    info!("Analyzers:");

    let harness = Harness::new_with_config(&AnalyzerConfig::default());
    for analyzer in harness.get_metadata().analyzers {
        info!("    - {}: {} (v{})", analyzer.name, analyzer.description, analyzer.version);
    }

    for maybe_entry in WalkDir::new(&args.path) {
        let Ok(entry) = maybe_entry else {
            error!("failed to open dir entry {:?}", maybe_entry);
            continue;
        };
        let name = entry.file_name();
        let name_str = name.to_str().unwrap();
        // instead of relying on the QMDL extension, can we check if a file is
        // QMDL by inspecting the contents?
        if name_str.ends_with(".qmdl") {
            let path = entry.path();
            let path_str = path.to_str().unwrap();
            analyze_file(path_str, args.show_skipped).await;
            if args.pcapify {
                pcapify(&path.to_path_buf()).await;
            }
        }
    }
}
