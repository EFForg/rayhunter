use std::{collections::HashMap, future, path::PathBuf, pin::pin};
use rayhunter::{analysis::analyzer::Harness, diag::DataType, qmdl::QmdlReader};
use tokio::fs::{metadata, read_dir, File};
use clap::Parser;
use futures::TryStreamExt;

mod dummy_analyzer;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    qmdl_path: PathBuf,

    #[arg(long)]
    show_skipped: bool,

    #[arg(long)]
    enable_dummy_analyzer: bool,
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
                if let Some(event) = maybe_event {
                    warnings += 1;
                    println!("{}: {:?}", analysis.timestamp, event);
                }
            }
        }
    }
    if show_skipped && skipped > 0 {
        println!("{}: messages skipped:", qmdl_path);
        for (reason, count) in skipped_reasons.iter() {
            println!("    - {}: \"{}\"", count, reason);
        }
    }
    println!("{}: {} messages analyzed, {} warnings, {} messages skipped", qmdl_path, total_messages, warnings, skipped);
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut harness = Harness::new_with_all_analyzers();
    if args.enable_dummy_analyzer {
        harness.add_analyzer(Box::new(dummy_analyzer::TestAnalyzer { count: 0 }));
    }
    println!("Analyzers:");
    for analyzer in harness.get_metadata().analyzers {
        println!("    - {}: {}", analyzer.name, analyzer.description);
    }

    let metadata = metadata(&args.qmdl_path).await.expect("failed to get metadata");
    if metadata.is_dir() {
        let mut dir = read_dir(&args.qmdl_path).await.expect("failed to read dir");
        while let Some(entry) = dir.next_entry().await.expect("failed to get entry") {
            let name = entry.file_name();
            let name_str = name.to_str().unwrap();
            if name_str.ends_with(".qmdl") {
                analyze_file(&mut harness, entry.path().to_str().unwrap(), args.show_skipped).await;
            }
        }
    } else {
        analyze_file(&mut harness, args.qmdl_path.to_str().unwrap(), args.show_skipped).await;
    }
}
