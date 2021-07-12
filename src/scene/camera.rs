use crate::math::{Ray, Vec3};
use crate::rendering::{Color, Sample, Pixel, PixelSize, RenderOpts, RenderTarget, Sampling, SubPixel, BLACK};
use crate::scene::{Scene, Transform, Hittable};
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
        let height = 1.0;
        let width = aspect * height;

        Camera {
            transform: Transform::default(),
            clear_color: BLACK,
            focal_length: 1f32,
            aspect,
            width,
            height,
        }
    }

    /// Sets the clear color
    pub fn with_clear_color(self, color: Color) -> Self {
        let mut new = self;
        new.clear_color = color;
        new
    }

    /// Gets the [Transform] associated with this [Camera]
    pub fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    /// Gets the aspect ratio
    pub fn aspect(&self) -> f32 {
        self.aspect
    }

    pub fn render(
        &self,
        scene: &Scene,
        target: &mut dyn RenderTarget,
        opts: &RenderOpts,
        progress_func: &dyn Fn(f32),
    ) {
        target.clear(self.clear_color);

        log::info!("rendering...");
        let mut progress: f32 = 0f32;

        let render_func = match opts.samples {
                Sampling::Disabled => Self::render_pixel_1_sample,
                Sampling::Samples4 => Self::render_pixel_4_samples,
                Sampling::Samples16 => Self::render_pixel_16_samples,
            };

        for y in 0..target.size().height {
            self.render_scanline(y, &scene, target, render_func);

            progress += 1f32 / (target.size().height as f32);
            progress_func(progress);
        }
    }

    /// Renders a single scanline
    fn render_scanline(
        &self,
        row: u32,
        scene: &Scene,
        target: &mut dyn RenderTarget,
        render_func: fn(&Camera, Pixel, &Scene, PixelSize, &mut Sample)
    ) {
        for col in 0..target.size().width {
            self.render_pixel(Pixel::new(col, row), scene, target, render_func);
        }
    }

    /// Render a single pixel
    fn render_pixel(
        &self,
        pixel: Pixel,
        scene: &Scene,
        target: &mut dyn RenderTarget,
        render_func: fn(&Camera, Pixel, &Scene, PixelSize, &mut Sample)
    ) {
        let mut hdr = Sample::default();

        render_func(&self, pixel, scene, target.size(), &mut hdr);

        let color = Color::from(hdr);

        target.set(pixel, color);
    }

    /// Render a single pixel
    fn render_pixel_1_sample(
        &self,
        pixel: Pixel,
        scene: &Scene,
        size: PixelSize,
        hdr: &mut Sample,
    ) {
        let center = SubPixel::from(pixel);

        self.sample(center, scene, size, hdr);
    }

    /// Render a single pixel with 4 samples (2*2)
    fn render_pixel_4_samples(
        &self,
        pixel: Pixel,
        scene: &Scene,
        size: PixelSize,
        hdr: &mut Sample,
    ) {
        // The pixel is divided into multiple samples in the following pattern
        // +--------+
        // | ul  ur |
        // | ll  lr |
        // +--------+
        let center = SubPixel::from(pixel);
        const SUB_OFFSET: f32 = 0.25;
        let ur = center.with_offset(SUB_OFFSET, SUB_OFFSET);
        let ll = center.with_offset(-SUB_OFFSET, -SUB_OFFSET);
        let lr = center.with_offset(SUB_OFFSET, -SUB_OFFSET);
        let ul = center.with_offset(-SUB_OFFSET, SUB_OFFSET);

        self.sample(ur, scene, size, hdr);
        self.sample(ul, scene, size, hdr);
        self.sample(lr, scene, size, hdr);
        self.sample(ll, scene, size, hdr);
    }

    /// Render a single pixel with 16 samples (4*4)
    fn render_pixel_16_samples(
        &self,
        pixel: Pixel,
        scene: &Scene,
        size: PixelSize,
        hdr: &mut Sample,
    ) {
        // The pixel is divided into multiple samples in the following pattern
        // +-----------------+
        // | r00 r10 r20 r30 |
        // | r01 r11 r21 r31 |
        // | r02 r12 r22 r32 |
        // | r03 r13 r23 r33 |
        // +-----------------+
        let center = SubPixel::from(pixel);
        const SUB_OFFSET: f32 = 0.125;
        let r00 = center.with_offset(-SUB_OFFSET*2.0, SUB_OFFSET*2.0);
        let r10 = center.with_offset(-SUB_OFFSET*1.0, SUB_OFFSET*2.0);
        let r20 = center.with_offset(SUB_OFFSET*1.0, SUB_OFFSET*2.0);
        let r30 = center.with_offset(SUB_OFFSET*2.0, SUB_OFFSET*2.0);

        let r01 = center.with_offset(-SUB_OFFSET*2.0, SUB_OFFSET*1.0);
        let r11 = center.with_offset(-SUB_OFFSET*1.0, SUB_OFFSET*1.0);
        let r21 = center.with_offset(SUB_OFFSET*1.0, SUB_OFFSET*1.0);
        let r31 = center.with_offset(SUB_OFFSET*2.0, SUB_OFFSET*1.0);

        let r02 = center.with_offset(-SUB_OFFSET*2.0, -SUB_OFFSET*1.0);
        let r12 = center.with_offset(-SUB_OFFSET*1.0, -SUB_OFFSET*1.0);
        let r22 = center.with_offset(SUB_OFFSET*1.0, -SUB_OFFSET*1.0);
        let r32 = center.with_offset(SUB_OFFSET*2.0, -SUB_OFFSET*1.0);

        let r03 = center.with_offset(-SUB_OFFSET*2.0, -SUB_OFFSET*2.0);
        let r13 = center.with_offset(-SUB_OFFSET*1.0, -SUB_OFFSET*2.0);
        let r23 = center.with_offset(SUB_OFFSET*1.0, -SUB_OFFSET*2.0);
        let r33 = center.with_offset(SUB_OFFSET*2.0, -SUB_OFFSET*2.0);

        self.sample(r00, scene, size, hdr);
        self.sample(r10, scene, size, hdr);
        self.sample(r20, scene, size, hdr);
        self.sample(r30, scene, size, hdr);

        self.sample(r01, scene, size, hdr);
        self.sample(r11, scene, size, hdr);
        self.sample(r21, scene, size, hdr);
        self.sample(r31, scene, size, hdr);

        self.sample(r02, scene, size, hdr);
        self.sample(r12, scene, size, hdr);
        self.sample(r22, scene, size, hdr);
        self.sample(r32, scene, size, hdr);

        self.sample(r03, scene, size, hdr);
        self.sample(r13, scene, size, hdr);
        self.sample(r23, scene, size, hdr);
        self.sample(r33, scene, size, hdr);
    }

    fn sample(&self, subpix: SubPixel, scene: &Scene, size: PixelSize, hdr: &mut Sample) {
        let uv = self.uv(subpix, size);
        let ray = self.pixel_to_ray(uv);

        *hdr += self.raytrace(scene, &ray);
    }

    fn raytrace(&self, scene: &Scene, ray: &Ray) -> Color {
        match &scene.hit(ray) {
            Some(hit) => hit.material().diffuse_color(),
            None => self.clear_color
        }
    }

    fn uv(&self, pixel: SubPixel, size: PixelSize) -> (f32, f32) {
        let u = pixel.x / (size.width - 1) as f32;
        let v = pixel.y / (size.height - 1) as f32;

        // the viewport is Y-up, but the framebuffer is Y-down
        (u, 1.0-v)
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
