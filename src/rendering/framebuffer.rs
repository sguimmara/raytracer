use nameof::name_of_type;
use std::fmt::{Display, Formatter};
use crate::rendering::{colors, Color, Pixel, PixelSize};

/// A [RenderTarget] is a matrix of pixels that were rendered.
pub trait RenderTarget {
    /// Gets the underlying bytes.
    fn as_bytes(&self) -> &[u8];

    fn size(&self) -> PixelSize;

    /// Gets the width of the [RenderTarget], in pixels.
    fn width(&self) -> u32;

    /// Gets the height of the [RenderTarget], in pixels.
    fn height(&self) -> u32;

    /// Clears the [RenderTarget] with the specified [Rgb] value.
    fn clear(&mut self, value: Color);

    /// Sets the pixel (x, y) with the specified [Rgb] value.
    fn set(&mut self, pixel: Pixel, value: Color);

    /// Gets the pixel (x, y).
    fn get(&self, pixel: Pixel) -> Color;
}

#[derive(Debug)]
pub struct FrameBuffer {
    size: PixelSize,
    pixels: Vec<u8>,
}

const R_OFFSET: usize = 0;
const G_OFFSET: usize = 1;
const B_OFFSET: usize = 2;

impl Display for FrameBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}*{} RGB)",
            name_of_type!(FrameBuffer),
            self.width(),
            self.height()
        )
    }
}

impl RenderTarget for FrameBuffer {
    /// Returns a view of the underlying bytes.
    fn as_bytes(&self) -> &[u8] {
        &self.pixels
    }

    fn size(&self) -> PixelSize {
        self.size
    }

    fn width(&self) -> u32 {
        self.size.width
    }

    fn height(&self) -> u32 {
        self.size.height
    }

    fn clear(&mut self, value: Color) {
        for x in 0..self.size.width {
            for y in 0..self.size.height {
                self.set(Pixel::new(x, y), value);
            }
        }
    }

    fn set(&mut self, pixel: Pixel, value: Color) {
        let offset = self.offset(pixel);

        self.pixels[offset + R_OFFSET] = value.r;
        self.pixels[offset + G_OFFSET] = value.g;
        self.pixels[offset + B_OFFSET] = value.b;
    }

    fn get(&self, pixel: Pixel) -> Color {
        let offset = self.offset(pixel);

        let r = self.pixels[offset + R_OFFSET];
        let g = self.pixels[offset + G_OFFSET];
        let b = self.pixels[offset + B_OFFSET];

        Color::new(r, g, b)
    }
}

impl FrameBuffer {
    /// Constructs a [FrameBuffer] with the specified pixel size
    pub fn new(width: u32, height: u32) -> Self {
        assert!(width > 0);
        assert!(height > 0);

        let byte_count = width * height * 3;
        let mut pixels = Vec::with_capacity((byte_count) as usize);
        let r = Color::default().r;
        let g = Color::default().g;
        let b = Color::default().b;
        for _ in 0..(width * height) {
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
        }

        FrameBuffer {
            size: PixelSize::new(width, height),
            pixels,
        }
    }

    fn offset(&self, p: Pixel) -> usize {
        ((3 * p.x) + p.y * self.size.width * 3) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let buffer = FrameBuffer::new(10, 15);
        assert_eq!(10, buffer.width());
        assert_eq!(15, buffer.height());
        assert_eq!(10 * 15 * 3, buffer.pixels.len());
    }

    #[test]
    fn clear() {
        let mut buffer = FrameBuffer::new(5, 9);

        buffer.clear(colors::RED);

        for x in 0..buffer.width() {
            for y in 0..buffer.height() {
                assert_eq!(colors::RED, buffer.get(Pixel::new(x, y)));
            }
        }
    }
}
