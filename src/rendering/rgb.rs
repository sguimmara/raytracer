use nameof::name_of_type;
use std::fmt::{Display, Formatter};

/// A simple RGB color
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Display for Rgb {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {} {})",
            name_of_type!(Rgb),
            self.r,
            self.g,
            self.b
        )
    }
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }

    pub fn red() -> Self {
        Rgb { r: 255, g: 0, b: 0 }
    }

    pub fn green() -> Self {
        Rgb { r: 0, g: 255, b: 0 }
    }

    pub fn gray() -> Self {
        Rgb { r: 50, g: 50, b: 50 }
    }
}
