use std::fmt::Debug;

use crate::math::{Ray, Vec3};
use crate::rendering::{Camera, Rgb, RenderTarget};

pub mod camera;

#[derive(Debug, Default)]
pub struct Transform {
    position: Vec3,
}

impl Transform {
    pub fn position(&self) -> Vec3 {
        self.position
    }
}

#[derive(Debug)]
pub struct Scene {
    entities: Vec<Entity>,
    camera: Camera
}

impl Scene {
    pub fn new() -> Self {
        let sphere = Box::new(Sphere::new(1.0));
        let entity = Entity::new(Transform::default(), sphere);
        let entities = vec![entity];
        let mut camera = Camera::new(Rgb::gray());
        camera.transform().position = Vec3::new(0.0,0.0 ,-2.0);

        Scene { entities, camera }
    }

    pub fn render(&self, target: &mut dyn RenderTarget) {
        self.camera.render(self, target)
    }
}

#[derive(Debug)]
pub struct Entity {
    transform: Transform,
    renderer: Box<dyn Renderer>,
}

impl Entity {
    pub fn new(transform: Transform, renderer: Box<dyn Renderer>) -> Self {
        Self { transform, renderer }
    }

    pub fn raytrace(&self, ray: &Ray) -> Option<Hit> {
        self.renderer.test(&ray, &self.transform)
    }
}

/// Contains information about the interaction between a [Ray] and a [Renderer].
#[derive(Debug, Default)]
pub struct Hit {
    /// the world-space position of the intersection
    position: Vec3,
    /// the normal of the intersected geometry
    normal: Vec3,
}

impl Hit {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self { position, normal }
    }
}

pub trait Renderer: Debug {
    fn test(&self, ray: &Ray, transform: &Transform) -> Option<Hit>;
}

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

impl Renderer for Sphere {
    fn test(&self, ray: &Ray, transform: &Transform) -> Option<Hit> {
        let center = transform.position;
        let oc = ray.origin() - center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = 2.0 * Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.sqr_radius;
        let discriminant = b * b - 4f32 * a * c;

        let is_hit = discriminant > 0f32;

        return match is_hit {
            true => Some(Hit::default()),
            false => None,
        };
    }
}
