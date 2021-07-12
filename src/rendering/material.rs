use crate::rendering::Color;

#[derive(Debug, Default)]
pub struct Material {
    diffuse: Color,
}

impl Material {
    pub const fn from_diffuse(diffuse: Color) -> Self {
        Material { diffuse }
    }

    pub fn diffuse_color(&self) -> Color {
        self.diffuse
    }
}

impl Clone for Material {
    fn clone(&self) -> Self {
        Self {
            diffuse: self.diffuse
        }
    }
}
