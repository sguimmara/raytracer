use crate::math::{Ray, Vec3};
use crate::rendering::Material;
use crate::scene::{Hittable, Transform, Hit};

#[derive(Debug)]
pub struct Entity<'a> {
    transform: Transform,
    material: &'a Material,
    renderer: Box<dyn Hittable>,
}

impl<'a> Entity<'a> {
    pub fn new(transform: Transform, material: &'a Material, renderer: Box<dyn Hittable>) -> Self {
        Self {
            transform,
            material,
            renderer,
        }
    }

    pub fn raytrace(&self, ray: &Ray) -> Option<RaytraceInfo> {
        match self.renderer.test(&ray, &self.transform) {
            Some(Hit { position, normal }) => {
                Some(RaytraceInfo::new(position, normal, &self.material))
            }
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct RaytraceInfo<'a> {
    position: Vec3,
    normal: Vec3,
    material: &'a Material,
}

impl<'a> RaytraceInfo<'a> {
    pub fn new(position: Vec3, normal: Vec3, material: &'a Material) -> Self {
        Self {
            position,
            normal,
            material,
        }
    }

    pub fn material(&self) -> &Material {
        self.material
    }
}
