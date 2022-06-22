use crate::algebra::vector_new::{vector3, Vector3};
use gltf::Material as GLTFMaterial;
use rand::Rng;
use std::f32::consts::PI;
use tobj::Material as ObjMaterial;

#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Diffuse,
    Microfacet,
}

#[derive(Debug, Clone)]
pub struct PhongMaterial {
    pub ambient_color: Vector3,
    pub diffuse_color: Vector3,
    pub specular_color: Vector3,
}

#[derive(Debug, Clone)]
pub struct PBRMaterial {
    pub albedo: Vector3,
    pub metalness: f32,
    pub roughness: f32,
    pub refraction: f32,
    f0: Vector3,
}

#[derive(Debug, Clone)]
pub struct EmissiveMaterial {
    pub base_color: Vector3,
    pub intensity: f32,
}

#[derive(Debug, Clone)]
pub struct OptionEmissiveMaterial(pub Option<EmissiveMaterial>);

#[derive(Debug, Clone)]
pub enum MaterialNew {
    Phong(PhongMaterial),
    PBR(PBRMaterial),
    Emissive(EmissiveMaterial),
}

impl MaterialNew {
    pub fn pbr_material(&self) -> Option<&PBRMaterial> {
        if let MaterialNew::PBR(m) = self {
            Some(m)
        } else {
            None
        }
    }

    pub fn emissive_material(&self) -> Option<&EmissiveMaterial> {
        if let MaterialNew::Emissive(m) = self {
            Some(m)
        } else {
            None
        }
    }

    pub fn phong_material(&self) -> Option<&PhongMaterial> {
        if let MaterialNew::Phong(m) = self {
            Some(m)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum IlluminateType {
    Direct,
    IBL,
}

impl From<&ObjMaterial> for PhongMaterial {
    fn from(
        ObjMaterial {
            ambient,
            diffuse,
            specular,
            ..
        }: &ObjMaterial,
    ) -> Self {
        Self {
            ambient_color: Vector3::from(ambient),
            diffuse_color: Vector3::from(diffuse),
            specular_color: Vector3::from(specular),
        }
    }
}

impl<'a> From<&GLTFMaterial<'a>> for PBRMaterial {
    fn from(material: &GLTFMaterial<'a>) -> Self {
        const DEFAULT_REFRACTION: f32 = 1.5;
        let pbr = material.pbr_metallic_roughness();
        let albedo = vector3(pbr.base_color_factor()[..3].try_into().unwrap());
        let metalness = pbr.metallic_factor();
        let roughness = pbr.roughness_factor().clamp(0.01, 1.0);
        let refraction = material.ior().unwrap_or(DEFAULT_REFRACTION);
        Self {
            f0: pbr_material_f0(refraction, &albedo, metalness),
            metalness,
            roughness,
            refraction,
            albedo,
        }
    }
}

impl PBRMaterial {
    fn fresnel_reflection(&self, wo: &Vector3, h: &Vector3) -> Vector3 {
        let f0 = &self.f0;
        let r = f0 + &((&vector3([1.0, 1.0, 1.0]) - f0) * (1.0 - wo.dot(h).abs()).powi(5));
        r
    }
    fn normal_distribution(&self, n: &Vector3, h: &Vector3) -> f32 {
        self.ggx(n, h)
    }

    fn geometry(&self, wi: &Vector3, wo: &Vector3, h: &Vector3, illum_type: IlluminateType) -> f32 {
        self.g_schlick_ggx(h, wi, illum_type) * self.g_schlick_ggx(h, wo, illum_type)
        /*
        let h_dot_wi = h.dot(wi);
        let h_dot_wo = h.dot(wo);
        let a = (2.0 *  h_dot_wi * h_dot_wo).abs();
        let b = (h.dot(wi) * h.dot(wo)).abs();
        let t = self.roughness;
        let denom = a * (1.0 - t) + b * t;
        2.0 / denom
        */
    }

    fn ggx(&self, n: &Vector3, h: &Vector3) -> f32 {
        let a2 = self.roughness.powi(2);
        let n_dot_h_2 = n.dot(h).powi(2);
        let nom = a2;
        let denom = n_dot_h_2 * (a2 - 1.0) + 1.0;
        let denom = PI * denom.powi(2);
        nom / denom
    }

    fn g_schlick_ggx(&self, n: &Vector3, v: &Vector3, illum_type: IlluminateType) -> f32 {
        let roughness = self.roughness;
        let k = match illum_type {
            IlluminateType::Direct => (roughness.powi(2) + 1.0).powi(2) / 8.0,
            IlluminateType::IBL => roughness.powi(4) / 2.0,
        };
        let n_dot_v = n.dot(v).abs();
        n_dot_v / (n_dot_v * (1.0 - k) + k)
    }
}

impl PBRMaterial {
    pub fn eval(
        &self,
        wi: &Vector3,
        wo: &Vector3,
        n: &Vector3,
        illum_type: IlluminateType,
    ) -> Vector3 {
        let check_ray_dir = n.dot(wi) * n.dot(wo);
        if check_ray_dir <= 0.0 {
            return Vector3::from(0.0);
        }

        let h = &(wi + wo).normalized();
        let f = &self.fresnel_reflection(wo, h);
        let d = self.normal_distribution(n, h);
        let g = self.geometry(wi, wo, h, illum_type);

        let cook_torrance_specular = f * g * d / (4.0 * wi.dot(n) * wo.dot(n)).abs();
        let kd = (&Vector3::from(1.0) - f) * (1.0 - self.metalness);
        let ks = f;
        let lambert_diffuse = &self.albedo / PI;
        kd.cwise_product(&lambert_diffuse) + ks.cwise_product(&cook_torrance_specular)
        // &self.albedo / PI
    }

    pub fn pdf(&self, wi: &Vector3, wo: &Vector3, n: &Vector3) -> f32 {
        if n.dot(wi) * n.dot(wo) < 0.0 {
            return 0.0;
        }

        // const DIFFUSE: f32 = 0.5 / PI;
        // let kd = Vector3::from(1.0) - self.fresnel_reflection(wi, h);
        // let kd = kd * DIFFUSE;
        let h = &(wi + wo).normalized();
        let cos_theta = h.dot(n).abs();
        let h_dot_wo = h.dot(wo);
        let ph = self.normal_distribution(n, h) * cos_theta;
        let po = if h_dot_wo == 0.0 {
            0.0
        } else {
            1.0 / (4.0 * h_dot_wo.abs())
        };
        ph * po
        // return Vector3::from(DIFFUSE);
        // Vector3::from(ks) + kd
    }

    pub fn sample(&self, wi: &Vector3, n: &Vector3) -> Vector3 {
        // self._diffse_sample(n)
        self.importance_sample_ggx(wi, n)
    }

    fn _diffse_sample(&self, n: &Vector3) -> Vector3 {
        let mut rng = rand::thread_rng();
        let x1 = rng.gen_range(0.0f32..1.0);
        let x2 = rng.gen_range(0.0f32..1.0);
        let z = (1.0 - 2.0 * x1).abs();
        let r = (1.0 - z * z).sqrt();
        let phi = 2.0 * PI * x2;
        let locay_ray = vector3([r * phi.cos(), r * phi.sin(), z]);
        Self::to_world(&locay_ray, n)
    }

    fn importance_sample_ggx(&self, wi: &Vector3, n: &Vector3) -> Vector3 {
        let mut rng = rand::thread_rng();
        let x1 = rng.gen_range(0.0f32..1.0);
        let x2 = rng.gen_range(0.0f32..1.0);
        let a2 = self.roughness.powi(4);
        let phi = 2.0 * PI * x1;
        let cos_theta = ((1.0 - x2) / (1.0 + (a2 - 1.0) * x2)).sqrt();
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let x = phi.cos() * sin_theta;
        let y = phi.sin() * sin_theta;
        let z = cos_theta;

        let h = Self::to_world(&vector3([x, y, z]).normalized(), n);
        let l = &(&h * 2.0 * wi.dot(&h)) - wi;
        l.normalized()
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

fn mix(v0: &Vector3, v1: &Vector3, alpha: f32) -> Vector3 {
    v0 * (1.0 - alpha) + v1 * alpha
}

fn pbr_material_f0(refraction: f32, albedo: &Vector3, metalness: f32) -> Vector3 {
    const AIR_REFRACTION: f32 = 1.0;
    let f0 = ((AIR_REFRACTION - refraction) / (AIR_REFRACTION + refraction)).powi(2);
    let f0 = &Vector3::from(f0);
    mix(f0, albedo, metalness)
}

impl<'a> From<&GLTFMaterial<'a>> for OptionEmissiveMaterial {
    fn from(material: &GLTFMaterial<'a>) -> Self {
        let intensity = material
            .extras()
            .as_ref()
            .and_then(|extras| {
                use gltf::json::{deserialize, Value};
                deserialize::from_str::<Value>(extras.get())
                    .map_or_else(|_| None, |value| Some(value))
            })
            .as_ref()
            .and_then(|extras| extras.as_object())
            .and_then(|obj| obj.get("intensity"))
            .and_then(|v| v.as_f64())
            .and_then(|v| Some(v as f32));

        Self(intensity.and_then(|intensity| {
            Some(EmissiveMaterial {
                intensity,
                base_color: Vector3::from(&material.emissive_factor()),
            })
        }))
    }
}

/*
impl From<&ObjMaterial> for Material {
    fn from(material: &ObjMaterial) -> Self {
        let emit = emit_from_material(material);
        let material_type =
            material
                .illumination_model
                .map_or(MaterialType::Diffuse, |illum_model| match illum_model {
                    2 => MaterialType::Microfacet,
                    _ => MaterialType::Diffuse,
                });

        let &ObjMaterial {
            ambient,
            diffuse,
            specular,
            optical_density,
            shininess,
            dissolve,
            ..
        } = material;

        const AIR_OPTICAL_DENSITY: f32 = 1.0;
        let roughness = cal_roughness(shininess);

        Self {
            ambient_color: vector3(ambient.clone()),
            diffuse_color: vector3(diffuse.clone()),
            specular_color: vector3(specular.clone()),
            shininess,
            optical_density,
            dissolve,
            emit,
            material_type,
            f0: if roughness < 0.2 {
                Vector3::from(&diffuse)
            } else {
                vector3([0.04, 0.04, 0.04])
            },
            roughness,
        }
    }
}
*/
