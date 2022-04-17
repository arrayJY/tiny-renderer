use rand::Rng;
use std::f32::consts::PI;
use crate::{algebra::vector_new::{Vector4, vector4}, pipeline::model::Triangle, Color};

type FrameBuffer = Vec<Option<Color>>;

pub fn pbr_shade(width: usize, height: usize, triangles: Vec<Triangle>) -> FrameBuffer {
    let mut framebuffer: FrameBuffer = vec![None; width * height];

    framebuffer
}

fn shade_pixel(p: (usize, usize), wo: &Vector4) -> f32 {
    // Contribute from the light source.
    // TODO: Uniformly sample the light at x' (pdf = 1 / A)

    // Contribute from other reflectors.
    const P_RR: f32 = 0.5;
    let ksi = rand::thread_rng().gen_range(0.0..1.0f32);
    if ksi > P_RR { return 0.0; }


    todo!()
}

fn random_direction() -> Vector4 {
    let mut rng = rand::thread_rng();
    let r1 = rng.gen_range(0.0..1.0f32); 
    let r2 = rng.gen_range(0.0..1.0f32);
    let cos_theta = (1.0 - r2).sqrt();
    let sin_theta = r2.sqrt();
    let phi = 2.0 * PI * r1;

    let x = phi.cos() * sin_theta;
    let y = phi.sin() * sin_theta;
    let z = cos_theta;

    vector4([x, y, z, 0.0]).normalized()
}


fn F(wi: &Vector4, h: &Vector4, ni1: f32, ni2: f32) -> f32 {
    fn r0(ni1: f32, ni2: f32) -> f32 {
        ((ni1 - ni2) / (ni1 + ni2)).powi(2)
    }
    let r0 = r0(ni1, ni2);
    r0 + (1.0 - r0) * (1.0 - wi.dot(h)).powi(5)
}

fn ggx(n: &Vector4, h: &Vector4, roughness: f32) -> f32 {
    use std::f32::consts::PI;
    roughness.powi(2) / (PI * (n.dot(h).powi(2) * (roughness.powi(2) - 1.0) + 1.0).powi(2))
}

#[derive(Clone, Copy)]
enum IlluminateType {
    Direct,
    IBL,
}
fn G(wi: &Vector4, wo: &Vector4, h: &Vector4, roughness: f32, illuminate_type: IlluminateType) -> f32 {
    fn g_schlick_ggx(
        n: &Vector4,
        v: &Vector4,
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
