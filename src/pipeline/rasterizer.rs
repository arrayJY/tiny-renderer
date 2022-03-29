use core::f32;

use crate::algebra::vector_new::{vector3};
use crate::Color;


use super::{fragment_shader::FragmentShader, model::Triangle};

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

    pub fn rasterize(&mut self, shader: &Box<dyn FragmentShader>) -> Vec<Option<Color>> {
        let z_buffer = &mut self.z_buffer;
        let mut frame_buffer = vec![None; self.height * self.width];
        let width = self.width;

        self.triangles.iter().for_each(|triangle| {
            let (min_x, min_y, max_x, max_y) = Rasterizer::bounding_box(triangle);
            let coord_iter = (min_x..max_x).flat_map(move |a| (min_y..max_y).map(move |b| (a, b)));

            coord_iter.for_each(|(x, y)| {
                let barycenter = Rasterizer::barycentric_2d(x as f32, y as f32, triangle);
                if Rasterizer::inside_triangle_by_barycenter(barycenter) {
                    let index = y * width + x;
                    let z = -Rasterizer::z_interpolation(triangle, barycenter);
                    if z < z_buffer[index] {
                        let barycenter = Rasterizer::perspective_correct(triangle, barycenter);
                        let color = shader.shade(triangle, barycenter, z);
                        frame_buffer[index] = Some(color);
                        z_buffer[index] = z;
                    }
                }
            })
        });

        frame_buffer
    }

    fn z_interpolation(triangle: &Triangle, (alpha, beta, gamma): (f32, f32, f32)) -> f32 {
        let v0 = &triangle.vertexs[0].position;
        let v1 = &triangle.vertexs[1].position;
        let v2 = &triangle.vertexs[2].position;
        let w_reciprocal = 1.0 / (alpha / v0.w() + beta / v1.w() + gamma / v2.w());
        let mut z_interpolated =
            alpha * v0.z() / v0.w() + beta * v1.z() / v1.w() + gamma * v2.z() / v2.w();
        z_interpolated *= w_reciprocal;
        z_interpolated
    }

    fn barycentric_2d(x: f32, y: f32, triangle: &Triangle) -> (f32, f32, f32) {
        let v0 = &triangle.vertexs[0].position;
        let v1 = &triangle.vertexs[1].position;
        let v2 = &triangle.vertexs[2].position;
        let c1 = (x * (v1.y() - v2.y()) + (v2.x() - v1.x()) * y + v1.x() * v2.y()
            - v2.x() * v1.y())
            / (v0.x() * (v1.y() - v2.y()) + (v2.x() - v1.x()) * v0.y() + v1.x() * v2.y()
                - v2.x() * v1.y());
        let c2 = (x * (v2.y() - v0.y()) + (v0.x() - v2.x()) * y + v2.x() * v0.y()
            - v0.x() * v2.y())
            / (v1.x() * (v2.y() - v0.y()) + (v0.x() - v2.x()) * v1.y() + v2.x() * v0.y()
                - v0.x() * v2.y());
        let c3 = (x * (v0.y() - v1.y()) + (v1.x() - v0.x()) * y + v0.x() * v1.y()
            - v1.x() * v0.y())
            / (v2.x() * (v0.y() - v1.y()) + (v1.x() - v0.x()) * v2.y() + v0.x() * v1.y()
                - v1.x() * v0.y());
        (c1, c2, c3)
    }

    fn perspective_correct(
        triangle: &Triangle,
        (alpha, beta, gamma): (f32, f32, f32),
    ) -> (f32, f32, f32) {
        let w0 = triangle.vertexs[0].w_reciprocal.unwrap() * alpha;
        let w1 = triangle.vertexs[1].w_reciprocal.unwrap() * beta;
        let w2 = triangle.vertexs[2].w_reciprocal.unwrap() * gamma;
        let normalizer = 1.0 / (w0 + w1 + w2);
        (w0 * normalizer, w1 * normalizer, w2 * normalizer)
    }

    fn bounding_box(triangle: &Triangle) -> (usize, usize, usize, usize) {
        let mut min_x: f32 = f32::MAX;
        let mut min_y: f32 = f32::MAX;
        let mut max_x: f32 = f32::MIN;
        let mut max_y: f32 = f32::MIN;
        triangle
            .vertexs
            .iter()
            .map(|vertex| &vertex.position)
            .for_each(|position| {
                let (x, y) = (position.x(), position.y());
                min_x = if x < min_x { x } else { min_x };
                min_y = if y < min_y { y } else { min_y };
                max_x = if x > max_x { x } else { max_x };
                max_y = if y > max_y { y } else { max_y };
            });

        let (min_x, min_y, max_x, max_y) = (
            min_x.floor() as usize,
            min_y.floor() as usize,
            max_x.ceil() as usize,
            max_y.ceil() as usize,
        );

        (min_x, min_y, max_x, max_y)
    }

    fn inside_triangle((x, y): (usize, usize), triangle: &Triangle) -> bool {
        let a = &triangle.vertexs[0].position;
        let b = &triangle.vertexs[1].position;
        let c = &triangle.vertexs[2].position;

        let pp = vector3([x as f32 + 0.5, y as f32 + 0.5, 0.0]);
        let pa = vector3([a.x(), a.y(), a.z()]);
        let pb = vector3([b.x(), b.y(), b.z()]);
        let pc = vector3([c.x(), c.y(), c.z()]);

        let ab = &pb - &pa;
        let bc = &pc - &pb;
        let ca = &pa - &pc;

        let ap = &pp - &pa;
        let bp = &pp - &pb;
        let cp = &pp - &pc;

        (ab.cross(&ap).z() > 0.0 && bc.cross(&bp).z() > 0.0 && ca.cross(&cp).z() > 0.0)
            || (ab.cross(&ap).z() < 0.0 && bc.cross(&bp).z() < 0.0 && ca.cross(&cp).z() < 0.0)
    }

    fn inside_triangle_by_barycenter((alpha, beta, gamma) : (f32, f32, f32)) -> bool {
        !(alpha < 0.0 || beta < 0.0 || gamma < 0.0)
    }
}
