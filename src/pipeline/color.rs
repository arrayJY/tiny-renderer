use std::ops::{Add, AddAssign, Mul};

use crate::algebra::vector_new::{Vector3, Vector4};
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub b: u8,
    pub g: u8,
    pub r: u8,
    pub a: u8,
}

impl From<&Color> for u32 {
    fn from(c: &Color) -> Self {
        unsafe { *(c as *const Color as *const u32) }
    }
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
    pub fn add(&mut self, rhs: &Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
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

impl AddAssign<&Color> for &mut Color {
    fn add_assign(&mut self, rhs: &Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl From<&Vector3> for Color {
    fn from(v: &Vector3) -> Self {
        Self {
            r: if v.x() > 1.0 {
                255
            } else {
                (v.x() * 255.0) as u8
            },
            g: if v.y() > 1.0 {
                255
            } else {
                (v.y() * 255.0) as u8
            },
            b: if v.z() > 1.0 {
                255
            } else {
                (v.z() * 255.0) as u8
            },
            a: 255,
        }
    }
}

impl From<&Vector4> for Color {
    fn from(v: &Vector4) -> Self {
        Self {
            r: v.x() as u8,
            g: v.y() as u8,
            b: v.z() as u8,
            a: v.w() as u8,
        }
    }
}
