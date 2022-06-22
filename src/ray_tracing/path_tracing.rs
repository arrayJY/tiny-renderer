use crate::{
    algebra::vector_new::{vector3, Vector3},
    interpolate, interpolate_triangle,
    pipeline::{
        material::{IlluminateType, MaterialNew, PBRMaterial},
        model::{Model, Triangle, TriangulatedModel},
    },
    ray_tracing::ray::Ray,
    renderer::triangulated_models_and_triangles,
    window::pbr_window::PBRWindow,
    Color,
};
use rand::Rng;
use std::{f32::consts::PI, sync::Arc};

use super::bvh::{BVHNode, BVHTree, AABB};

pub struct RayTracer {
    pub objects_tree: BVHTree,
    pub objects: Vec<TriangulatedModel>,
    pub bounding_boxes: Vec<AABB>,
    pub framebuffer: Vec<Vector3>,
    pub shaded_count: usize,
    pub width: usize,
    pub height: usize,
    pub spp: usize,
    pub background_color: Vector3,
}

impl RayTracer {
    pub fn new(
        width: usize,
        height: usize,
        triangles: Vec<Triangle>,
        objects: Vec<TriangulatedModel>,
        spp: usize,
    ) -> Self {
        let objects_tree = BVHTree::new();
        // let objects_tree = BVHTree::from_triangles(&triangles.as_slice());
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
            spp,
            background_color: vector3([0.27, 0.27, 0.27]),
        };
        ray_tracer
    }

    pub fn pixel_to_ray(&self, x: usize, y: usize) -> Ray {
        const FOV: f32 = PI / 4.0;
        let scale: f32 = (FOV / 2.0).tan();

        let (width, height) = (self.width as f32, self.height as f32);
        let (x, y) = (x as f32, y as f32);
        let aspect_radio = width / height;

        let x = (2.0 * (x) / width - 1.0) * scale * aspect_radio;
        let y = (1.0 - 2.0 * (y) / height) * scale;

        let dir = vector3([x, y, -1.0]).normalized();
        let origin_z = self.width as f32 * 1.7;
        let origin = vector3([0.0, self.width as f32 / 2.0, origin_z]);

        Ray { origin, dir }
    }

    pub fn frame_buffer<'a>(&self) -> Vec<u32> {
        self.framebuffer
            .iter()
            .map(|v| (&Color::from(v)).into())
            .collect()
    }

    pub fn render(path: &str, spp: usize) {
        use indicatif::{ProgressBar, ProgressStyle};
        const WIDTH: usize = 800;
        const HEIGHT: usize = 800;
        let models = Model::from_gltf(path);
        let (objects, triangles) = triangulated_models_and_triangles(&models, (WIDTH / 2) as f32);
        let mut ray_tracer = RayTracer::new(WIDTH, HEIGHT, triangles, objects, spp);

        println!("Rendering {}, {}x{}, {} spp...\n", path, WIDTH, HEIGHT, spp);
        const CPU_NUM: usize = 16;
        const LINE: usize = HEIGHT / CPU_NUM;
        let mut framebuffer = vec![Vector3::new(); WIDTH * HEIGHT];
        let multi_bar = indicatif::MultiProgress::new();
        let progress_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-");

        std::thread::scope(|scope| {
            let multi_bar = &multi_bar;
            let ray_tracer = &ray_tracer;
            framebuffer
                .chunks_mut(WIDTH * HEIGHT / CPU_NUM)
                .enumerate()
                .for_each(|(i, s)| {
                    let pb = multi_bar.add(ProgressBar::new((s.len() * spp) as u64));
                    pb.set_style(progress_style.clone());
                    scope.spawn(move || {
                        pb.set_message(format!("thread #{}", i + 1));
                        for _ in 0..spp {
                            let start = i * LINE;
                            let pixel_iter = (start..start + LINE)
                                .flat_map(move |a| (0..WIDTH).map(move |b| (a, b)));
                            let mut count = 0;
                            s.iter_mut().zip(pixel_iter).for_each(|(p, (y, x))| {
                                let ray = ray_tracer.pixel_to_ray(x, y);
                                *p += ray_tracer.shade(&ray, 0) / spp as f32;
                                count += 1;
                                if count % WIDTH == 0 {
                                    pb.inc(WIDTH as u64);
                                }
                            })
                        }
                        pb.finish_with_message("done");
                    });
                });
            multi_bar.join().unwrap();
        });

        ray_tracer.framebuffer = framebuffer;

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
    pub material: Option<Arc<MaterialNew>>,
}

impl HitResult {
    pub fn material_eval(
        &self,
        wi: &Vector3,
        wo: &Vector3,
        n: &Vector3,
        illum_type: IlluminateType,
    ) -> Vector3 {
        self.pbr_material().eval(wi, wo, n, illum_type)
    }

    pub fn pbr_material(&self) -> &PBRMaterial {
        match self.material.as_ref().unwrap().as_ref() {
            MaterialNew::PBR(m) => m,
            _ => panic!("Only accept PBRMaterial"),
        }
    }
}

impl RayTracer {
    pub fn shade(&self, ray: &Ray, depth: usize) -> Vector3 {
        let intersection = self.get_nearest_intersection(ray);

        if let Some(intersection) = intersection {
            if intersection.emit.is_some() {
                if depth > 0 {
                    return vector3([0.0, 0.0, 0.0]);
                }
                return vector3([1.0, 1.0, 1.0]);
            }

            let wo = -&ray.dir;
            let p = &intersection.position;
            let n = &intersection.normal;

            // Direct light
            let mut l_dir = Vector3::new();
            if let Some((inter, pdf_light)) = self.sample_light() {
                let x = &inter.position;
                let nn = &inter.normal;
                let ws = &(x - p).normalized();

                let rray = Ray::new(p, &ws);

                if let Some(nearest_inter) = self.get_nearest_intersection(&rray) {
                    // Light not be blocked
                    if (&nearest_inter.position - x).norm() < 0.0001 {
                        let cos_theta0 = ws.dot(&n).abs();
                        let cos_theta1 = (-ws).dot(&nn).abs();
                        let fr = intersection.material_eval(&ws, &wo, n, IlluminateType::Direct);
                        if let Some(li) = inter.emit {
                            if cos_theta0 * cos_theta0 > 0.0 {
                                let per = fr * cos_theta0 * cos_theta1 /  ((x - p).norm().powi(2) * pdf_light);
                                let per = per.clamp_max(1.0);
                                l_dir = li.cwise_product(&per)
                                // println!("{:?}", l_dir);
                            }
                        }
                    }
                }
            }

            // Indirect light
            let mut l_indir = Vector3::new();
            const P_RR: f32 = 0.9;
            let ksi = rand::thread_rng().gen_range(0.0..=1.0f32);
            if ksi < P_RR {
                let m = intersection.pbr_material();
                let wi = m.sample(&wo, n);
                let ray = Ray::new(p, &wi);
                let fr = m.eval(&wi, &wo, n, IlluminateType::IBL);
                let cos_theta = wi.dot(&n).abs();
                let pdf_bsdf = m.pdf(&wi, &wo, n);
                if pdf_bsdf > 0.0 {
                    let fr = fr * cos_theta / (pdf_bsdf * P_RR);
                    l_indir = self.shade(&ray, depth + 1).cwise_product(&fr);
                }
            }
            let r = l_dir + l_indir;
            return r;
        }
        return self.background_color.clone();
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
                    if distance > 0.001 {
                        let material = triangle.material.clone();
                        return Some(HitResult {
                            position,
                            normal,
                            distance,
                            emit: material.as_ref().and_then(|m| {
                                m.emissive_material()
                                    .and_then(|e| Some(&e.base_color * e.intensity))
                            }),
                            material,
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
                            return Some(HitResult {
                                position,
                                normal,
                                distance,
                                emit: None,
                                material: triangle.material.clone(),
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
            if h1.distance < 0.0001 {
                return r2;
            }
            if h1.distance < h2.distance {
                r1
            } else {
                r2
            }
        }
        _ => r1.or(r2),
    }
}
