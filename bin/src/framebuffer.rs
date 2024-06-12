use image::{codecs::gif::GifDecoder, imageops::FilterType, AnimationDecoder, DynamicImage};
use std::{io::Cursor, time::Duration};
use include_dir::{include_dir, Dir};

const FB_PATH:&str = "/dev/fb0";
static IMAGE_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/static/images/");


#[derive(Copy, Clone)]
// TODO actually poll for this, maybe w/ fbset?
struct Dimensions {
    height: u32,
    width: u32,
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


    pub fn draw_img(&mut self, img_name: &str) {
        let img = IMAGE_DIR.get_file(img_name).unwrap();
        if img_name.ends_with(".gif") {
            // this is dumb and i'm sure there's a better way to loop this
            let cursor = Cursor::new(img.contents());
            let decoder = GifDecoder::new(cursor).unwrap();
            for maybe_frame in decoder.into_frames() {
                let frame = maybe_frame.unwrap();
                let (numerator, _) = frame.delay().numer_denom_ms();
                let img = DynamicImage::from(frame.into_buffer());
                self.write(img);
                std::thread::sleep(Duration::from_millis(numerator as u64));
            }
        } else {
            let img = image::load_from_memory(img.contents()).unwrap();
            self.write(img);
        }
    }
}