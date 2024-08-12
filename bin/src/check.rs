use std::{collections::HashMap, future, path::PathBuf, pin::pin};
use rayhunter::{analysis::analyzer::Harness, diag::DataType, qmdl::QmdlReader};
use tokio::fs::File;
use clap::Parser;
use futures::TryStreamExt;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    qmdl_path: PathBuf,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut harness = Harness::new_with_all_analyzers();

    let qmdl_file = File::open(args.qmdl_path).await.expect("failed to open QMDL file");
    let file_size = qmdl_file.metadata().await.expect("failed to get QMDL file metadata").len();
    let mut qmdl_reader = QmdlReader::new(qmdl_file, Some(file_size as usize));
    let mut qmdl_stream = pin!(qmdl_reader.as_stream()
        .try_filter(|container| future::ready(container.data_type == DataType::UserSpace)));
    println!("Analyzers:");
    for analyzer in harness.get_metadata().analyzers {
        println!("    - {}: {}", analyzer.name, analyzer.description);
    }
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
    if skipped > 0 {
        println!("Messages skipped:");
        for (reason, count) in skipped_reasons.iter() {
            println!("    - {}: \"{}\"", count, reason);
        }
    }
    println!("{} messages analyzed, {} warnings, {} messages skipped", total_messages, warnings, skipped);
}
