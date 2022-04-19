use rand::{prelude::SliceRandom, Rng};

use crate::{algebra::vector_new::Vector3, interpolate_triangle, pipeline::model::Triangle, interpolate};

pub struct AreaLight {
    pub triangles: Vec<Triangle>,
}

impl AreaLight {
    pub fn random_point(&self) -> (Vector3, Vector3) {
        let mut rng = rand::thread_rng();
        let triangle = self.triangles.choose(&mut rng).unwrap();
        let a = rng.gen_range(0.0f32..1.0);
        let b = rng.gen_range(0.0f32..(1.0 - a));
        let c = 1.0 - a - b;
        let barycenter = (a,b,c);
        let position = Vector3::from(&interpolate_triangle!(triangle, world_position; barycenter));
        let normal = Vector3::from(&interpolate!(triangle, normal; barycenter).normalized());
        (position, normal)
    }
}
