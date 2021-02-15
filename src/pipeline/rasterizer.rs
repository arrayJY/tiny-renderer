use algebra::vector::Vector3f;

use crate::*;

use super::model::Triangle;

#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn new() -> Self {
        Self {
            r: 0u8,
            g: 0u8,
            b: 0u8,
            a: 0u8,
        }
    }
}

#[allow(dead_code)]
pub struct Rasterizer {
    pub triangles: Vec<Triangle>,
    pub frame_buffer: Vec<Color>,
    pub z_buffer: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            triangles: Vec::new(),
            frame_buffer: vec![Color::new(); width * height],
            z_buffer: vec![f32::MAX; width * height],
            width,
            height,
        }
    }

    pub fn triangles(mut self, triangles: Vec<Triangle>) -> Self {
        self.triangles = triangles;
        self
    }

    pub fn rasterize(&mut self) {
        let frame_buffer = &mut self.frame_buffer;
        let z_buffer = &mut self.z_buffer;
        let width = self.width;
        let red = Color {
            r: 255u8,
            g: 0u8,
            b: 0u8,
            a: 0u8,
        };

        self.triangles.iter().for_each(|triangle| {
            let (min_x, min_y, max_x, max_y) = Rasterizer::bounding_box(triangle);
            let coord_iter = (min_x..max_x).flat_map(move |a| (min_y..max_y).map(move |b| (a, b)));

            coord_iter.for_each(|(x, y)| {
                if Rasterizer::inside_triangle((x, y), triangle) {
                    let index = y * width + x;
                    let z = -Rasterizer::z_interpolation(triangle);
                    if z < z_buffer[index] {
                        //set color
                        frame_buffer[index] = red.clone();
                        z_buffer[index] = z;
                    }
                }
            })
        });
    }

    fn z_interpolation(triangle: &Triangle) -> f32 {
        triangle.points.iter().map(|v| v.z()).sum::<f32>() / triangle.points.len() as f32
    }

    fn bounding_box(triangle: &Triangle) -> (usize, usize, usize, usize) {
        let mut x = triangle
            .points
            .iter()
            .map(|vertex| vertex.x())
            .collect::<Vec<_>>();
        let mut y = triangle
            .points
            .iter()
            .map(|vertex| vertex.y())
            .collect::<Vec<_>>();

        x.sort_by(|a, b| a.partial_cmp(b).unwrap());
        y.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let &min_x = x.first().unwrap();
        let &max_x = x.last().unwrap();
        let &min_y = y.first().unwrap();
        let &max_y = y.last().unwrap();

        let (min_x, min_y, max_x, max_y) = (
            min_x as usize,
            min_y as usize,
            max_x as usize,
            max_y as usize,
        );

        (min_x, min_y, max_x, max_y)
    }

    fn inside_triangle((x, y): (usize, usize), triangle: &Triangle) -> bool {
        let a = &triangle.points[0];
        let b = &triangle.points[1];
        let c = &triangle.points[2];

        let pp = vector3f!(x as f32 + 0.5, y as f32 + 0.5, 0.0);
        let pa = vector3f!(a.x(), a.y(), a.z());
        let pb = vector3f!(b.x(), b.y(), b.z());
        let pc = vector3f!(c.x(), c.y(), c.z());

        let ab = &pb - &pa;
        let bc = &pc - &pb;
        let ca = &pa - &pc;

        let ap = &pp - &pa;
        let bp = &pp - &pb;
        let cp = &pp - &pc;

        (ab.cross(&ap).z() > 0.0 && bc.cross(&bp).z() > 0.0 && ca.cross(&cp).z() > 0.0)
            || (ab.cross(&ap).z() < 0.0 && bc.cross(&bp).z() < 0.0 && ca.cross(&cp).z() < 0.0)
    }
}
