use crate::math::{Ray, Vec3};
use crate::scene::{Transform, Hit, Primitive};
use crate::rendering::Material;

#[derive(Debug)]
pub struct Sphere {
    radius: f32,
    sqr_radius: f32,
}

impl Clone for Sphere {
    fn clone(&self) -> Self {
        Self::new(self.radius)
    }
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Sphere {
            radius,
            sqr_radius: radius * radius,
        }
    }
}

impl Primitive for Sphere {
    fn hit(&self, ray: &Ray, transform: &Transform, material: Material) -> Option<Hit> {
        let center = transform.position();
        let oc = ray.origin() - center;
        let a = ray.direction().magnitude_squared();
        let half_b = Vec3::dot(&oc, &ray.direction());
        let c = &oc.magnitude_squared() - self.sqr_radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        const T_MIN : f32 = 0.0;
        const T_MAX : f32 = 99999999.0;

        if root < T_MIN || T_MAX < root {
            root = (-half_b + sqrtd) / a;
            if root < T_MIN || T_MAX < root {
                return None
            }
        }

        let point = ray.at(root);
        let normal = (point - center) / self.radius;
        let sqr_dist = root;
        let hit = Hit::new(point, normal, sqr_dist, material);

        Some(hit)
    }
}
