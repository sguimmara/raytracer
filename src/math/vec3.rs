use std::ops::{Add, Mul, Div, Sub};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let v = Vec3::new(1f32, 2f32, 3f32);
        assert_eq!(v.x, 1f32);
        assert_eq!(v.y, 2f32);
        assert_eq!(v.z, 3f32);
    }

    #[test]
    fn add() {
        let v0 = Vec3::new(2f32, 5f32, 99f32);
        let v1 = Vec3::new(6f32, 1f32, 1f32);

        let v = v0 + v1;

        assert_eq!(v.x, 8f32);
        assert_eq!(v.y, 6f32);
        assert_eq!(v.z, 100f32);
    }
}
