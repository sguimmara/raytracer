pub mod sphere;
pub use sphere::Sphere;
use crate::scene::{Transform, Hit};
use crate::math::Ray;
use crate::rendering::Material;
use std::fmt::Debug;

pub trait Primitive : Debug {
    fn hit(&self, ray: &Ray, transform: &Transform, material: Material) -> Option<Hit>;
}
