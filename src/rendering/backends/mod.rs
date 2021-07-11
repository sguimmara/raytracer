use crate::rendering::RenderTarget;
use image::codecs::png::PngEncoder;
use image::ColorType;
use nameof::name_of_type;
use std::fmt::{Display, Formatter};
use std::fs;
use minifb::{WindowOptions, Window, Key};

/// Trait for types that can present a [RenderTarget]
pub trait Backend {
    /// Presents the [RenderTarget].
    fn present(&self, buf: &dyn RenderTarget);
}

/// A [Backend] that discards the [RenderTarget]
#[derive(Debug, Default)]
pub struct NullBackend {}

impl Display for NullBackend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

impl Backend for NullBackend {
    fn present(&self, _: &dyn RenderTarget) {
        // do nothing
    }
}

/// A [Backend] that writes the [RenderTarget] into an image file.
pub struct FileBackend<'a> {
    filename: &'a str,
}

impl<'a> Display for FileBackend<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", name_of_type!(FileBackend), self.filename)
    }
}

#[allow(dead_code)]
impl<'a> FileBackend<'a> {
    pub fn new(filename: &'a str) -> Self {
        FileBackend { filename }
    }
}

impl<'a> Backend for FileBackend<'a> {
    fn present(&self, buf: &dyn RenderTarget) {
        let stream = fs::File::create(self.filename).expect("could not create file");

        let encoder = PngEncoder::new(stream);
        encoder
            .encode(
                buf.bytes(),
                buf.size().width,
                buf.size().height,
                ColorType::Rgb8,
            )
            .unwrap();
    }
}

/// A [Backend] that renders into a window.
pub struct WindowBackend {}

impl Backend for WindowBackend {
    fn present(&self, buf: &dyn RenderTarget) {
        let pixel_count = (buf.size().width * buf.size().height) as usize;

        let mut window = Window::new(
            "raytracer - Press 'Esc' to exit",
            buf.size().width as usize,
            buf.size().height as usize,
            WindowOptions::default(),
        )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            });

        let mut output_buf = vec![0; pixel_count * 4];

        const R_OFFSET : usize = 0;
        const G_OFFSET : usize = 1;
        const B_OFFSET : usize = 2;

        let input = buf.bytes();

        for i in 0..pixel_count {
            let offset = (i * 3) as usize;
            let r = input[offset + R_OFFSET] as u32;
            let g = input[offset + G_OFFSET] as u32;
            let b = input[offset + B_OFFSET] as u32;

            // weird pattern 0RGB
            output_buf[i as usize] = r << 16 | g << 8 | b << 0;
        }

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        window
            .update_with_buffer(
                &output_buf,
                buf.size().width as usize,
                buf.size().height as usize,
            )
            .unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update();
        }
    }
}

#[allow(dead_code)]
impl WindowBackend {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for WindowBackend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WindowBackend")
    }
}
