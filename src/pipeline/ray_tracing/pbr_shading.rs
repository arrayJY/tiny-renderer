use crate::{
    algebra::vector_new::{vector3, Vector3},
    interpolate, interpolate_triangle,
    pipeline::{
        model::{Material, Model, Triangle, TriangulatedModel},
        ray_tracing::ray::Ray,
    },
    renderer::triangulated_models_and_triangles,
    window::pbr_window::PBRWindow,
    Color,
};
use rand::Rng;
use std::{f32::consts::PI, rc::Rc};

use super::data_structure::bvh::{BVHNode, BVHTree, AABB};

pub struct RayTracer {
    pub objects_tree: BVHTree,
    pub objects: Vec<TriangulatedModel>,
    pub bounding_boxes: Vec<AABB>,
    pub framebuffer: Vec<Vector3>,
    pub shaded_count: usize,
    pub width: usize,
    pub height: usize,
    pub pixel_iter: Box<dyn Iterator<Item = (usize, usize)>>,
    pub ssp: usize,
}

impl RayTracer {
    pub fn new(
        width: usize,
        height: usize,
        triangles: Vec<Triangle>,
        objects: Vec<TriangulatedModel>,
        ssp: usize,
    ) -> Self {
        let objects_tree = BVHTree::from_triangles(&triangles.as_slice());
        let pixel_iter = Box::new(
            (0..width)
                .flat_map(move |a| (0..height).map(move |b| (a, b)))
                .cycle(),
        );
        let bounding_boxes = objects
            .iter()
            .map(|model| AABB::from(&model.triangles[..]))
            .collect::<Vec<_>>();
        let ray_tracer = Self {
            objects,
            objects_tree,
            bounding_boxes,
            framebuffer: vec![Vector3::new(); width * height],
            shaded_count: 0,
            width,
            height,
            pixel_iter,
            ssp,
        };
        ray_tracer
    }

    pub fn pixel_to_ray(&self, x: usize, y: usize) -> Ray {
        const FOV: f32 = PI / 2.0;
        let scale: f32 = (FOV / 2.0).tan();

        let (width, height) = (self.width as f32, self.height as f32);
        let (x, y) = (x as f32, y as f32);
        let aspect_radio = width / height;

        let x = (2.0 * (x) / width - 1.0) * scale * aspect_radio;
        let y = (1.0 - 2.0 * (y) / height) * scale;

        let dir = vector3([x, y, -1.0]).normalized();
        let origin_z = self.width as f32;
        let origin = vector3([0.0, 0.0, origin_z]);

        Ray { origin, dir }
    }

    pub fn frame_buffer<'a>(&self) -> Vec<u32> {
        self.framebuffer
            .iter()
            .map(|v| (&Color::from(v)).into())
            .collect()
    }

    pub fn shade_next_pixel(&mut self) {
        let count = self.shaded_count;
        let total = self.framebuffer.len() * self.ssp;
        if count > total {
            return;
        }

        if let Some((x, y)) = self.pixel_iter.next() {
            let ray = self.pixel_to_ray(x, y);
            let color = &self.shade(&ray, 0) / self.ssp as f32;
            let index = y * self.width + x;
            *self.framebuffer.get_mut(index).unwrap() += color;
            self.shaded_count += 1;
        }
    }

    pub fn render(path: &str, ssp: usize) {
        use indicatif::{ProgressBar, ProgressStyle};
        const WIDTH: usize = 800;
        const HEIGHT: usize = 800;
        let models = Model::from_obj(path);
        let (objects, triangles) = triangulated_models_and_triangles(&models, (WIDTH / 2) as f32);
        let mut ray_tracer = RayTracer::new(WIDTH, HEIGHT, triangles, objects, ssp);
        let shade_times = WIDTH * HEIGHT * ssp;
        let bar = ProgressBar::new(shade_times as u64);

        let style_string = format!("Rendering {}, {}x{}, {} ssp...\n", path, WIDTH, HEIGHT, ssp);
        let style_string = format!(
            "{} {}",
            style_string, "[{elapsed_precise}] {bar} {percent}%"
        );

        bar.set_style(ProgressStyle::default_bar().template(&style_string));
        for _ in 0..shade_times {
            ray_tracer.shade_next_pixel();
            bar.inc(1);
        }
        bar.finish();
        let mut window = PBRWindow::new(WIDTH, HEIGHT);
        window.run(ray_tracer)
    }
}

#[derive(Debug, Clone)]
pub struct HitResult {
    pub position: Vector3,
    pub normal: Vector3,
    pub distance: f32,
    pub emit: Option<Vector3>,
    pub material: Option<Rc<Material>>,
}

impl HitResult {
    pub fn material_eval(&self, wi: &Vector3, wo: &Vector3, n: &Vector3) -> Vector3 {
        self.material.as_ref().unwrap().eval(wi, wo, n)
    }
}

impl RayTracer {
    pub fn shade(&self, ray: &Ray, depth: usize) -> Vector3 {
        let intersection = self.get_nearest_intersection(ray);

        if let Some(intersection) = intersection {
            if let Some(emit) = intersection.emit {
                return vector3([1.0, 1.0, 1.0]);
                // return emit / intersection.distance.powf(2.0);
            }
            // return Vector3::new();

            let wo = -&ray.dir;
            let p = &intersection.position;
            let n = &intersection.normal;

            // Direct light
            let mut l_dir = Vector3::new();
            if let Some((inter, pdf_light)) = self.sample_light() {
                let x = &inter.position;
                let nn = &inter.normal;
                let ws = (x - p).normalized();

                let rray = Ray::new(p, &ws);

                if let Some(nearest_inter) = self.get_nearest_intersection(&rray) {
                    // Light not be blocked
                    if (&nearest_inter.position - x).norm() < 0.01 {
                        let cos_theta0 = ws.dot(&n);
                        let cos_theta1 = (-&ws).dot(&nn);
                        let fr = intersection.material_eval(&wo, &ws, n);

                        if let Some(li) = inter.emit {
                            l_dir = li.cwise_product(&fr) * cos_theta0 * cos_theta1
                                / (x - p).norm().powf(2.0)
                                / pdf_light;
                        }
                    }
                }
            }

            // Indirect light
            let mut l_indir = Vector3::new();
            const P_RR: f32 = 0.9;
            let ksi = rand::thread_rng().gen_range(0.0..=1.0f32);
            if ksi < P_RR {
                let m = intersection.material.clone().unwrap();
                let wi = m.sample(&wo, n);
                let ray = Ray::new(p, &wi);
                let fr = m.eval(&wi, &wo, n);
                let cos_theta = wi.dot(&n);
                let pdf_hemi = m.pdf(&wi, &wo, n);
                l_indir =
                    self.shade(&ray, depth + 1).cwise_product(&fr) * cos_theta / (pdf_hemi * P_RR);
            }
            return l_dir + l_indir;
        }
        return Vector3::new();
    }

    fn sample_light(&self) -> Option<(HitResult, f32)> {
        let emit_area = self.objects.iter().fold(0.0f32, |acc, model| {
            acc + if model.has_emit() { model.area() } else { 0.0 }
        });
        let p = rand::thread_rng().gen_range(0.0f32..=1.0) * emit_area;
        let mut emit_area = 0.0;
        let mut sample_result = None;
        for model in self.objects.iter() {
            if model.has_emit() {
                emit_area += model.area();
                if p <= emit_area {
                    sample_result = Some(model.sample());
                    break;
                }
            }
        }
        sample_result
    }

    fn get_nearest_intersection(&self, ray: &Ray) -> Option<HitResult> {
        self.objects
            .iter()
            .flat_map(|t| t.triangles.iter())
            .map(|triangle| {
                ray.intersect_triangle(triangle).and_then(|barycenter| {
                    let position =
                        Vector3::from(&interpolate_triangle!(triangle, position; barycenter));
                    let normal =
                        Vector3::from(&interpolate!(triangle, normal; barycenter)).normalized();
                    let distance = (&position - &ray.origin).norm();
                    if distance > 0.1 {
                        let material = triangle.material.clone();
                        return Some(HitResult {
                            position,
                            normal,
                            distance,
                            emit: material.as_ref().and_then(|m| m.emit.clone()),
                            material 
                        });
                    } else {
                        return None;
                    }
                })
            })
            .fold(None, |acc, x| nearer_option_hitresult(acc, x))
        // self._get_nearest_intersection(ray, &self.objects_tree.root)
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
                                &interpolate_triangle!(triangle, position; barycenter),
                            );
                            let normal = Vector3::from(&interpolate!(triangle, normal; barycenter))
                                .normalized();
                            let distance = (&position - &ray.origin).norm();
                            if distance > 0.1 {
                                return Some(HitResult {
                                    position,
                                    normal,
                                    distance,
                                    emit: None,
                                    material: triangle.material.clone(),
                                });
                            } else {
                                return None;
                            }
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

/*
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
*/
