use crate::math::{Ray, Vec3};
use crate::rendering::{RenderTarget, Color, colors, Pixel};
use crate::scene::{Scene, Transform};
use nameof::name_of_type;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Camera {
    transform: Transform,
    clear_color: Color,
    focal_length: f32,
    aspect: f32,
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
    pub fn new() -> Self {
        let aspect = 16.0 / 9.0;
        let height = 2.0;
        let width = aspect * height;

        Camera {
            transform: Transform::default(),
            clear_color: colors::BLACK,
            focal_length: 1f32,
            aspect,
            width,
            height,
        }
    }

    pub fn with_clear_color(self, color: Color) -> Self {
        let mut new = self;
        new.clear_color = color;
        new
    }

    pub fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
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
            self.render_pixel(Pixel::new(col, row), scene, target);
        }
    }

    /// Render a single pixel
    fn render_pixel(&self, pixel: Pixel, scene: &Scene, target: &mut dyn RenderTarget) {
        let uv = self.uv(pixel, target.width(), target.height());
        let ray = self.pixel_to_ray(uv);

        for entity in &scene.entities {
            match entity.raytrace(&ray) {
                Some(hit) => target.set(pixel, hit.material().diffuse_color()),
                _ => {}
            }
        }
    }

    fn uv(&self, pixel: Pixel, pixel_width: u32, pixel_height: u32) -> (f32, f32) {
        (
            pixel.x as f32 / (pixel_width - 1) as f32,
            pixel.y as f32 / (pixel_height - 1) as f32,
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
