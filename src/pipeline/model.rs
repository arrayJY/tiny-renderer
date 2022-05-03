use crate::algebra::vector_new::{vector3, vector4, Vector3, Vector4};
use crate::{interpolate, interpolate_triangle};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::convert::TryInto;
use std::f32::consts::PI;
use std::sync::Arc;
use tobj;

use super::ray_tracing::pbr_shading::HitResult;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vector4,
    pub world_position: Vector4,
    pub normal: Option<Vector4>,
    pub texture_coordinate: Option<(f32, f32)>,
    // pub color: Option<Color>,
    // pub material: Option<Material>,
    pub w_reciprocal: Option<f32>,
}

#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Diffuse,
}

#[derive(Debug, Clone)]
pub struct Material {
    pub ambient_color: Vector3,  //Ka
    pub diffuse_color: Vector3,  //Kd
    pub specular_color: Vector3, //Ks
    pub shininess: f32,          //Ns
    pub optical_density: f32,    //Ni
    pub dissolve: f32,           //d
    pub emit: Option<Vector3>,
    pub material_type: MaterialType,
}

impl Material {
    pub fn eval(&self, _wi: &Vector3, wo: &Vector3, n: &Vector3) -> Vector3 {
        match self.material_type {
            MaterialType::Diffuse => {
                let cos_alpha = n.dot(wo);
                if cos_alpha > 0.0 {
                    &self.diffuse_color / PI
                } else {
                    Vector3::new()
                }
            }
        }
    }

    pub fn pdf(&self, _wi: &Vector3, _wo: &Vector3, _n: &Vector3) -> f32 {
        match self.material_type {
            MaterialType::Diffuse => {
                // if wo.dot(n) > 0.0 {
                0.5 / PI
                // } else {
                // 0.0
                // }
            }
        }
    }

    pub fn sample(&self, _wi: &Vector3, n: &Vector3) -> Vector3 {
        match self.material_type {
            MaterialType::Diffuse => {
                let mut rng = rand::thread_rng();
                let x1 = rng.gen_range(0.0f32..1.0);
                let x2 = rng.gen_range(0.0f32..1.0);
                let z = (1.0 - 2.0 * x1).abs();
                let r = (1.0 - z * z).sqrt();
                let phi = 2.0 * PI * x2;
                let locay_ray = vector3([r * phi.cos(), r * phi.sin(), z]);
                Material::to_world(&locay_ray, n)
            }
        }
    }

    fn to_world(a: &Vector3, n: &Vector3) -> Vector3 {
        let c = if n.x().abs() > n.y().abs() {
            let inv_len = 1.0 / (n.x().powi(2) + n.z().powi(2)).sqrt();
            vector3([n.z() * inv_len, 0.0, -n.x() * inv_len])
        } else {
            let inv_len = 1.0 / (n.y().powi(2) + n.z().powi(2)).sqrt();
            vector3([0.0, n.z() * inv_len, -n.y() * inv_len])
        };
        let b = c.cross(n);

        b * a.x() + c * a.y() + n * a.z()
    }
}

#[derive(Debug, Clone)]
pub struct Model {
    pub indices: Vec<[u32; 3]>,
    pub vertexs: Vec<Vertex>,
    pub material: Option<Arc<Material>>,
}

#[derive(Debug, Clone)]
pub struct TriangulatedModel {
    pub triangles: Vec<Triangle>,
    pub material: Option<Arc<Material>>,
    pub area: f32,
}

impl TriangulatedModel {
    pub fn emit(&self) -> Option<Vector3> {
        self.material
            .as_ref()
            .and_then(|material| material.emit.clone())
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
            emit: material.as_ref().and_then(|m| m.emit.clone()),
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
    pub material: Option<Arc<Material>>,
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
                    let m = &meterials[id];
                    let ka = &m.ambient;
                    let kd = &m.diffuse;
                    let ks = &m.specular;
                    let d = m.dissolve;
                    let ns = m.shininess;
                    let ni = m.optical_density;

                    Some(Arc::new(Material {
                        ambient_color: to_vector3f(ka),
                        diffuse_color: to_vector3f(kd),
                        specular_color: to_vector3f(ks),
                        shininess: ns,
                        optical_density: ni,
                        dissolve: d,
                        emit: emit_from_material(m),
                        material_type: MaterialType::Diffuse,
                    }))
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

fn emit_from_material(material: &tobj::Material) -> Option<Vector3> {
    material.unknown_param.get("emit").and_then(|emit| {
        let emit = emit
            .split(" ")
            .map(|v| v.parse::<f32>().unwrap())
            .collect::<Vec<_>>();
        Some(vector3([emit[0], emit[1], emit[2]]))
    })
}
