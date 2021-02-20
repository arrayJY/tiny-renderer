use algebra::vector::{Vector3f};

use crate::*;

use super::model::Triangle;

#[allow(dead_code)]
pub struct Rasterizer {
    pub triangles: Vec<Triangle>,
    pub z_buffer: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl Rasterizer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            triangles: Vec::new(),
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
        let z_buffer = &mut self.z_buffer;
        let width = self.width;

        self.triangles.iter().for_each(|triangle| {
            let (min_x, min_y, max_x, max_y) = Rasterizer::bounding_box(triangle);
            let coord_iter = (min_x..max_x).flat_map(move |a| (min_y..max_y).map(move |b| (a, b)));

            coord_iter.for_each(|(x, y)| {
                if Rasterizer::inside_triangle((x, y), triangle) {
                    let index = y * width + x;
                    let barycenter = Rasterizer::barycentric_2d(x as f32, y as f32, triangle);
                    let z = -Rasterizer::z_interpolation(triangle, barycenter);
                    if z < z_buffer[index] {
                        //set color
                        z_buffer[index] = z;
                    }
                }
            })
        });
    }

    fn z_interpolation(triangle: &Triangle, (alpha, beta, gamma): (f32, f32, f32)) -> f32 {
        let v = &triangle.points;
        let w_reciprocal= 1.0 / (alpha / v[0].w() + beta / v[1].w() + gamma / v[2].w());
        let mut z_interpolated =
                        alpha * v[0].z() / v[0].w() + beta * v[1].z() / v[1].w() + gamma * v[2].z() / v[2].w();
        z_interpolated *= w_reciprocal;
        z_interpolated
    }

    fn barycentric_2d(x: f32, y: f32, triangle: &Triangle) -> (f32, f32, f32) {
        let v = &triangle.points;
        let c1 = (x * (v[1].y() - v[2].y()) + (v[2].x() - v[1].x()) * y + v[1].x() * v[2].y()
            - v[2].x() * v[1].y())
            / (v[0].x() * (v[1].y() - v[2].y())
                + (v[2].x() - v[1].x()) * v[0].y()
                + v[1].x() * v[2].y()
                - v[2].x() * v[1].y());
        let c2 = (x * (v[2].y() - v[0].y()) + (v[0].x() - v[2].x()) * y + v[2].x() * v[0].y()
            - v[0].x() * v[2].y())
            / (v[1].x() * (v[2].y() - v[0].y())
                + (v[0].x() - v[2].x()) * v[1].y()
                + v[2].x() * v[0].y()
                - v[0].x() * v[2].y());
        let c3 = (x * (v[0].y() - v[1].y()) + (v[1].x() - v[0].x()) * y + v[0].x() * v[1].y()
            - v[1].x() * v[0].y())
            / (v[2].x() * (v[0].y() - v[1].y())
                + (v[1].x() - v[0].x()) * v[2].y()
                + v[0].x() * v[1].y()
                - v[1].x() * v[0].y());
        (c1, c2, c3)
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
