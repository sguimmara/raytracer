pub mod sphere;
pub use sphere::Sphere;
use crate::scene::hittable::Hittable;

pub trait Primitive: Hittable {

}
