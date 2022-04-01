use crate::algebra::vector_new::{vector3, Vector3};

#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vector3,
    pub intensity: f32,
}

impl Default for Light {
    fn default() -> Self {
        Self {
            position: vector3([10.0, 10.0, -10.0]),
            intensity: 300.0,
        }
    }
}

impl Light {
    pub fn position(mut self, position: Vector3) -> Self {
        self.position = position;
        self
    }
}
