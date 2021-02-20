use super::Color;
pub struct DepthShader {
    frame_buffer: Vec<Color>,
}

impl DepthShader {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            frame_buffer: Vec::with_capacity(height * width),
        }
    }
    pub fn shade(&mut self, z_buffer: &[f32]) {
        z_buffer.iter().for_each(|&z| {
            let v = (z * 255.0) as u8;
            self.frame_buffer.push(Color::rgba(v, v, v, 100u8))
        })
    }

    pub fn frame_buffer(&self) -> &[Color] {
        &self.frame_buffer
    }
}
