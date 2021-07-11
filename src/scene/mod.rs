use std::fmt::Debug;

use crate::math::Vec3;
use crate::rendering::{colors, Camera, RenderTarget, RenderOpts, Material};

pub mod camera;
pub mod entity;
pub mod hit;
pub mod hittable;
pub mod primitives;
pub mod transform;

use crate::scene::entity::Entity;
pub use hittable::{Hittable, Hit};
pub use primitives::{Primitive};
pub use transform::Transform;
use crate::scene::primitives::Sphere;

#[derive(Debug)]
pub struct Scene {
    entities: Vec<Entity>,
    camera: Camera,
}

impl Scene {
    pub fn new() -> Self {
        let sphere = Box::new(Sphere::new(1.0));
        let material = Material::from_diffuse(colors::GREEN);
        let entity = Entity::new(Transform::default(), material, sphere);
        let entities = vec![entity];
        let mut camera = Camera::new().with_clear_color(colors::BLUE);
        camera.transform().set_position(Vec3::new(0.0, 0.0, -2.0));

        Scene { entities, camera }
    }

    pub fn render(&self, target: &mut dyn RenderTarget, opts: &RenderOpts, progress_func: &dyn Fn(f32)) {
        self.camera.render(self, target, opts, progress_func);
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}
