use crate::{
    algebra::vector_new::{vector3, Vector3},
    interpolate_triangle,
    pipeline::{model::Triangle, ray_tracing::ray::Ray},
    Color, interpolate,
};
use rand::Rng;
use std::f32::consts::PI;

use super::data_structure::{bvh::{BVHNode, BVHTree}, light::AreaLight};

type FrameBuffer = Vec<Option<Color>>;

pub fn pbr_shade(width: usize, height: usize, triangles: Vec<Triangle>) -> FrameBuffer {
    let mut framebuffer: FrameBuffer = vec![None; width * height];

    framebuffer
}

pub struct RayTracer {
    objects: BVHTree,
    light: AreaLight
}

#[derive(Debug, Clone)]
pub struct HitResult {
    position: Vector3,
    normal: Vector3,
    distance: f32,
}

impl RayTracer {
    pub fn shade_pixel(&self, p: &Vector3, wo: &Vector3) -> f32 {
        // Contribute from the light source.
        // TODO: Uniformly sample the light at x' (pdf = 1 / A)
        let l_dir = 0.0f32;
        let (xp, xn) = self.light.random_point();


        // Contribute from other reflectors.
        let mut l_indir = 0.0f32;
        const P_RR: f32 = 0.8;
        let ksi = rand::thread_rng().gen_range(0.0..1.0f32);
        if ksi > P_RR {
            return 0.0;
        }

        // Random choose one direction wi~pdf(w)
        let wi = ONB::from(p).local(&random_direction());
        let fr: f32 = 0.0;
        let ray = Ray::new(p, &wi);
        let hit_result = self.get_nearest_intersection(&ray);
        if let Some(hit_result) = hit_result {
            let q = &hit_result.position;
            let cos_theta = {
                let d = q - p;
                let n = &hit_result.normal;
                n.dot(&d) / n.norm() * q.norm()
            };
            l_indir = self.shade_pixel(q, &-wi) * fr * cos_theta * 2.0 * PI / P_RR;
        }
        l_dir + l_indir
    }

    fn get_nearest_intersection(&self, ray: &Ray) -> Option<HitResult> {
        self._get_nearest_intersection(ray, &self.objects.root)
    }

    fn _get_nearest_intersection(&self, ray: &Ray, node: &BVHNode) -> Option<HitResult> {
        if !node.bounding_box.intersect_ray(ray) {
            return None;
        }

        // leaf
        if node.l.is_none() && node.r.is_none() {
            let nearest_result = node.data.as_ref().and_then(|triangles| {
                let nearest_result: Option<HitResult> = triangles
                    .iter()
                    .map(|triangle| {
                        ray.intersect_triangle(triangle).and_then(|barycenter| {
                            let position = Vector3::from(
                                &interpolate_triangle!(triangle, world_position; barycenter),
                            );
                            let normal = Vector3::from(&interpolate!(triangle, normal; barycenter)).normalized();
                            let distance = (&position - &ray.origin).norm();
                            return Some(HitResult {
                                position,
                                normal,
                                distance,
                            });
                        })
                    })
                    .fold(None, |acc, x| nearer_option_hitresult(acc, x));
                nearest_result
            });
            return nearest_result;
        }

        let left = node
            .l
            .as_ref()
            .and_then(|node| self._get_nearest_intersection(ray, node.as_ref()));
        let right = node
            .r
            .as_ref()
            .and_then(|node| self._get_nearest_intersection(ray, node.as_ref()));

        nearer_option_hitresult(left, right)
    }
}

fn nearer_option_hitresult(r1: Option<HitResult>, r2: Option<HitResult>) -> Option<HitResult> {
    match (&r1, &r2) {
        (Some(h1), Some(h2)) => {
            if h1.distance < h2.distance {
                r1
            } else {
                r2
            }
        }
        _ => r1.or(r2),
    }
}

fn random_direction() -> Vector3 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0f32);
    let r2 = rng.gen_range(0.0..1.0f32);
    let cos_theta = (1.0 - r2).sqrt();
    let sin_theta = r2.sqrt();
    let phi = 2.0 * PI * r1;

    let x = phi.cos() * sin_theta;
    let y = phi.sin() * sin_theta;
    let z = cos_theta;

    vector3([x, y, z]).normalized()
}

struct ONB {
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
}
impl From<&Vector3> for ONB {
    fn from(n: &Vector3) -> Self {
        let w = n.clone().normalized();
        let a = if w.x().abs() > 0.9 {
            vector3([0.0, 1.0, 0.0])
        } else {
            vector3([1.0, 0.0, 0.0])
        };
        let v = w.cross(&a).normalized();
        let u = w.cross(&v);
        Self { u, v, w }
    }
}
impl ONB {
    pub fn local(&self, a: &Vector3) -> Vector3 {
        &self.u * a.x() + &self.v * a.y() + &self.w * a.z()
    }
}

fn F(wi: &Vector3, h: &Vector3, ni1: f32, ni2: f32) -> f32 {
    fn r0(ni1: f32, ni2: f32) -> f32 {
        ((ni1 - ni2) / (ni1 + ni2)).powi(2)
    }
    let r0 = r0(ni1, ni2);
    r0 + (1.0 - r0) * (1.0 - wi.dot(h)).powi(5)
}

fn ggx(n: &Vector3, h: &Vector3, roughness: f32) -> f32 {
    roughness.powi(2) / (PI * (n.dot(h).powi(2) * (roughness.powi(2) - 1.0) + 1.0).powi(2))
}

#[derive(Clone, Copy)]
enum IlluminateType {
    Direct,
    IBL,
}
fn G(
    wi: &Vector3,
    wo: &Vector3,
    h: &Vector3,
    roughness: f32,
    illuminate_type: IlluminateType,
) -> f32 {
    fn g_schlick_ggx(
        n: &Vector3,
        v: &Vector3,
        roughness: f32,
        illuminate_type: IlluminateType,
    ) -> f32 {
        fn k_direct(roughness: f32) -> f32 {
            (roughness + 1.0).powi(2) / 8.0
        }
        fn k_ibl(roughness: f32) -> f32 {
            roughness.powi(2) / 2.0
        }
        let k = match illuminate_type {
            IlluminateType::Direct => k_direct(roughness),
            IlluminateType::IBL => k_ibl(roughness),
        };
        n.dot(v) / (n.dot(v) * (1.0 - k) + k)
    }
    g_schlick_ggx(h, wi, roughness, illuminate_type)
        * g_schlick_ggx(h, wo, roughness, illuminate_type)
}
