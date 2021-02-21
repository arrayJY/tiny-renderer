use crate::{
    algebra::{matrix::Matrix4f, vector::Vector4f},
    *,
};
use std::convert::TryInto;
use tobj;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Model {
    pub indices: Vec<[u32; 3]>,
    pub vertexs: Vec<Vector4f>,
    pub normals: Option<Vec<Vector4f>>,
    pub texture_coordinates: Option<Vec<(f32, f32)>>,
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub points: Vec<Vector4f>,
}

#[allow(dead_code)]
impl Model {
    pub fn new() -> Self {
        Model {
            indices: Vec::new(),
            vertexs: Vec::new(),
            normals: None,
            texture_coordinates: None,
        }
    }

    pub fn from_obj(path: &str) -> Vec<Self> {
        let obj = tobj::load_obj(path, true);
        assert!(obj.is_ok());
        let (models, _) = obj.unwrap();

        models
            .iter()
            .map(|model| {
                let mesh = &model.mesh;
                let mut indices = Vec::new();
                let mut next_face = 0;

                //Load indices.
                for f in 0..mesh.num_face_indices.len() {
                    let end = next_face + mesh.num_face_indices[f] as usize;
                    let face_indices: [u32; 3] = mesh.indices[next_face..end].try_into().unwrap();
                    indices.push(face_indices);
                    next_face = end;
                }

                //Load vertexs.
                let vertexs = (0..mesh.positions.len() / 3)
                    .map(|i| {
                        let p = &mesh.positions;
                        //Point homogeneous coordinates: (x, y, z) -> (x, y, z, 1.0)
                        vector4f!(p[i * 3], p[i * 3 + 1], p[i * 3 + 2], 1.0)
                    })
                    .collect::<Vec<_>>();

                //Load normals.
                let normals = if mesh.normals.is_empty() {
                    None
                } else {
                    Some(
                        (0..mesh.normals.len() / 3)
                            .map(|i| {
                                let p = &mesh.positions;
                                //Point homogeneous coordinates: (x, y, z) -> (x, y, z, 1.0)
                                vector4f!(p[i * 3], p[i * 3 + 1], p[i * 3 + 2], 1.0)
                            })
                            .collect::<Vec<_>>(),
                    )
                };

                //Load texture_coordinates.
                let texture_coordinates = if mesh.texcoords.is_empty() {
                    None
                } else {
                    Some(
                        (0..mesh.texcoords.len() / 2)
                            .map(|i| {
                                let p = &mesh.texcoords;
                                //Point homogeneous coordinates: (x, y, z) -> (x, y, z, 1.0)
                                (p[i * 2], p[i * 2 + 1])
                            })
                            .collect::<Vec<_>>(),
                    )
                };
                Model {
                    indices,
                    vertexs,
                    normals,
                    texture_coordinates,
                }
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
        self.normalize_vertex();
    }

    pub fn triangles(&self) -> Vec<Triangle> {
        self.indices()
            .iter()
            .map(|index_group| Triangle {
                points: index_group
                    .iter()
                    .map(|&index| &self.vertexs[index as usize])
                    .map(|vertex| Vector4f::from(vertex))
                    .collect(),
            })
            .collect()
    }

    pub fn normalize_vertex(&mut self) {
        self.vertexs = self
            .vertexs
            .iter()
            .map(|vertex| {
                if vertex.w() != 1.0 {
                    vertex / vertex.w()
                } else {
                    vertex / 1.0
                }
            })
            .collect()
    }
}
