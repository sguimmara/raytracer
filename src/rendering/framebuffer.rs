use nameof::name_of_type;
use std::fmt::{Display, Formatter};
use crate::rendering::{Color, Pixel, PixelSize};

/// Types that can be rendered into.
pub trait RenderTarget {
    /// Gets the underlying bytes.
    fn bytes(&self) -> &[u8];

    /// Gets the size of this [RenderTarget]
    fn size(&self) -> PixelSize;

    /// Clears the [RenderTarget] with the specified [Color] value.
    fn clear(&mut self, value: Color);

    /// Sets the [Pixel] with the specified [Color] value.
    fn set(&mut self, pixel: Pixel, value: Color);

    /// Gets the color of the [Pixel].
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
            self.size.width,
            self.size.height
        )
    }
}

impl RenderTarget for FrameBuffer {
    /// Returns a view of the underlying bytes.
    fn bytes(&self) -> &[u8] {
        &self.pixels
    }

    fn size(&self) -> PixelSize {
        self.size
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
    use crate::rendering::RED;

    #[test]
    fn new() {
        let buffer = FrameBuffer::new(10, 15);
        assert_eq!(10, buffer.size().width);
        assert_eq!(15, buffer.size().height);
        assert_eq!(10 * 15 * 3, buffer.pixels.len());
    }

    #[test]
    fn clear() {
        let mut buffer = FrameBuffer::new(5, 9);

        buffer.clear(RED);

        for x in 0..buffer.size().width {
            for y in 0..buffer.size().height {
                assert_eq!(RED, buffer.get(Pixel::new(x, y)));
            }
        }
    }
}
