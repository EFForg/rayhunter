use image::{AnimationDecoder, DynamicImage, codecs::gif::GifDecoder, imageops::FilterType};
use std::io::Cursor;
use std::time::Duration;

use crate::config;
use crate::display::DisplayState;

use log::{error, info};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;
use tokio_util::task::TaskTracker;

use std::thread::sleep;

use include_dir::{Dir, include_dir};

#[derive(Copy, Clone)]
pub struct Dimensions {
    pub height: u32,
    pub width: u32,
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
        }
    }
}

impl Color {
    fn from_state(state: DisplayState, colorblind_mode: bool) -> Self {
        match state {
            DisplayState::Paused => Color::White,
            DisplayState::Recording => {
                if colorblind_mode {
                    Color::Blue
                } else {
                    Color::Green
                }
            }
            DisplayState::WarningDetected => Color::Red,
        }
    }
}

pub trait GenericFramebuffer: Send + 'static {
    fn dimensions(&self) -> Dimensions;

    fn write_buffer(
        &mut self,
        buffer: &[(u8, u8, u8)], // rgb, row-wise, left-to-right, top-to-bottom
    );

    fn write_dynamic_image(&mut self, img: DynamicImage) {
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
        let img_rgba8 = resized_img.as_rgba8().unwrap();
        let mut buf = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let px = img_rgba8.get_pixel(x, y);
                buf.push((px[0], px[1], px[2]));
            }
        }

        self.write_buffer(&buf);
    }

    fn draw_gif(&mut self, img_buffer: &[u8]) {
        // this is dumb and i'm sure there's a better way to loop this
        let cursor = Cursor::new(img_buffer);
        let decoder = GifDecoder::new(cursor).unwrap();
        for maybe_frame in decoder.into_frames() {
            let frame = maybe_frame.unwrap();
            let (numerator, _) = frame.delay().numer_denom_ms();
            let img = DynamicImage::from(frame.into_buffer());
            self.write_dynamic_image(img);
            std::thread::sleep(Duration::from_millis(numerator as u64));
        }
    }

    fn draw_img(&mut self, img_buffer: &[u8]) {
        let img = image::load_from_memory(img_buffer).unwrap();
        self.write_dynamic_image(img);
    }

    fn draw_line(&mut self, color: Color, height: u32) {
        let width = self.dimensions().width;
        let px_num = height * width;
        let mut buffer = Vec::new();
        for _ in 0..px_num {
            buffer.push(color.rgb());
        }

        self.write_buffer(&buffer);
    }
}

pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    mut fb: impl GenericFramebuffer,
    mut ui_shutdown_rx: oneshot::Receiver<()>,
    mut ui_update_rx: Receiver<DisplayState>,
) {
    static IMAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/images/");
    let display_level = config.ui_level;
    if display_level == 0 {
        info!("Invisible mode, not spawning UI.");
    }

    let colorblind_mode = config.colorblind_mode;
    let mut display_color = Color::from_state(DisplayState::Recording, colorblind_mode);

    task_tracker.spawn_blocking(move || {
        // this feels wrong, is there a more rusty way to do this?
        let mut img: Option<&[u8]> = None;
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
        }
        loop {
            match ui_shutdown_rx.try_recv() {
                Ok(_) => {
                    info!("received UI shutdown");
                    break;
                }
                Err(TryRecvError::Empty) => {}
                Err(e) => panic!("error receiving shutdown message: {e}"),
            }
            match ui_update_rx.try_recv() {
                Ok(state) => {
                    display_color = Color::from_state(state, colorblind_mode);
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
                Err(e) => error!("error receiving framebuffer update message: {e}"),
            }

            match display_level {
                2 => {
                    fb.draw_gif(img.unwrap());
                }
                3 => fb.draw_img(img.unwrap()),
                128 => {
                    fb.draw_line(Color::Cyan, 128);
                    fb.draw_line(Color::Pink, 102);
                    fb.draw_line(Color::White, 76);
                    fb.draw_line(Color::Pink, 50);
                    fb.draw_line(Color::Cyan, 25);
                }
                _ => {
                    // this branch id for ui_level 1, which is also the default if an
                    // unknown value is used
                    fb.draw_line(display_color, 2);
                }
            };
            sleep(Duration::from_millis(1000));
        }
    });
}
