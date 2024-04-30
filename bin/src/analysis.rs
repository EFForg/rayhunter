use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use log::error;
use rayhunter::{analysis::analyzer::{AnalysisReport, Harness}, diag::MessagesContainer};
use tokio::sync;
use tokio_util::task::TaskTracker;

use crate::server::ServerState;

#[derive(Debug)]
pub enum AnalysisMessage {
    Reset,
    GetReport(sync::oneshot::Sender<AnalysisReport>),
    AnalyzeContainer(MessagesContainer),
    StopThread,
}

pub fn run_analysis_thread(task_tracker: &TaskTracker) -> sync::mpsc::Sender<AnalysisMessage> {
    let (tx, mut rx) = sync::mpsc::channel(5);

    task_tracker.spawn(async move {
        let mut harness = Harness::new_with_all_analyzers();
        loop {
            match rx.recv().await {
                Some(AnalysisMessage::GetReport(sender)) => {
                    // this might fail if the client closes their connection
                    // before we're done building the report
                    if let Err(e) = sender.send(harness.build_analysis_report()) {
                        error!("failed to send analysis report: {:?}", e);
                    }
                },
                Some(AnalysisMessage::Reset) => harness = Harness::new_with_all_analyzers(),
                Some(AnalysisMessage::AnalyzeContainer(container)) => harness.analyze_qmdl_messages(container),
                Some(AnalysisMessage::StopThread) | None => break,
            }
        }
    });

    tx
}

pub async fn get_analysis_report(State(state): State<Arc<ServerState>>) -> Result<Json<AnalysisReport>, (StatusCode, String)> {
    if state.readonly_mode {
        return Err((StatusCode::FORBIDDEN, "server is in readonly mode".to_string()));
    }
    let analysis_tx = state.maybe_analysis_tx.as_ref().unwrap();
    let (report_tx, report_rx) = tokio::sync::oneshot::channel();
    if let Err(e) = analysis_tx.send(AnalysisMessage::GetReport(report_tx)).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("error reaching analysis thread: {:?}", e)));
    }
    match report_rx.await {
        Ok(report) => Ok(Json(report)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("error fetching analysis report: {:?}", e)))
    }
}
