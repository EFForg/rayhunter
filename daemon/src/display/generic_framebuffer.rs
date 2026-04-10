use async_trait::async_trait;
use image::{AnimationDecoder, DynamicImage, codecs::gif::GifDecoder, imageops::FilterType};
use std::io::Cursor;
use std::time::Duration;

use crate::config;
use crate::display::DisplayState;
use rayhunter::analysis::analyzer::EventType;

use log::{error, info};
use tokio::sync::mpsc::Receiver;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

use include_dir::{Dir, include_dir};

static IMAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/images/");
const REFRESH_RATE: u64 = 1000; //how often in milliseconds to refresh the display
const SEVERITY_RUNTIME_DIR: &str = "/data/rayhunter/severity-indicator-images";

#[derive(Copy, Clone)]
pub struct Dimensions {
    pub height: u32,
    pub width: u32,
}

#[derive(Copy, Clone)]
pub enum LinePattern {
    Solid,
    Dashed, // _ _ _ _
    Dotted, // . . . .
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
    Cyan,
    Yellow,
    Pink,
    Orange,
}

impl Color {
    fn rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Red => (0xff, 0, 0),
            Color::Green => (0, 0xff, 0),
            Color::Blue => (0, 0, 0xff),
            Color::White => (0xff, 0xff, 0xff),
            Color::Black => (0, 0, 0),
            Color::Cyan => (0, 0xff, 0xff),
            Color::Yellow => (0xff, 0xff, 0),
            Color::Pink => (0xfe, 0x24, 0xff),
            Color::Orange => (0xff, 0xa5, 0),
        }
    }
}

pub(crate) fn display_style_from_state(state: DisplayState, colorblind_mode: bool) -> (Color, LinePattern) {
    match state {
        DisplayState::Paused => (Color::White, LinePattern::Solid),
        DisplayState::Recording => {
            if colorblind_mode {
                (Color::Blue, LinePattern::Solid)
            } else {
                (Color::Green, LinePattern::Solid)
            }
        }
        DisplayState::WarningDetected { event_type } => match event_type {
            EventType::Informational => {
                if colorblind_mode {
                    (Color::Blue, LinePattern::Solid)
                } else {
                    (Color::Green, LinePattern::Solid)
                }
            }
            EventType::Low => (Color::Yellow, LinePattern::Dotted),
            EventType::Medium => (Color::Orange, LinePattern::Dashed),
            EventType::High => (Color::Red, LinePattern::Solid),
        },
    }
}

#[derive(Clone, Copy)]
pub enum SeverityIndicatorImageSlot {
    Default,
    Low,
    Medium,
    High,
}

impl SeverityIndicatorImageSlot {
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

    fn bundled_path(self) -> &'static str {
        match self {
            Self::Default => "severity/indicator_default.png",
            Self::Low => "severity/indicator_low.png",
            Self::Medium => "severity/indicator_medium.png",
            Self::High => "severity/indicator_high.png",
        }
    }

    fn bundled_bytes(self) -> &'static [u8] {
        IMAGE_DIR
            .get_file(self.bundled_path())
            .unwrap_or_else(|| panic!("missing bundled severity indicator {}", self.filename()))
            .contents()
    }

    fn path(self) -> std::path::PathBuf {
        std::path::PathBuf::from(SEVERITY_RUNTIME_DIR).join(self.filename())
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

#[derive(serde::Serialize)]
#[cfg_attr(feature = "apidocs", derive(utoipa::ToSchema))]
pub struct SeverityIndicatorImageStatus {
    pub runtime_dir: String,
    pub slots_with_overrides: Vec<String>,
}

pub async fn get_severity_indicator_image_status() -> SeverityIndicatorImageStatus {
    let mut slots_with_overrides = Vec::new();
    for slot in SeverityIndicatorImageSlot::all() {
        if tokio::fs::metadata(slot.path()).await.is_ok() {
            slots_with_overrides.push(slot.as_str().to_string());
        }
    }

    SeverityIndicatorImageStatus {
        runtime_dir: SEVERITY_RUNTIME_DIR.to_string(),
        slots_with_overrides,
    }
}

pub async fn store_severity_indicator_image(
    slot: SeverityIndicatorImageSlot,
    bytes: &[u8],
) -> Result<(), std::io::Error> {
    tokio::fs::create_dir_all(SEVERITY_RUNTIME_DIR).await?;
    tokio::fs::write(slot.path(), bytes).await
}

pub async fn remove_severity_indicator_image(
    slot: SeverityIndicatorImageSlot,
) -> Result<(), std::io::Error> {
    match tokio::fs::remove_file(slot.path()).await {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err),
    }
}

async fn load_severity_indicator_image(slot: SeverityIndicatorImageSlot) -> Vec<u8> {
    match tokio::fs::read(slot.path()).await {
        Ok(bytes) => bytes,
        Err(_) => slot.bundled_bytes().to_vec(),
    }
}

#[async_trait]
pub trait GenericFramebuffer: Send + 'static {
    fn dimensions(&self) -> Dimensions;

    async fn write_buffer(&mut self, buffer: Vec<(u8, u8, u8)>); // rgb, row-wise, left-to-right, top-to-bottom

    async fn write_dynamic_image(&mut self, img: DynamicImage) {
        let dimensions = self.dimensions();
        let mut width = img.width();
        let mut height = img.height();
        let resized_img: DynamicImage;
        if height > dimensions.height || width > dimensions.width {
            resized_img = img.resize(dimensions.width, dimensions.height, FilterType::CatmullRom);
            width = dimensions.width.min(resized_img.width());
            height = dimensions.height.min(resized_img.height());
        } else {
            resized_img = img;
        }
        let img_rgba8 = resized_img.to_rgba8();
        let mut buf = Vec::with_capacity((height * width).try_into().unwrap());
        for y in 0..height {
            for x in 0..width {
                let px = img_rgba8.get_pixel(x, y);
                buf.push((px[0], px[1], px[2]));
            }
        }

        self.write_buffer(buf).await
    }

    async fn draw_gif(&mut self, img_buffer: &[u8]) {
        let cursor = Cursor::new(img_buffer);
        if let Ok(decoder) = GifDecoder::new(cursor) {
            let frames: Vec<_> = decoder
                .into_frames()
                .filter_map(|f| f.ok())
                .map(|frame| {
                    let (numerator, _) = frame.delay().numer_denom_ms();
                    let img = DynamicImage::from(frame.into_buffer());
                    (img, numerator as u64)
                })
                .collect();

            for (img, delay_ms) in frames {
                self.write_dynamic_image(img).await;
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
            }
        }
    }

    async fn draw_img(&mut self, img_buffer: &[u8]) {
        let img = image::load_from_memory(img_buffer).unwrap();
        self.write_dynamic_image(img).await
    }

    async fn draw_line(&mut self, color: Color, height: u32) {
        self.draw_patterned_line(color, height, LinePattern::Solid)
            .await
    }

    async fn draw_patterned_line(&mut self, color: Color, height: u32, pattern: LinePattern) {
        let width = self.dimensions().width;
        let mut buffer = Vec::with_capacity((height * width).try_into().unwrap());

        for _row in 0..height {
            for col in 0..width {
                let should_draw = match pattern {
                    LinePattern::Solid => true,
                    LinePattern::Dashed => (col / 4) % 2 == 0, // 4 pixels on, 4 pixels off
                    LinePattern::Dotted => col % 4 == 0,       // 1 pixel on, 3 pixels off
                };

                if should_draw {
                    buffer.push(color.rgb());
                } else {
                    buffer.push((0, 0, 0)); // Black background
                }
            }
        }

        self.write_buffer(buffer).await
    }
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    mut fb: impl GenericFramebuffer,
    shutdown_token: CancellationToken,
    mut ui_update_rx: Receiver<DisplayState>,
) {
    let display_level = config.ui_level;
    if display_level == 0 {
        info!("Invisible mode, not spawning UI.");
        return;
    }

    let colorblind_mode = config.colorblind_mode;
    let mut current_state = DisplayState::Recording;
    let mut display_style = display_style_from_state(current_state, colorblind_mode);

    task_tracker.spawn(async move {
        // this feels wrong, is there a more rusty way to do this?
        let mut img: Option<&[u8]> = None;
        let mut severity_default_img: Option<Vec<u8>> = None;
        let mut severity_low_img: Option<Vec<u8>> = None;
        let mut severity_medium_img: Option<Vec<u8>> = None;
        let mut severity_high_img: Option<Vec<u8>> = None;
        if display_level == 2 {
            img = Some(
                IMAGE_DIR
                    .get_file("orca.gif")
                    .expect("failed to read orca.gif")
                    .contents(),
            );
        } else if display_level == 3 {
            img = Some(
                IMAGE_DIR
                    .get_file("eff.png")
                    .expect("failed to read eff.png")
                    .contents(),
            );
        } else if display_level == 5 {
            severity_default_img = Some(load_severity_indicator_image(SeverityIndicatorImageSlot::Default).await);
            severity_low_img = Some(load_severity_indicator_image(SeverityIndicatorImageSlot::Low).await);
            severity_medium_img = Some(load_severity_indicator_image(SeverityIndicatorImageSlot::Medium).await);
            severity_high_img = Some(load_severity_indicator_image(SeverityIndicatorImageSlot::High).await);
        }
        loop {
            if shutdown_token.is_cancelled() {
                info!("received UI shutdown");
                break;
            }
            match ui_update_rx.try_recv() {
                Ok(state) => {
                    current_state = state;
                    display_style = display_style_from_state(current_state, colorblind_mode);
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
                Err(e) => error!("error receiving framebuffer update message: {e}"),
            }

            let mut status_bar_height = 2;
            match display_level {
                2 => fb.draw_gif(img.unwrap()).await,
                3 => fb.draw_img(img.unwrap()).await,
                4 => {
                    status_bar_height = fb.dimensions().height;
                }
                5 => {
                    let severity_img = match SeverityIndicatorImageSlot::from_display_state(current_state) {
                        SeverityIndicatorImageSlot::Default => severity_default_img.as_ref().unwrap(),
                        SeverityIndicatorImageSlot::Low => severity_low_img.as_ref().unwrap(),
                        SeverityIndicatorImageSlot::Medium => severity_medium_img.as_ref().unwrap(),
                        SeverityIndicatorImageSlot::High => severity_high_img.as_ref().unwrap(),
                    };
                    fb.draw_img(severity_img).await;
                }
                128 => {
                    fb.draw_line(Color::Cyan, 128).await;
                    fb.draw_line(Color::Pink, 102).await;
                    fb.draw_line(Color::White, 76).await;
                    fb.draw_line(Color::Pink, 50).await;
                    fb.draw_line(Color::Cyan, 25).await;
                }
                // this branch is for ui_level 1, which is also the default if an
                // unknown value is used
                _ => {}
            };
            let (color, pattern) = display_style;
            fb.draw_patterned_line(color, status_bar_height, pattern)
                .await;
            tokio::time::sleep(Duration::from_millis(REFRESH_RATE)).await;
        }
    });
}
