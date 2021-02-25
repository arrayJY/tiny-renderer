use crate::{algebra::vector::Vector3f, *};

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vector3f,
    pub intensity: f32,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            position: vector3f!(10.0, 10.0, -10.0),
            intensity: 300.0,
        }
    }
}

impl Light{
    pub fn position(mut self, position: Vector3f) -> Self {
        self.position = position;
        self
    }
}
