use crate::math::{Ray, Vec3};
use crate::rendering::RenderTarget;
use crate::rendering::Rgb;
use crate::scene::{Scene, Transform};
use nameof::name_of_type;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Camera {
    transform: Transform,
    clear_color: Rgb,
    focal_length: f32,
    width: f32,
    height: f32,
}

impl Display for Camera {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (background: {})",
            name_of_type!(Camera),
            self.clear_color
        )
    }
}

impl Camera {
    pub fn new(clear_color: Rgb) -> Self {
        Camera {
            transform: Transform::default(),
            clear_color,
            focal_length: 1f32,
            width: 2f32,
            height: 2f32,
        }
    }

    pub fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn render(&self, scene: &Scene, target: &mut dyn RenderTarget) {
        target.clear(self.clear_color);

        for y in 0..target.height() {
            self.render_scanline(y, &scene, target);
        }
    }

    /// Renders a single scanline
    fn render_scanline(&self, row: u32, scene: &Scene, target: &mut dyn RenderTarget) {
        for col in 0..target.width() {
            self.render_pixel((col, row), scene, target);
        }
    }

    /// Render a single pixel
    fn render_pixel(&self, pixel: (u32, u32), scene: &Scene, target: &mut dyn RenderTarget) {
        let uv = self.uv(pixel.0, pixel.1, target.width(), target.height());
        let ray = self.pixel_to_ray(uv);

        for entity in &scene.entities {
            match entity.raytrace(&ray) {
                Some(hit) =>         target.set(pixel.0, pixel.1, Rgb::red()),
                _ => {}
            }
        }
    }

    fn uv(&self, x: u32, y: u32, pixel_width: u32, pixel_height: u32) -> (f32, f32) {
        (
            x as f32 / (pixel_width - 1) as f32,
            y as f32 / (pixel_height - 1) as f32,
        )
    }

    fn pixel_to_ray(&self, uv: (f32, f32)) -> Ray {
        let horiz = Vec3::new(self.width, 0f32, 0f32);
        let vert = Vec3::new(0f32, self.height, 0f32);
        let fwd = Vec3::new(0.0, 0.0, self.focal_length);
        let origin = self.transform.position();
        let ll = origin - horiz / 2.0 - vert / 2.0 - fwd;

        Ray::new(origin, ll + horiz * uv.0 + vert * uv.1 - origin)
    }
}
