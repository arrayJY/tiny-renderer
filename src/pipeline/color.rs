use std::ops::{Mul, Add};
#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 100 }
    }
}

impl Mul<f32> for &Color {
    type Output = Color;
    fn mul(self, v: f32) -> Self::Output {
        let r = (self.r as f32 * v) as u8;
        let g = (self.g as f32 * v) as u8;
        let b = (self.b as f32 * v) as u8;
        Color { r, g, b, a: self.a }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a,
        }
    }
}