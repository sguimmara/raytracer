use crate::rendering::ColorMatrix;
use crate::rendering::Rgb;
use crate::rendering::Transform;
use std::fmt::{Display, Formatter};
use nameof::name_of_type;

pub struct Camera {
    transform: Transform,
    clear_color: Rgb,
}

impl Display for Camera {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (background: {})", name_of_type!(Camera), self.clear_color)
    }
}

impl Camera {
    pub fn new(clear_color: Rgb) -> Self {
        Camera {
            transform: Transform::default(),
            clear_color,
        }
    }

    pub fn render(&self, target: &mut dyn ColorMatrix) {
        target.clear(self.clear_color);

        for y in 0..target.height() {
            self.render_scanline(y, target);
        }
    }

    /// Renders a single scanline
    fn render_scanline(&self, row: u32, target: &mut dyn ColorMatrix) {
        for col in 0..target.width() {
            self.render_pixel((col, row), target);
        }
    }

    /// Render a single pixel
    fn render_pixel(&self, pixel: (u32, u32), target: &mut dyn ColorMatrix) {
        let y = pixel.1;
        let color = match y % 2 {
            0 => Rgb::red(),
            _ => Rgb::green()
        };
        target.set(pixel.0, pixel.1, color);
    }
}
