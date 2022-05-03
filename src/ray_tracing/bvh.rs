use crate::algebra::vector_new::Vector4;
use crate::pipeline::model::Triangle;
use crate::ray_tracing::ray::Ray;

#[derive(Debug, Default, Clone)]
pub struct AABB {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
}
impl From<&[Triangle]> for AABB {
    fn from(triangles: &[Triangle]) -> Self {
        let mut x0 = std::f32::MAX;
        let mut x1 = std::f32::MIN;
        let mut y0 = std::f32::MAX;
        let mut y1 = std::f32::MIN;
        let mut z0 = std::f32::MAX;
        let mut z1 = std::f32::MIN;

        triangles.iter().for_each(|triangles| {
            triangles.vertexs.iter().for_each(|v| {
                x0 = x0.min(v.position.x());
                x1 = x1.max(v.position.x());
                y0 = y0.min(v.position.y());
                y1 = y1.max(v.position.y());
                z0 = z0.min(v.position.z());
                z1 = z1.max(v.position.z());
            })
        });
        Self {
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
        }
    }
}

impl AABB {
    pub fn union(&self, other: &Self) -> Self {
        Self {
            x0: self.x0.min(other.x0),
            x1: self.x1.max(other.x1),
            y0: self.y0.min(other.y0),
            y1: self.y1.max(other.y1),
            z0: self.z0.min(other.z0),
            z1: self.z1.max(other.z1),
        }
    }

    pub fn intersect_ray(&self, ray: &Ray) -> bool {
        let s = &ray.origin;
        let v = &ray.dir;
        let &Self {
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
        } = self;
        let (sx, sy, sz) = (s.x(), s.y(), s.z());
        let (vx, vy, vz) = (v.x(), v.y(), v.z());

        let tx_min = (x0 - sx) / vx;
        let tx_max = (x1 - sx) / vx;

        let ty_min = (y0 - sy) / vy;
        let ty_max = (y1 - sy) / vy;

        let tz_min = (z0 - sz) / vz;
        let tz_max = (z1 - sz) / vz;

        let (tx_min, tx_max) = if tx_min > tx_max {
            (tx_max, tx_min)
        } else {
            (tx_min, tx_max)
        };
        let (ty_min, ty_max) = if ty_min > ty_max {
            (ty_max, ty_min)
        } else {
            (ty_min, ty_max)
        };
        let (tz_min, tz_max) = if tz_min > tz_max {
            (tz_max, tz_min)
        } else {
            (tz_min, tz_max)
        };

        let t_enter = tx_min.max(ty_min).max(tz_min);
        let t_exit = tx_max.min(ty_max).min(tz_max);

        t_enter < t_exit && t_exit > 0.0
        // let tx0 = self.x0 - s.x()
    }
}

#[derive(Debug, Default, Clone)]
pub struct BVHNode {
    pub bounding_box: AABB,
    pub data: Option<Vec<Triangle>>,
    pub l: Option<Box<BVHNode>>,
    pub r: Option<Box<BVHNode>>,
}

pub struct BVHTree {
    pub root: BVHNode,
}

impl BVHTree {
    pub fn from_triangles(triangles: &[Triangle]) -> Self {
        let funcs: [fn(&Vector4) -> f32; 3] = [
            |v: &Vector4| v.x(),
            |v: &Vector4| v.y(),
            |v: &Vector4| v.z(),
        ];

        let mut barycenters: Vec<Vector4> = triangles
            .iter()
            .map(|triangle| triangle.get_barycenter())
            .collect();

        let mut node_indexs: Vec<Option<usize>> = vec![None; triangles.len() * 2 + 1 + 1];

        BVHTree::build_tree_leaves(
            &mut barycenters,
            &mut node_indexs,
            0,
            triangles.len() - 1,
            1,
            &funcs,
            0,
        );
        let mut tree = Self {
            root: BVHNode::default(),
        };
        BVHTree::build_tree(
            triangles,
            &mut node_indexs,
            &mut tree.root,
            1,
            0,
            triangles.len() - 1,
        );
        tree
    }

    fn build_tree(
        triangles: &[Triangle],
        node_indexs: &mut [Option<usize>],
        node: &mut BVHNode,
        i: usize,
        l: usize,
        r: usize,
    ) {
        if let Some(median) = node_indexs[i] {
            node.data = None;
            node.l = Some(Box::new(BVHNode::default()));
            node.r = Some(Box::new(BVHNode::default()));

            let left = node.l.as_mut().unwrap().as_mut();
            let right = node.r.as_mut().unwrap().as_mut();
            BVHTree::build_tree(triangles, node_indexs, left, i * 2, l, median);
            BVHTree::build_tree(triangles, node_indexs, right, i * 2 + 1, median + 1, r);
            node.bounding_box = left.bounding_box.union(&right.bounding_box);
        } else {
            let triangles = triangles[l..=r].to_vec();
            let bounding_box = AABB::from(triangles.as_slice());
            node.data = Some(triangles);
            node.bounding_box = bounding_box;
        }
    }

    fn build_tree_leaves(
        barycenters: &mut [Vector4],
        node_indexs: &mut [Option<usize>],
        p: usize,
        r: usize,
        i: usize,
        funcs: &[fn(&Vector4) -> f32],
        mut fi: usize,
    ) {
        if r - p < 5 {
            return;
        }
        let median_index = BVHTree::quick_select(barycenters, p, r, (r - p) / 2, funcs[fi]);
        node_indexs[i] = Some(median_index);
        fi = (fi + 1) % funcs.len();
        // Build left child tree.
        BVHTree::build_tree_leaves(barycenters, node_indexs, p, median_index, i * 2, funcs, fi);
        // Build right child tree.
        BVHTree::build_tree_leaves(
            barycenters,
            node_indexs,
            median_index,
            r,
            i * 2 + 1,
            funcs,
            fi,
        );
    }

    fn quick_select(
        barycenters: &mut [Vector4],
        p: usize,
        r: usize,
        i: usize,
        f: impl Fn(&Vector4) -> f32,
    ) -> usize {
        if p == r {
            return p;
        }
        let q = BVHTree::randomized_partition(barycenters, p, r, &f);
        let k = q - p + 1;

        if i == k {
            q
        } else if i < k {
            return BVHTree::quick_select(barycenters, p, q - 1, i, f);
        } else {
            return BVHTree::quick_select(barycenters, q + 1, r, i - k, f);
        }
    }

    fn randomized_partition(
        barycenters: &mut [Vector4],
        p: usize,
        r: usize,
        f: impl Fn(&Vector4) -> f32,
    ) -> usize {
        use rand::Rng;
        let i = rand::thread_rng().gen_range(p..r);
        barycenters.swap(i, r);
        return BVHTree::partition(barycenters, p, r, f);
    }

    fn partition(
        barycenters: &mut [Vector4],
        p: usize,
        r: usize,
        f: impl Fn(&Vector4) -> f32,
    ) -> usize {
        // let x =
        let mut i = p as isize - 1;
        for j in p..r {
            let m = f(&barycenters[j]);
            let n = f(&barycenters[r]);
            if m < n {
                i += 1;
                barycenters.swap(i as usize, j);
            }
        }
        barycenters.swap((i + 1) as usize, r);
        (i + 1) as usize
    }
}
