use crate::{algebra::vector::Vector4f, Color, *};
use std::{collections::HashSet, convert::TryInto};
use tobj;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector4f,
    pub world_position: Option<Vector4f>,
    pub normal: Option<Vector4f>,
    pub texture_coordinate: Option<(f32, f32)>,
    pub color: Option<Color>,
    pub w_reciprocal: Option<f32>,
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
                            //Vector homogeneous coordinates: (x, y, z) -> (x, y, z, 0.0)
                            Some(vector4f!(p[i * 3], p[i * 3 + 1], p[i * 3 + 2], 0.0).normalized())
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
                        //Reserve positions in world space for fragment shader.
                        world_position: Some(position.clone()),
                        normal: normal.clone(),
                        texture_coordinate: texture_coordinate.clone(),
                        color: None,
                        w_reciprocal:None,
                    })
                    .collect();

                Model { indices, vertexs }.default_color()
            })
            .collect()
    }

    // Set colors of vertexs.
    // Colors will be circular used if they are less than vertexs,
    pub fn colors(mut self, colors: &[Color]) -> Self {
        // The vertex in the same position may have different attributes when they are sharing
        // by different surfaces. So vertexs.len() in `Model` >= vertexs in .obj files.
        // So we have to find vertexs in the same position, then color they with same color
        let mut vertex_index_set: HashSet<usize> = HashSet::with_capacity(self.vertexs.len());
        let mut repeat_vertexs: Vec<Vec<usize>> = vec![Vec::new(); self.vertexs.len()];
        self.vertexs
            .iter()
            .map(|v| &v.position)
            .enumerate()
            .for_each(|(i, p)| {
                if !vertex_index_set.contains(&i) {
                    (&self.vertexs[i..]).iter().enumerate().for_each(|(j, v)| {
                        if &v.position == p {
                            vertex_index_set.insert(i + j);
                            repeat_vertexs[i].push(i + j);
                        }
                    });
                }
            });
        let repeat_vertexs: Vec<Vec<usize>> = repeat_vertexs
            .into_iter()
            .filter(|v| !v.is_empty())
            .collect();

        // Color
        repeat_vertexs
            .iter()
            .zip(colors.iter().cycle())
            .for_each(|(indexs, color)| {
                indexs.iter().for_each(|&i| {
                    self.vertexs[i].color = Some(color.clone());
                })
            });

        self
    }

    pub fn default_color(self) -> Self {
        self.colors(&vec![Color::rgb(127, 127, 127)])
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
