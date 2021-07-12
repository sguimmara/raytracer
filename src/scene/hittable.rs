use crate::math::Ray;
use std::fmt::Debug;

use crate::math::Vec3;
use crate::rendering::Material;

/// Contains information about the interaction between a [Ray] and a [Hittable].
#[derive(Debug)]
pub struct Hit {
    /// the world-space position of the intersection
    position: Vec3,
    /// the normal of the intersected geometry
    normal: Vec3,
    /// The squared distance to the ray origin
    sqr_distance: f32,
    material: Material
}

impl Hit {
    pub fn new(position: Vec3, normal: Vec3, sqr_distance: f32, material: Material) -> Self {
        Self {
            position,
            normal,
            sqr_distance,
            material
        }
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn sqr_distance(&self) -> f32 {
        self.sqr_distance
    }
    pub fn material(&self) -> &Material {
        &self.material
    }
}

/// A trait for objects than can interact with [Ray]s.
pub trait Hittable: Debug {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}
