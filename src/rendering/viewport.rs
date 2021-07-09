use crate::math::ray::Ray;
use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct Viewport {
    origin: Vec3,
    pixel_height: u32,
    pixel_width: u32,
    height: f32,
    width: f32,
    near_plane: f32,
}

impl Viewport {
    pub fn new(origin: Vec3, pixel_height: u32, pixel_width: u32) -> Self {
        assert!(pixel_height > 0);
        assert!(pixel_width > 0);
        Viewport {
            origin,
            pixel_height,
            pixel_width,
            width: 2.0,
            height: 2.0,
            near_plane: 1.0,
        }
    }

    fn uv(&self, x: u32, y: u32) -> (f32, f32) {
        (
            x as f32 / (self.pixel_width - 1) as f32,
            y as f32 / (self.pixel_height - 1) as f32,
        )
    }

    pub fn primary_ray(&self, x: u32, y: u32) -> Ray {
        let horiz = Vec3::new(self.width, 0f32, 0f32);
        let vert = Vec3::new(0f32, self.height, 0f32);
        let (u, v) = self.uv(x, y);
        let fwd = Vec3::new(0.0, 0.0, self.near_plane);
        let ll = self.origin - horiz / 2.0 - vert / 2.0 - fwd;
        Ray::new(self.origin, ll + horiz * u + vert * v - self.origin)
    }
}
