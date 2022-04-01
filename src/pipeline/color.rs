use std::ops::{Mul, Add};

use crate::algebra::vector_new::{Vector3, Vector4};
#[repr(C)]
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

impl From<&Vector3> for Color{
    fn from(v: &Vector3) -> Self {
        Self {
            r: v.x() as u8,
            g: v.y() as u8,
            b: v.z() as u8,
            a: 255,
        }
    }
}

impl From<&Vector4> for Color{
    fn from(v: &Vector4) -> Self {
        Self {
            r: v.x() as u8,
            g: v.y() as u8,
            b: v.z() as u8,
            a: v.w() as u8,
        }
    }
}