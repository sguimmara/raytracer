use crate::math::{Ray};
use crate::rendering::Material;
use crate::scene::{Hittable, Transform, Hit, Primitive};

#[derive(Debug)]
pub struct Entity {
    transform: Transform,
    material: Material,
    renderer: Box<dyn Primitive>,
}

impl Entity {
    pub fn new(transform: Transform, material: Material, renderer: Box<dyn Primitive>) -> Self {
        Self {
            transform,
            material,
            renderer,
        }
    }
}

impl Hittable for Entity {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.renderer.hit(&ray, &self.transform, self.material.clone())
    }
}