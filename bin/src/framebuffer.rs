use image::{codecs::gif::GifDecoder, imageops::FilterType, AnimationDecoder, DynamicImage};
use std::{io::Cursor, time::Duration};

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