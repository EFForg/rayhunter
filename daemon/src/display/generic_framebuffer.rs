use async_trait::async_trait;
use fb_utils::{determine_format, get_var_screeninfo};
use image::{AnimationDecoder, DynamicImage, codecs::gif::GifDecoder, imageops::FilterType};
use std::io::Cursor;
use std::os::fd::{AsFd, BorrowedFd};
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};

use crate::config;
use crate::display::DisplayState;

use log::{error, info};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio::sync::oneshot::error::TryRecvError;
use tokio_util::task::TaskTracker;

use include_dir::{Dir, include_dir};

#[derive(Copy, Clone)]
pub struct Dimensions {
    pub height: u32,
    pub width: u32,
}

#[derive(Copy, Clone, Debug)]
pub enum FbFormat {
    ARGB888,
    ABGR888,
    RGB888,
    BGR888,
    RGB666,
    RGB565,
    BGR565,
    RGB555,
    BGR555,
    RGB444,
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
        let img_rgba8 = resized_img.as_rgba8().unwrap();
        let mut buf = Vec::new();
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
        let width = self.dimensions().width;
        let px_num = height * width;
        let mut buffer = Vec::new();
        for _ in 0..px_num {
            buffer.push(color.rgb());
        }

        self.write_buffer(buffer).await
    }
}

/// Attempt to determine the FB dimensions from FB vinfo.
pub fn read_fb_dimentions(fb: BorrowedFd<'_>) -> std::io::Result<Dimensions> {
    let vinfo = get_var_screeninfo(fb)?;
    Ok(Dimensions {
        height: vinfo.yres,
        width: vinfo.xres,
    })
}

/// Attempt to determine the FBs format
///
/// Returns `Ok(None)` if the format cannot be determined
pub fn read_fb_format(fb: BorrowedFd<'_>) -> std::io::Result<Option<FbFormat>> {
    let vinfo = get_var_screeninfo(fb)?;
    Ok(determine_format(vinfo))
}

pub fn buffer_to_fb_format(
    buffer: &Vec<(u8, u8, u8)>,
    format: &FbFormat,
    big_endian: bool,
) -> Vec<u8> {
    let mut raw_buffer = Vec::new();
    for (r, g, b) in buffer {
        match format {
            FbFormat::RGB565 => {
                let mut rgb565: u16 = (*r as u16 & 0b11111000) << 8;
                rgb565 |= (*g as u16 & 0b11111100) << 3;
                rgb565 |= (*b as u16) >> 3;
                if big_endian {
                    raw_buffer.extend(rgb565.to_be_bytes());
                } else {
                    raw_buffer.extend(rgb565.to_le_bytes());
                }
            }
            other => panic!("This display uses a format we haven't implemneted yet {other:?}"),
        }
    }
    raw_buffer
}

pub type CallBack = Box<dyn FnMut(&mut FbInner, &[(u8, u8, u8)]) + Send + 'static>;

pub struct FramebufferDevice {
    data: FbInner,
    pre_write_fn: Option<CallBack>,
    post_write_fn: Option<CallBack>,
}

pub struct FbInner {
    pub fd: File,
    pub dims: Dimensions,
    pub format: FbFormat,
}

impl FramebufferDevice {
    pub fn new(
        path: &str,
        pre_write_fn: Option<CallBack>,
        post_write_fn: Option<CallBack>,
    ) -> Self {
        // This is done as a blocking call to prevent all of the UI init code from having to
        // be made async, making it more verbose. This is a single syscall that would have been
        // done via spawn_blocking anyway, and it's done once on startup.
        let fb = std::fs::File::create(path).expect("Failed to open /dev/fb0");
        let dims = read_fb_dimentions(fb.as_fd()).expect("Failed to read FB dimensions");
        let format = read_fb_format(fb.as_fd())
            .expect("Failed to read FB format")
            .expect("FB retruned unexpected format");
        Self {
            data: FbInner {
                fd: File::from_std(fb),
                dims,
                format,
            },
            pre_write_fn,
            post_write_fn,
        }
    }
}

#[async_trait]
impl GenericFramebuffer for FramebufferDevice {
    fn dimensions(&self) -> Dimensions {
        self.data.dims
    }

    async fn write_buffer(
        &mut self,
        buffer: Vec<(u8, u8, u8)>, // rgb, row-wise, left-to-right, top-to-bottom
    ) {
        if let Some(func) = self.pre_write_fn.as_mut() {
            func(&mut self.data, &buffer);
        }
        let raw_buffer = buffer_to_fb_format(&buffer, &self.data.format, false);
        self.data.fd.write_all(&raw_buffer).await.unwrap();
        self.data.fd.rewind().await.unwrap();
        if let Some(func) = self.post_write_fn.as_mut() {
            func(&mut self.data, &buffer);
        }
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

    task_tracker.spawn(async move {
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
                2 => fb.draw_gif(img.unwrap()).await,
                3 => fb.draw_img(img.unwrap()).await,
                128 => {
                    fb.draw_line(Color::Cyan, 128).await;
                    fb.draw_line(Color::Pink, 102).await;
                    fb.draw_line(Color::White, 76).await;
                    fb.draw_line(Color::Pink, 50).await;
                    fb.draw_line(Color::Cyan, 25).await;
                }
                // this branch id for ui_level 1, which is also the default if an
                // unknown value is used
                _ => {}
            };
            fb.draw_line(display_color, 2).await;
            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    });
}

mod fb_utils {
    use std::io::{Error, Result};
    use std::os::fd::{AsRawFd, BorrowedFd};

    use libc::ioctl;

    use super::FbFormat;

    const FBIOGET_VSCREENINFO: libc::c_ulong = 0x4600;
    // const FBIOGET_FSCREENINFO: libc::c_ulong = 0x4602;

    /// Bitfield which is a part of VarScreeninfo.
    #[repr(C)]
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct Bitfield {
        pub offset: u32,
        pub length: u32,
        pub msb_right: u32,
    }

    /// Struct as defined in /usr/include/linux/fb.h
    #[repr(C)]
    #[derive(Clone, Debug, Default)]
    pub struct VarScreeninfo {
        pub xres: u32,
        pub yres: u32,
        pub xres_virtual: u32,
        pub yres_virtual: u32,
        pub xoffset: u32,
        pub yoffset: u32,
        pub bits_per_pixel: u32,
        pub grayscale: u32,
        pub red: Bitfield,
        pub green: Bitfield,
        pub blue: Bitfield,
        pub transp: Bitfield,
        pub nonstd: u32,
        pub activate: u32,
        pub height: u32,
        pub width: u32,
        pub accel_flags: u32,
        pub pixclock: u32,
        pub left_margin: u32,
        pub right_margin: u32,
        pub upper_margin: u32,
        pub lower_margin: u32,
        pub hsync_len: u32,
        pub vsync_len: u32,
        pub sync: u32,
        pub vmode: u32,
        pub rotate: u32,
        pub colorspace: u32,
        pub reserved: [u32; 4],
    }

    // /// Struct as defined in /usr/include/linux/fb.h
    // /// Note: type is a keyword in Rust and therefore has been changed to fb_type.
    // #[repr(C)]
    // #[derive(Clone, Debug, Default)]
    // pub struct FixScreeninfo {
    //     pub id: [u8; 16],
    //     pub smem_start: usize,
    //     pub smem_len: u32,
    //     pub fb_type: u32,
    //     pub type_aux: u32,
    //     pub visual: u32,
    //     pub xpanstep: u16,
    //     pub ypanstep: u16,
    //     pub ywrapstep: u16,
    //     pub line_length: u32,
    //     pub mmio_start: usize,
    //     pub mmio_len: u32,
    //     pub accel: u32,
    //     pub capabilities: u16,
    //     pub reserved: [u16; 2],
    // }

    // pub fn get_fix_screeninfo(fb: BorrowedFd<'_>) -> Result<FixScreeninfo> {
    //     let mut info: FixScreeninfo = Default::default();
    //     let result = unsafe { ioctl(fb.as_raw_fd(), FBIOGET_FSCREENINFO as _, &mut info) };
    //     match result {
    //         -1 => Err(Error::last_os_error()),
    //         _ => Ok(info),
    //     }
    // }

    pub fn get_var_screeninfo(fb: BorrowedFd<'_>) -> Result<VarScreeninfo> {
        let mut info: VarScreeninfo = Default::default();
        let result = unsafe { ioctl(fb.as_raw_fd(), FBIOGET_VSCREENINFO as _, &mut info) };
        match result {
            -1 => Err(Error::last_os_error()),
            _ => Ok(info),
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    struct RgbaBitfield {
        red: Bitfield,
        green: Bitfield,
        blue: Bitfield,
        transp: Bitfield,
    }

    impl From<&VarScreeninfo> for RgbaBitfield {
        fn from(value: &VarScreeninfo) -> Self {
            Self {
                red: value.red.clone(),
                green: value.green.clone(),
                blue: value.blue.clone(),
                transp: value.transp.clone(),
            }
        }
    }

    type BitfieldShort = (u32, u32);
    type FbInfoShort = (BitfieldShort, BitfieldShort, BitfieldShort, BitfieldShort);

    const fn tuple_to_bitfield(v: BitfieldShort) -> Bitfield {
        let (offset, length) = v;
        // None of formats we support have msb_right set.
        Bitfield {
            offset,
            length,
            msb_right: 0,
        }
    }

    /// Takes a tuple of 4 tuples `(r, g, b, a)`. Each color tuple is a tuple of `(offset, length)`.
    const fn rgba_bitfield(v: FbInfoShort) -> RgbaBitfield {
        let (r, g, b, a) = v;
        RgbaBitfield {
            red: tuple_to_bitfield(r),
            green: tuple_to_bitfield(g),
            blue: tuple_to_bitfield(b),
            transp: tuple_to_bitfield(a),
        }
    }

    // Logic borrowed from QT https://github.com/qt/qtbase/blob/498ae026e98ed181d1480fe5f6f2f1453a725e78/src/plugins/platforms/linuxfb/qlinuxfbscreen.cpp

    const ARGB888: RgbaBitfield = rgba_bitfield(((16, 8), (8, 8), (0, 8), (24, 8)));
    const ABGR888: RgbaBitfield = rgba_bitfield(((0, 8), (8, 8), (16, 8), (24, 8)));
    const RGB888: RgbaBitfield = rgba_bitfield(((16, 8), (8, 8), (0, 8), (0, 0)));
    const BGR888: RgbaBitfield = rgba_bitfield(((0, 8), (8, 8), (16, 8), (0, 0)));
    const RGB666: RgbaBitfield = rgba_bitfield(((12, 6), (6, 6), (0, 6), (0, 0)));
    const RGB565: RgbaBitfield = rgba_bitfield(((11, 5), (5, 6), (0, 5), (0, 0)));
    const BGR565: RgbaBitfield = rgba_bitfield(((0, 5), (5, 6), (11, 5), (0, 0)));
    const RGB555: RgbaBitfield = rgba_bitfield(((10, 5), (5, 5), (0, 5), (0, 0)));
    const BGR555: RgbaBitfield = rgba_bitfield(((0, 5), (5, 5), (10, 5), (0, 0)));
    const RGB444: RgbaBitfield = rgba_bitfield(((8, 4), (4, 4), (0, 4), (0, 0)));

    fn determine_depth(vinfo: &VarScreeninfo) -> u32 {
        let depth = vinfo.red.length + vinfo.green.length + vinfo.blue.length;
        match vinfo.bits_per_pixel {
            24 if depth == 0 => 24,
            16 if depth == 0 => 16,
            24 | 16 => depth,
            v => v,
        }
    }

    pub fn determine_format(vinfo: VarScreeninfo) -> Option<FbFormat> {
        let rgba = RgbaBitfield::from(&vinfo);
        let depth = determine_depth(&vinfo);

        match (depth, rgba) {
            (32, ARGB888) => Some(FbFormat::ARGB888),
            (32, ABGR888) => Some(FbFormat::ABGR888),
            (24, RGB888) => Some(FbFormat::RGB888),
            (24, BGR888) => Some(FbFormat::BGR888),
            (18, RGB666) => Some(FbFormat::RGB666),
            (16, RGB565) => Some(FbFormat::RGB565),
            (16, BGR565) => Some(FbFormat::BGR565),
            (15, RGB555) => Some(FbFormat::RGB555),
            (15, BGR555) => Some(FbFormat::BGR555),
            (12, RGB444) => Some(FbFormat::RGB444),
            _ => None,
        }
    }
}
