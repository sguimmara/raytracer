use std::fmt::Debug;

use crate::math::Vec3;
use crate::rendering::{colors, materials, Camera, RenderTarget, RenderOpts};

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
pub struct Scene<'a> {
    entities: Vec<Entity<'a>>,
    camera: Camera,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        let sphere = Box::new(Sphere::new(1.0));
        let material = &materials::SOLID_RED;
        let entity = Entity::new(Transform::default(), material, sphere);
        let entities = vec![entity];
        let mut camera = Camera::new().with_clear_color(colors::BLUE);
        camera.transform().set_position(Vec3::new(0.0, 0.0, -2.0));

        Scene { entities, camera }
    }

    pub fn render(&self, target: &mut dyn RenderTarget, opts: &RenderOpts) {
        self.camera.render(self, target, opts);
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }
}
