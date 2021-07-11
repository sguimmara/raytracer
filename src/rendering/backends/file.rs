use crate::rendering::backends::Backend;
use crate::rendering::framebuffer::RenderTarget;
use image::codecs::png::PngEncoder;
use image::ColorType;
use nameof::name_of_type;
use std::fmt::{Display, Formatter};
use std::fs;

/// A [Backend] that writes the [FrameBuffer] into an image file.
pub struct FileBackend<'a> {
    filename: &'a str,
}

impl<'a> Display for FileBackend<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", name_of_type!(FileBackend), self.filename)
    }
}

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
            .encode(buf.as_bytes(), buf.width(), buf.height(), ColorType::Rgb8)
            .unwrap();
    }
}
