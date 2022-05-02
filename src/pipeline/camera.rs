use crate::algebra::vector_new::{vector3, Vector3};
use std::f32::consts::PI;
#[derive(Debug, Clone)]
pub struct Camera {
    pub gaze_direct: Vector3,
    pub up_direct: Vector3,
    pub eye_position: Vector3,
    pub eye_fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

#[allow(dead_code)]
impl Camera {
    pub fn new() -> Self {
        Camera {
            gaze_direct: Vector3::new(),
            up_direct: Vector3::new(),
            eye_position: Vector3::new(),
            eye_fov: 0.0,
            aspect_ratio: 0.0,
            near: 0.0,
            far: 0.0,
        }
    }

    pub fn gaze_direct(mut self, g: Vector3) -> Self {
        self.gaze_direct = g;
        self
    }
    pub fn up_direct(mut self, u: Vector3) -> Self {
        self.up_direct = u;
        self
    }
    pub fn eye_position(mut self, e: Vector3) -> Self {
        self.eye_position = e;
        self
    }
    pub fn eye_fov(mut self, fov: f32) -> Self {
        self.eye_fov = fov;
        self
    }
    pub fn aspect_ratio(mut self, ratio: f32) -> Self {
        self.aspect_ratio = ratio;
        self
    }

    pub fn near(mut self, near: f32) -> Self {
        self.near = near;
        self
    }
    pub fn far(mut self, far: f32) -> Self {
        self.far = far;
        self
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            /*
            gaze_direct: vector3([-1.0, -1.0, -1.0]).normalized(),
            up_direct: vector3([-1.0, 1.0, -1.0]).normalized(),
            eye_position: vector3([3.0, 3.0, 3.0]),
            */
            gaze_direct: vector3([0.0, 0.0, -1.0]).normalized(),
            up_direct: vector3([0.0, 1.0, 0.0]).normalized(),
            eye_position: vector3([0.0, 0.0, 2.0]),
            eye_fov: PI / 2.0,
            aspect_ratio: 1.0,
            near: 0.1,
            far: 50.0,
        }
    }
}
