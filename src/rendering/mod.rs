use std::fmt::{Display, Formatter};

use nameof::name_of_type;

pub use framebuffer::{FrameBuffer, RenderTarget};
pub use material::{Material};

pub use crate::scene::camera::Camera;
use std::ops::{Add, AddAssign};

pub mod backends;
pub mod framebuffer;
pub mod material;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RenderOpts {
    pub samples: Sampling,
}

impl RenderOpts {
    pub fn new() -> Self {
        RenderOpts { samples: Sampling::Disabled }
    }

    pub fn with_samples(self, samples: Sampling) -> Self {
        let mut s = self;
        s.samples = samples;
        s
    }
}


#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sampling {
    // disabled: only 1 sample per pixel is computed
    Disabled,
    // 2x2 samples per pixel
    Samples4,
    // 4x4 samples per pixel
    Samples16,
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct PixelSize {
    pub width : u32,
    pub height: u32,
}

impl PixelSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
}

impl Pixel {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct SubPixel {
    pub x: f32,
    pub y: f32,
}

impl SubPixel {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn with_offset(self, x: f32, y: f32) -> Self {
        Self::new(self.x + x, self.y + y)
    }
}

impl From<Pixel> for SubPixel {
    fn from(p: Pixel) -> Self {
        Self::new(p.x as f32, p.y as f32)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct HdrColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub samples: u32,
}

impl AddAssign<Color> for HdrColor {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r as f64;
        self.g += rhs.g as f64;
        self.b += rhs.b as f64;
        self.samples += 1;
    }
}

impl Add<Color> for HdrColor {
    type Output = HdrColor;

    fn add(self, rhs: Color) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r as f64,
            g: self.g + rhs.g as f64,
            b: self.b + rhs.b as f64,
            samples: self.samples + 1,
        }
    }
}

impl From<HdrColor> for Color {
    fn from(hdr: HdrColor) -> Self {
        let r = hdr.r / hdr.samples as f64;
        let g = hdr.g / hdr.samples as f64;
        let b = hdr.b / hdr.samples as f64;

        Self::new(r as u8, g as u8, b as u8)
    }
}

/// A simple RGB color
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
pub mod colors {
    use super::Color;

    pub static BLACK: Color = Color::new(0, 0, 0);
    pub static WHITE: Color = Color::new(255, 255, 255);
    pub static RED: Color = Color::new(255, 0, 0);
    pub static BLUE: Color = Color::new(0, 0, 255);
    pub static GREEN: Color = Color::new(0, 255, 0);
    pub static GRAY: Color = Color::new(100, 100, 100);
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {} {})",
            name_of_type!(Color),
            self.r,
            self.g,
            self.b
        )
    }
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }
}
