use image::{codecs::gif::GifDecoder, imageops::FilterType, AnimationDecoder, DynamicImage};
use std::time::Duration;
use std::io::Cursor;

use crate::config;
use crate::display::DisplayState;

use log::{error, info};
use tokio::sync::oneshot;
use tokio::sync::mpsc::Receiver;
use tokio_util::task::TaskTracker;
use tokio::sync::oneshot::error::TryRecvError;

use std::thread::sleep;

use include_dir::{include_dir, Dir};

const FB_PATH:&str = "/dev/fb0";

#[derive(Copy, Clone)]
// TODO actually poll for this, maybe w/ fbset?
struct Dimensions {
    height: u32,
    width: u32,
}

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum Color565 {
    Red    = 0b1111100000000000,
    Green  = 0b0000011111100000,
    Blue   = 0b0000000000011111,
    White  = 0b1111111111111111,
    Black  = 0b0000000000000000,
    Cyan   = 0b0000011111111111,
    Yellow = 0b1111111111100000,
    Pink =   0b1111010010011111,
}

impl From<DisplayState> for Color565 {
    fn from(state: DisplayState) -> Self {
        match state {
            DisplayState::Paused => Color565::White,
            DisplayState::Recording => Color565::Green, 
            DisplayState::RecordingCBM => Color565::Blue, 
            DisplayState::WarningDetected => Color565::Red,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Framebuffer<'a> {
    dimensions: Dimensions,
    path: &'a str,
}

impl Framebuffer<'_>{
    pub const fn new() -> Self {
        Framebuffer{
            dimensions: Dimensions{height: 128, width: 128},
            path: FB_PATH,
        }
    }

    fn write(&mut self, img: DynamicImage) {
        let mut width = img.width();
        let mut height = img.height();
        let resized_img: DynamicImage;
        if height > self.dimensions.height ||
        width > self.dimensions.width {
            resized_img = img.resize( self.dimensions.width, self.dimensions.height, FilterType::CatmullRom);
            width = self.dimensions.width.min(resized_img.width());
            height = self.dimensions.height.min(resized_img.height());
        } else {
            resized_img = img;
        }
        let img_rgba8 = resized_img.as_rgba8().unwrap();
        let mut buf = Vec::new();
        for y in 0..height {
            for x in 0..width {
                let px = img_rgba8.get_pixel(x, y);
                let mut rgb565: u16 = (px[0] as u16 & 0b11111000) << 8;
                rgb565 |= (px[1] as u16 & 0b11111100) << 3;
                rgb565 |= (px[2] as u16) >> 3;
                buf.extend(rgb565.to_le_bytes());
            }
        }

        std::fs::write(self.path, &buf).unwrap();
    }

    pub fn draw_gif(&mut self, img_buffer: &[u8]) {
        // this is dumb and i'm sure there's a better way to loop this
        let cursor = Cursor::new(img_buffer);
        let decoder = GifDecoder::new(cursor).unwrap();
        for maybe_frame in decoder.into_frames() {
            let frame = maybe_frame.unwrap();
            let (numerator, _) = frame.delay().numer_denom_ms();
            let img = DynamicImage::from(frame.into_buffer());
            self.write(img);
            std::thread::sleep(Duration::from_millis(numerator as u64));
        }
    }

    pub fn draw_img(&mut self, img_buffer: &[u8]) {
        let img = image::load_from_memory(img_buffer).unwrap();
        self.write(img);
    }

    pub fn draw_line(&mut self, color: Color565, height: u32){
        let px_num= height * self.dimensions.width;
        let color: u16 = color as u16;
        let mut buffer: Vec<u8> = Vec::new();
        for _ in 0..px_num {
            buffer.extend(color.to_le_bytes());
        }

        std::fs::write(self.path, &buffer).unwrap();
    }
}


pub fn update_ui(
    task_tracker: &TaskTracker,
    config: &config::Config,
    mut ui_shutdown_rx: oneshot::Receiver<()>,
    mut ui_update_rx: Receiver<DisplayState>
) {
    static IMAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static/images/");
    let mut display_color: Color565;
    let display_level = config.ui_level;
    if display_level == 0 {
        info!("Invisible mode, not spawning UI.");
    }

    if config.colorblind_mode {
        display_color = Color565::Blue;
    } else {
        display_color = Color565::Green;
    }

    task_tracker.spawn_blocking(move || {
        let mut fb: Framebuffer = Framebuffer::new();
        // this feels wrong, is there a more rusty way to do this?
        let mut img: Option<&[u8]> = None;
        if display_level == 2 {
            img = Some(IMAGE_DIR.get_file("orca.gif").expect("failed to read orca.gif").contents());
        } else if display_level == 3 {
            img = Some(IMAGE_DIR.get_file("eff.png").expect("failed to read eff.png").contents());
        }
        loop {
            match ui_shutdown_rx.try_recv() {
                Ok(_) => {
                    info!("received UI shutdown");
                    break;
                },
                Err(TryRecvError::Empty) => {},
                Err(e) => panic!("error receiving shutdown message: {e}")
            }
            match ui_update_rx.try_recv() {
                    Ok(state) => {
                        display_color = state.into();
                    },
                    Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {},
                    Err(e) => error!("error receiving framebuffer update message: {e}")
            }

            match display_level  {
                2 => {
                    fb.draw_gif(img.unwrap());
                },
                3 => {
                    fb.draw_img(img.unwrap())
                },
                128 => {
                    fb.draw_line(Color565::Cyan, 128);
                    fb.draw_line(Color565::Pink, 102);
                    fb.draw_line(Color565::White, 76);
                    fb.draw_line(Color565::Pink, 50);
                    fb.draw_line(Color565::Cyan, 25);
                },
                _ => { // this branch id for ui_level 1, which is also the default if an
                       // unknown value is used
                    fb.draw_line(display_color, 2);
                },
            };
            sleep(Duration::from_millis(1000));
        }
    });
}
