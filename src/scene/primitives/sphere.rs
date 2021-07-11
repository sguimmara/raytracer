use crate::math::{Ray, Vec3};
use crate::scene::{Transform, Hittable, Hit};

#[derive(Debug)]
pub struct Sphere {
    radius: f32,
    sqr_radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Sphere {
            radius,
            sqr_radius: radius * radius,
        }
    }
}

impl Hittable for Sphere {
    fn test(&self, ray: &Ray, transform: &Transform) -> Option<Hit> {
        let center = transform.position();
        let oc = ray.origin() - center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = 2.0 * Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.sqr_radius;
        let discriminant = b * b - 4f32 * a * c;

        if discriminant < 0f32 {
            return None;
        }

        let distance = (-b - discriminant.sqrt()) / (2.0 * a);

        let hit_point = ray.at(distance);
        let hit_normal = Vec3::normalize(&(hit_point - center));

        let hit = Hit::new(hit_point, hit_normal);

        Some(hit)
    }
}
