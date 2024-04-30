use std::{future, path::PathBuf, pin::pin};
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
    while let Some(container) = qmdl_stream.try_next().await.expect("failed getting QMDL container") {
        harness.analyze_qmdl_messages(container)
    }

    let report = harness.build_analysis_report();
    println!("{}", serde_json::to_string(&report).expect("failed to serialize report"));
}
