use crate::{
    algebra::{matrix::Matrix4f, vector::Vector4f},
    *,
};
use std::convert::TryInto;
use tobj;

#[allow(dead_code)]
pub struct Model {
    pub indices: Vec<[u32; 3]>,
    pub vertexs: Vec<Vector4f>,
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
                        let indice: [u32; 3] = mesh.indices[start..start + *num as usize]
                            .try_into()
                            .unwrap();
                        indice
                    })
                    .collect::<Vec<_>>();
                let vertexs = (0..mesh.positions.len())
                    .step_by(3)
                    .map(|i| {
                        let p = &mesh.positions;
                        //Point homogeneous coordinates: (x, y, z) -> (x, y, z, 1.0)
                        vector4f!(p[i], p[i + 1], p[i + 2], 1.0)
                    })
                    .collect::<Vec<_>>();
                Model { indices, vertexs }
            })
            .collect()
    }

    pub fn indices(&self) -> &Vec<[u32; 3]> {
        &self.indices
    }
    pub fn indices_mut(&mut self) -> &mut Vec<[u32; 3]> {
        &mut self.indices
    }

    pub fn vertexs(&self) -> &Vec<Vector4f> {
        &self.vertexs
    }
    pub fn vertexs_mut(&mut self) -> &mut Vec<Vector4f> {
        &mut self.vertexs
    }

    pub fn transform(&mut self, transform_matrix: &Matrix4f) {
        self.vertexs = self
            .vertexs
            .iter()
            .map(|vertex| transform_matrix * vertex)
            .collect();
    }
}
