use std::fmt::{Display, Formatter};

use nameof::name_of_type;

pub use framebuffer::{FrameBuffer, RenderTarget};
pub use material::{Material, materials};

pub use crate::scene::camera::Camera;

pub mod backends;
pub mod framebuffer;
pub mod material;

/// A simple RGB color
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

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
