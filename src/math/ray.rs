use crate::math::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Returns the point along the ray at the specified distance from the origin.
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
