use crate::rendering::Color;

#[derive(Debug, Default)]
pub struct Material {
    diffuse: Color,
}

pub mod materials {
    use super::*;
    use crate::rendering::colors;

    pub static SOLID_BLUE : Material = Material::from_diffuse(colors::BLUE);
    pub static SOLID_RED : Material = Material::from_diffuse(colors::RED);
}

impl Material {
    pub const fn from_diffuse(diffuse: Color) -> Self {
        Material { diffuse }
    }

    pub fn diffuse_color(&self) -> Color {
        self.diffuse
    }
}
