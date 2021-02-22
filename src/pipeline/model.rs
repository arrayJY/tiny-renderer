use crate::{algebra::vector::Vector4f, Color, *};
use std::convert::TryInto;
use tobj;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector4f,
    pub normal: Option<Vector4f>,
    pub texture_coordinate: Option<(f32, f32)>,
    pub color: Option<Color>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Model {
    pub indices: Vec<[u32; 3]>,
    pub vertexs: Vec<Vertex>,
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertexs: Vec<Vertex>,
}

#[allow(dead_code)]
impl Model {
    pub fn new() -> Self {
        Model {
            indices: Vec::new(),
            vertexs: Vec::new(),
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
                let positions = (0..mesh.positions.len() / 3)
                    .map(|i| {
                        let p = &mesh.positions;
                        //Point homogeneous coordinates: (x, y, z) -> (x, y, z, 1.0)
                        vector4f!(p[i * 3], p[i * 3 + 1], p[i * 3 + 2], 1.0)
                    })
                    .collect::<Vec<_>>();

                //Load normals.
                let normals = if mesh.normals.is_empty() {
                    vec![None; mesh.positions.len() / 3]
                } else {
                    (0..mesh.normals.len() / 3)
                        .map(|i| {
                            let p = &mesh.normals;
                            //Point homogeneous coordinates: (x, y, z) -> (x, y, z, 1.0)
                            Some(vector4f!(p[i * 3], p[i * 3 + 1], p[i * 3 + 2], 1.0))
                        })
                        .collect::<Vec<_>>()
                };

                //Load texture_coordinates.
                let texture_coordinates = if mesh.texcoords.is_empty() {
                    vec![None; mesh.positions.len() / 3]
                } else {
                    (0..mesh.texcoords.len() / 2)
                        .map(|i| {
                            let p = &mesh.texcoords;
                            Some((p[i * 2], p[i * 2 + 1]))
                        })
                        .collect::<Vec<_>>()
                };

                let iter = positions
                    .iter()
                    .zip(normals.iter())
                    .zip(texture_coordinates.iter())
                    .map(|((a, b), c)| (a, b, c));

                let vertexs = iter
                    .map(|(position, normal, texture_coordinate)| Vertex {
                        position: position.clone(),
                        normal: normal.clone(),
                        texture_coordinate: texture_coordinate.clone(),
                        color: None,
                    })
                    .collect();

                Model { indices, vertexs }
            })
            .collect()
    }

    // Set colors of vertexs.
    // Colors will be circular used if they are less than vertexs,
    pub fn colors(mut self, colors: &[Color]) -> Self {
        self.vertexs
            .iter_mut()
            .zip(colors.iter().cycle())
            .for_each(|(vertex, color)| vertex.color = Some(color.clone()));
        self
    }

    pub fn triangles(self) -> Vec<Triangle> {
        self.indices
            .iter()
            .map(|index_group| Triangle {
                vertexs: index_group
                    .iter()
                    .map(|&index| &self.vertexs[index as usize])
                    .map(|vertex| vertex.clone())
                    .collect(),
            })
            .collect()
    }
}
