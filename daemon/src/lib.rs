pub mod analysis;
pub mod battery;
pub mod config;
pub mod diag;
pub mod display;
pub mod error;
pub mod key_input;
pub mod notifications;
pub mod pcap;
pub mod qmdl_store;
pub mod server;
pub mod stats;

#[cfg(feature = "apidocs")]
use utoipa::OpenApi;

// Add anotated paths to api docs
#[cfg(feature = "apidocs")]
#[derive(OpenApi)]
#[openapi(
    info(
        description = "OpenAPI documentation for Rayhunter daemon\n\n**Note:** API endpoints are subject to change as needs arise, though we will try to keep them as stable as possible and notify about breaking changes in the changelogs for new versions.\n\nNo endpoints require any authentication. To use the in-browser execution on this page, you may need to disable CORS temporarily for your browser.",
        license(
            name = "GNU General Public License v3.0",
            url = "https://github.com/EFForg/rayhunter/blob/main/LICENSE"
        )
    ),
    paths(
        pcap::get_pcap,
        server::get_qmdl,
        server::get_zip,
        stats::get_system_stats,
        stats::get_qmdl_manifest,
        stats::get_log,
        diag::start_recording,
        diag::stop_recording,
        diag::delete_recording,
        diag::delete_all_recordings,
        diag::get_analysis_report,
        analysis::get_analysis_status,
        analysis::start_analysis,
        server::get_config,
        server::set_config,
        server::test_notification,
        server::get_time,
        server::set_time_offset,
        server::debug_set_display_state
    ),
    servers(
        (
            url = "http://localhost:8080",
            description = "ADB port bridge"
        ),
        (
            url = "http://192.168.1.1:8080",
            description = "Orbic WiFi GUI"
        ),
        (
            url = "http://192.168.0.1:8080",
            description = "TPLink WiFi GUI"
        ),
    )
)]
pub struct ApiDocs;

#[cfg(feature = "apidocs")]
impl ApiDocs {
    pub fn generate() -> String {
        ApiDocs::openapi().to_pretty_json().unwrap()
    }
}
