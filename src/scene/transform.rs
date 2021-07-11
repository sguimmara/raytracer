use crate::math::Vec3;

#[derive(Debug, Default)]
pub struct Transform {
    position: Vec3,
}

impl Transform {
    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }
}
