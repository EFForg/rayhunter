use std::path::PathBuf;

use include_dir::{Dir, include_dir};
use log::{error, info};
use rayhunter::analysis::analyzer::EventType;
use serde::Serialize;
use tokio::sync::mpsc::Receiver;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use crate::config;
use crate::display::DisplayState;
use crate::display::generic_framebuffer::{self, GenericFramebuffer};

const REFRESH_RATE: u64 = 1000;
const RUNTIME_DIR: &str = "/data/rayhunter/orbic-display-images";
static IMAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/images/orbic/severity");

#[derive(Clone, Copy)]
pub enum OrbicSeverityImageSlot {
    Default,
    Low,
    Medium,
    High,
}

impl OrbicSeverityImageSlot {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "default" => Some(Self::Default),
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    pub fn filename(self) -> &'static str {
        match self {
            Self::Default => "indicator_default.png",
            Self::Low => "indicator_low.png",
            Self::Medium => "indicator_medium.png",
            Self::High => "indicator_high.png",
        }
    }

    fn bundled_bytes(self) -> &'static [u8] {
        IMAGE_DIR
            .get_file(self.filename())
            .unwrap_or_else(|| panic!("missing bundled severity indicator {}", self.filename()))
            .contents()
    }

    fn path(self) -> PathBuf {
        PathBuf::from(RUNTIME_DIR).join(self.filename())
    }

    fn from_display_state(state: DisplayState) -> Self {
        match state {
            DisplayState::Paused | DisplayState::Recording => Self::Default,
            DisplayState::WarningDetected { event_type } => match event_type {
                EventType::Informational => Self::Default,
                EventType::Low => Self::Low,
                EventType::Medium => Self::Medium,
                EventType::High => Self::High,
            },
        }
    }

    fn all() -> [Self; 4] {
        [Self::Default, Self::Low, Self::Medium, Self::High]
    }
}

#[derive(Serialize)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct OrbicSeverityIndicatorImageStatus {
    pub runtime_dir: String,
    pub slots_with_overrides: Vec<String>,
}

pub async fn get_status() -> OrbicSeverityIndicatorImageStatus {
    let mut slots_with_overrides = Vec::new();
    for slot in OrbicSeverityImageSlot::all() {
        if tokio::fs::metadata(slot.path()).await.is_ok() {
            slots_with_overrides.push(slot.as_str().to_string());
        }
    }

    OrbicSeverityIndicatorImageStatus {
        runtime_dir: RUNTIME_DIR.to_string(),
        slots_with_overrides,
    }
}

pub async fn store_override(
    slot: OrbicSeverityImageSlot,
    bytes: &[u8],
) -> Result<(), std::io::Error> {
    tokio::fs::create_dir_all(RUNTIME_DIR).await?;
    tokio::fs::write(slot.path(), bytes).await
}

pub async fn remove_override(slot: OrbicSeverityImageSlot) -> Result<(), std::io::Error> {
    match tokio::fs::remove_file(slot.path()).await {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

async fn load_slot(slot: OrbicSeverityImageSlot) -> Vec<u8> {
    match tokio::fs::read(slot.path()).await {
        Ok(bytes) => bytes,
        Err(_) => slot.bundled_bytes().to_vec(),
    }
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    mut fb: impl GenericFramebuffer,
    shutdown_token: CancellationToken,
    mut ui_update_rx: Receiver<DisplayState>,
) {
    let colorblind_mode = config.colorblind_mode;

    task_tracker.spawn(async move {
        let default_img = load_slot(OrbicSeverityImageSlot::Default).await;
        let low_img = load_slot(OrbicSeverityImageSlot::Low).await;
        let medium_img = load_slot(OrbicSeverityImageSlot::Medium).await;
        let high_img = load_slot(OrbicSeverityImageSlot::High).await;

        let mut state = DisplayState::Recording;

        loop {
            if shutdown_token.is_cancelled() {
                info!("received UI shutdown");
                break;
            }

            match ui_update_rx.try_recv() {
                Ok(new_state) => state = new_state,
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
                Err(err) => error!("error receiving framebuffer update message: {err}"),
            }

            let image = match OrbicSeverityImageSlot::from_display_state(state) {
                OrbicSeverityImageSlot::Default => &default_img,
                OrbicSeverityImageSlot::Low => &low_img,
                OrbicSeverityImageSlot::Medium => &medium_img,
                OrbicSeverityImageSlot::High => &high_img,
            };

            fb.draw_img(image).await;
            let (color, pattern) = generic_framebuffer::display_style_from_state(state, colorblind_mode);
            fb.draw_patterned_line(color, 2, pattern).await;
            tokio::time::sleep(std::time::Duration::from_millis(REFRESH_RATE)).await;
        }
    });
}
