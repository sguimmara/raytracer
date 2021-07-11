use crate::math::Ray;
use std::fmt::Debug;

use crate::math::Vec3;
use crate::scene::Transform;

/// Contains information about the interaction between a [Ray] and a [Hittable].
#[derive(Debug)]
pub struct Hit {
    /// the world-space position of the intersection
    pub position: Vec3,
    /// the normal of the intersected geometry
    pub normal: Vec3,
}

impl Hit {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self { position, normal }
    }
}

/// A trait for objects than can interact with [Ray]s.
pub trait Hittable: Debug {
    fn test(&self, ray: &Ray, transform: &Transform) -> Option<Hit>;
}
