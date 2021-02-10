use crate::algebra::vector::Vector3f;
pub struct Camera {
    pub gaze_direct: Vector3f,
    pub up_direct: Vector3f,
    pub eye_position: Vector3f,
    pub eye_fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

#[allow(dead_code)]
impl Camera {
    pub fn new() -> Self {
        Camera {
            gaze_direct: Vector3f::new(),
            up_direct: Vector3f::new(),
            eye_position: Vector3f::new(),
            eye_fov: 0.0,
            aspect_ratio: 0.0,
            near: 0.0,
            far: 0.0,
        }
    }

    pub fn gaze_direct(mut self, g: Vector3f) -> Self {
        self.gaze_direct = g;
        self
    }
    pub fn up_direct(mut self, u: Vector3f) -> Self {
        self.up_direct = u;
        self
    }
    pub fn eye_position(mut self, e: Vector3f) -> Self {
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
