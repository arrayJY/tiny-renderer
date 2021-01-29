use std::convert::TryInto;

use crate::algebra::typenum::U4;
use crate::{algebra::vector::Vectorf, vectorf};
use tobj;

#[allow(dead_code)]
pub struct Model {
    indices: Vec<[u32; 3]>,
    vertexs: Vec<Vectorf<U4>>,
}

#[allow(dead_code)]
impl Model {
    pub fn new() -> Self {
        Model {
            indices: Vec::new(),
            vertexs: Vec::new(),
        }
    }

    //TODO: load more information.
    pub fn from_obj(path: &str) -> Vec<Self> {
        let obj = tobj::load_obj(path, true);
        assert!(obj.is_ok());
        let (models, _) = obj.unwrap();

        models
            .iter()
            .map(|model| {
                let mesh = &model.mesh;
                //Faces has been triangulated, so step by 3.
                println!("num_face_indices: {:?}", mesh.num_face_indices);
                let indices = (0..mesh.num_face_indices.len())
                    .zip(mesh.num_face_indices.iter())
                    .map(|(start, num)| {
                        let indice: [u32; 3] = mesh.indices[start..start + *num as usize].try_into().unwrap();
                        indice
                    })
                    .collect::<Vec<_>>();
                let vertexs = (0..mesh.positions.len())
                    .step_by(3)
                    .map(|i| {
                        let p = &mesh.positions;
                        //Point homogeneous coordinates: (x, y, z) -> (x, y, z, 1.0)
                        vectorf!(U4; p[i], p[i+1], p[i+2], 1.0)
                    })
                    .collect::<Vec<_>>();
                Model { indices, vertexs }
            })
            .collect()
    }

    pub fn indices(&self) -> &Vec<[u32; 3]> {
        &self.indices
    }

    pub fn vertexs(&self) -> &Vec<Vectorf<U4>> {
        &self.vertexs
    }
}
