use image::codecs::png::PngEncoder;
use image::ColorType;
use std::fs;
use crate::rendering::framebuffer::{ColorMatrix};
use crate::rendering::backends::Backend;
use std::fmt::{Display, Formatter};
use nameof::name_of_type;

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
    fn write(&self, buf: &dyn ColorMatrix) {
        let stream = fs::File::create(self.filename).expect("could not create file");

        let encoder = PngEncoder::new(stream);
        encoder
            .encode(buf.as_bytes(), buf.width(), buf.height(), ColorType::Rgb8)
            .unwrap();
    }
}
