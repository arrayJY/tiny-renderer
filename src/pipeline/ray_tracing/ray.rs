use crate::{algebra::vector_new::Vector3, pipeline::model::Triangle};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub dir: Vector3,
}

impl Ray {
    pub fn new(p: &Vector3, d: &Vector3) -> Self {
        Self {
            origin: p.clone(),
            dir: d.clone().normalized(),
        }
    }

    pub fn intersect_triangle(&self, triangle: &Triangle) -> Option<(f32, f32, f32)> {
        // Moller-Trumbore
        let p0 = Vector3::from(&triangle.vertexs[0].position);
        let p1 = Vector3::from(&triangle.vertexs[1].position);
        let p2 = Vector3::from(&triangle.vertexs[2].position);

        let s = &self.origin - &p0;
        let e1 = &p1 - &p0;
        let e2 = &p2 - &p0;
        let s1 = self.dir.cross(&e2);
        let s2 = s.cross(&e1);

        let s1e1 = s1.dot(&e1);

        let t = s2.dot(&e2) / s1e1;
        let b1 = s1.dot(&s) / s1e1;
        let b2 = s2.dot(&self.dir) / s1e1;
        if t >= 0.0 && b1 >= 0.0 && b2 >= 0.0 && (1.0 - b1 - b2) >= 0.0 {
            Some((1.0 - b1 - b2, b1, b2))
        } else {
            None
        }
    }
}
