use image::{io::Reader as ImageReader, AnimationDecoder, imageops::FilterType, codecs::gif::GifDecoder, DynamicImage};
use std::{io::BufReader, fs::File, time::Duration};
use include_dir::{include_dir, Dir};
use log::{info, error};

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

        let resized_img = img.resize( self.dimensions.width, self.dimensions.height, FilterType::CatmullRom);
        let width = self.dimensions.width.min(resized_img.width());
        let height = self.dimensions.height.min(resized_img.height());
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
        //let img_path = IMAGE_DIR.get_file(img_name).unwrap().path();
        let img_path = img_name;
        info!("img_path: {:?}", img_path);
        if img_path.ends_with(".gif") {
            loop{
                // this is dumb and i'm sure there's a better way to loop this
                let stream = BufReader::new(File::open(&img_path).unwrap());
                let decoder = GifDecoder::new(stream).unwrap();
                for maybe_frame in decoder.into_frames() {
                    let frame = maybe_frame.unwrap();
                    let (numerator, _) = frame.delay().numer_denom_ms();
                    let img = DynamicImage::from(frame.into_buffer());
                    self.write(img);
                    std::thread::sleep(Duration::from_millis(numerator as u64));
                }
            }
        } else {
            let img_reader = ImageReader::open(img_path).unwrap();
            let img = img_reader.decode().unwrap();
            self.write(img);
        }
    }
}