use crate::algebra::vector_new::{vector3, vector4, Vector3, Vector4};
use crate::pipeline::material::{
    MaterialNew, OptionEmissiveMaterial, PBRMaterial, PhongMaterial,
};
use crate::{interpolate, interpolate_triangle};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::sync::Arc;
use tobj;

use crate::ray_tracing::path_tracing::HitResult;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector4,
    pub world_position: Vector4,
    pub normal: Option<Vector4>,
    pub texture_coordinate: Option<(f32, f32)>,
    pub w_reciprocal: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub indices: Vec<[u32; 3]>,
    pub vertexs: Vec<Vertex>,
    pub material: Option<Arc<MaterialNew>>,
}

#[derive(Debug, Clone)]
pub struct TriangulatedModel {
    pub triangles: Vec<Triangle>,
    pub material: Option<Arc<MaterialNew>>,
    pub area: f32,
}

impl TriangulatedModel {
    pub fn emit(&self) -> Option<Vector3> {
        self.material
            .as_ref()
            .and_then(|m| m.emissive_material())
            .and_then(|m| Some(&m.base_color * m.intensity))
    }
    pub fn has_emit(&self) -> bool {
        self.emit().is_some()
    }
    pub fn sample(&self) -> (HitResult, f32) {
        let pdf = 1.0 / self.area();
        let mut rng = rand::thread_rng();
        let chosen_triangle = self
            .triangles
            .choose(&mut rng)
            .expect("model has no triangle.");
        let material = chosen_triangle.material.clone();

        let (position, normal) = chosen_triangle.sample_position();

        let hit_result = HitResult {
            position,
            normal,
            distance: 0.0, //No use
            emit: material
                .as_ref()
                .and_then(|m| Some(m.as_ref()))
                .and_then(|m| {
                    if let MaterialNew::Emissive(m) = m {
                        Some(&m.base_color * m.intensity)
                    } else {
                        None
                    }
                }),
            material,
        };

        (hit_result, pdf)
    }

    pub fn area(&self) -> f32 {
        self.triangles.iter().map(|triangle| triangle.area).sum()
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertexs: Vec<Vertex>,
    pub material: Option<Arc<MaterialNew>>,
    pub area: f32,
}

impl Triangle {
    pub fn get_barycenter(&self) -> Vector4 {
        const C: f32 = 1.0f32 / 3.0f32;
        let mut r = Vector4::new();
        let iter = {
            let i = self.vertexs.iter();
            let j = self.vertexs.iter().cycle().skip(1);
            i.zip(j)
        };
        for (i, j) in iter {
            r = r + (&i.position - &j.position) * C;
        }
        r
    }

    pub fn calc_area(&mut self) {
        let a = &self.vertexs[0].position;
        let b = &self.vertexs[1].position;
        let c = &self.vertexs[2].position;
        let ab = Vector3::from(&(b - a));
        let ac = Vector3::from(&(c - a));
        self.area = ab.cross(&ac).norm() / 2.0;
    }

    pub fn sample_position(&self) -> (Vector3, Vector3) {
        let x = rand::thread_rng().gen_range(0.0f32..=1.0).sqrt();
        let y = rand::thread_rng().gen_range(0.0f32..=1.0);
        let a = 1.0 - x;
        let b = x * (1.0 - y);
        let c = x * y;

        let position = Vector3::from(&interpolate_triangle!(self, position; (a,b,c)));
        let normal = Vector3::from(&interpolate!(self, normal; (a, b, c)).normalized());
        (position, normal)
    }
}

#[allow(dead_code)]
impl Model {
    pub fn new() -> Self {
        Model {
            indices: Vec::new(),
            vertexs: Vec::new(),
            material: None,
        }
    }

    pub fn from_obj(path: &str) -> Vec<Self> {
        let obj = tobj::load_obj(path, true);
        assert!(obj.is_ok());
        let (models, meterials) = obj.unwrap();

        fn to_vector3f(c: &[f32; 3]) -> Vector3 {
            vector3(c.clone())
        }

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
                        vector4([p[i * 3], p[i * 3 + 1], p[i * 3 + 2], 1.0])
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
                            Some(vector4([p[i * 3], p[i * 3 + 1], p[i * 3 + 2], 0.0]).normalized())
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
                        world_position: position.clone(),
                        normal: normal.clone(),
                        texture_coordinate: texture_coordinate.clone(),
                        // color: None,
                        // material: material,
                        w_reciprocal: None,
                    })
                    .collect();

                let material = mesh.material_id.map_or(None, |id| {
                    Some(Arc::new(MaterialNew::Phong(PhongMaterial::from(
                        &meterials[id],
                    ))))
                });
                Model {
                    indices,
                    vertexs,
                    material,
                }
            })
            .collect()
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
                material: self.material.clone(),
                area: 0.0,
            })
            .collect()
    }
}

impl Model {
    pub fn from_gltf(path: &str) -> Vec<Self> {
        use crate::algebra::matrix_new::Matrix4;
        let (gltf, buffers, _) = gltf::import(path).unwrap();

        // println!("transforms: {}", transforms.len());

        let nodes = gltf.scenes().flat_map(|scene| scene.nodes()).map(|node| {
            let attrs = node
                .mesh()
                .into_iter()
                .flat_map(|mesh| mesh.primitives())
                .enumerate()
                .map(|(i, primitive)| {
                    println!("{}", i);
                    let material = primitive.material();
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                    let indices = reader
                        .read_indices()
                        .expect("Indices no found.")
                        .into_u32()
                        // .map(|i| i as usize)
                        .collect::<Vec<_>>();
                    let indices = indices
                        .chunks(3)
                        .map(|c| [c[0], c[1], c[2]])
                        .collect::<Vec<_>>();

                    let positions = reader
                        .read_positions()
                        .expect("Positions not found.")
                        .map(|p| Vector4::from(&vector3(p)))
                        // .map(|p| transform * &p)
                        .collect::<Vec<_>>();

                    let normals = reader
                        .read_normals()
                        .expect("Normals not found.")
                        .map(|n| Vector4::from(&vector3(n)))
                        // .map(|n| transform * &n)
                        .collect::<Vec<_>>();
                    ((positions, normals, indices), material)
                });
            attrs
        });

        let transforms = gltf
            .scenes()
            .flat_map(|scene| scene.nodes())
            .map(|node| Matrix4::from(node.transform().matrix()).transpose())
            .cycle();

        let models = nodes
            .zip(transforms)
            .flat_map(|(mesh_attr, transform)| {
                let translation = Matrix4::translation_matrix(0.0, 0.0, -1.0);
                println!("{:?}", transform);
                println!("{:?}", translation);
                let models = mesh_attr
                    .map(|((positions, normals, indices), material)| {
                        let positions = positions
                            .iter()
                            // .map(|p| vector4([p.x() + x, p.y() + y, p.z() + z, 1.0]))
                            .map(|v| &transform * v)
                            .collect::<Vec<_>>();

                        let normals = normals
                            .iter()
                            .map(|n| &transform * n)
                            .map(|v| v.normalized() )
                            .collect::<Vec<_>>();

                        let material = OptionEmissiveMaterial::from(&material).0.map_or_else(
                            || MaterialNew::PBR(PBRMaterial::from(&material)),
                            |m| MaterialNew::Emissive(m),
                        );

                        let vertexs = positions
                            .into_iter()
                            .zip(normals.into_iter())
                            .map(|(position, normal)| Vertex {
                                position: position.clone(),
                                world_position: position.clone(),
                                normal: Some(normal),
                                texture_coordinate: None,
                                w_reciprocal: None,
                            })
                            .collect::<Vec<_>>();
                        Model {
                            vertexs,
                            indices,
                            material: Some(Arc::new(material)),
                        }
                    })
                    .collect::<Vec<_>>();
                models
            })
            .collect::<Vec<_>>();
        models
    }
}
