use crate::algebra::vector_new::VectorNew4;
use crate::pipeline::model::{Triangle};

use super::tree::Node;
use super::tree::Tree;

pub struct BVHTree;

type BVHTreeNode = Node<Option<Vec<Triangle>>>;

impl BVHTree {
    pub fn from_triangles(triangles: &[Triangle]) -> Tree<Option<Vec<Triangle>>> {
        let funcs: [fn(&VectorNew4) -> f32; 3] = [
            |v: &VectorNew4| v.x(),
            |v: &VectorNew4| v.y(),
            |v: &VectorNew4| v.z(),
        ];

        let mut barycenters: Vec<VectorNew4> = triangles
            .iter()
            .map(|triangle| triangle.get_barycenter())
            .collect();

        let mut node_indexs: Vec<Option<usize>> = vec![None; triangles.len()];

        BVHTree::build_tree_leaves(&mut barycenters, &mut node_indexs, 0, triangles.len(), 1, &funcs, 0);
        let mut tree = Tree {
            root: BVHTreeNode::default()
        };
        BVHTree::build_tree(triangles, &mut node_indexs, &mut tree.root, 1, 0, triangles.len());
        tree
    }

    fn build_tree(
        triangles: &[Triangle],
        node_indexs: &mut [Option<usize>],
        node: &mut BVHTreeNode,
        i: usize,
        l: usize,
        r: usize,
    ) {
        if let Some(median) = node_indexs[i] {
            node.val = None;
            node.l = Some(Box::new(BVHTreeNode::default()));
            node.r = Some(Box::new(BVHTreeNode::default()));

            let left = node.l.as_mut().unwrap().as_mut();
            let right= node.r.as_mut().unwrap().as_mut();
            BVHTree::build_tree(triangles, node_indexs, left, i * 2, l, median);
            BVHTree::build_tree(triangles, node_indexs, right, i * 2 + 1, median+ 1, r);
        } else {
            node.val = Some(triangles[l..r].to_vec())
        }
    }

    fn build_tree_leaves(
        barycenters: &mut [VectorNew4],
        node_indexs: &mut [Option<usize>],
        p: usize,
        r: usize,
        i: usize,
        funcs: &[fn(&VectorNew4) -> f32],
        mut fi: usize,
    ) {
        if r - p < 20 {
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
            p,
            median_index,
            i * 2 + 1,
            funcs,
            fi,
        );
    }

    fn quick_select(
        barycenters: &mut [VectorNew4],
        p: usize,
        r: usize,
        i: usize,
        f: impl Fn(&VectorNew4) -> f32,
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
        barycenters: &mut [VectorNew4],
        p: usize,
        r: usize,
        f: impl Fn(&VectorNew4) -> f32,
    ) -> usize {
        use rand::Rng;
        let i = rand::thread_rng().gen_range(p..r);
        barycenters.swap(i, r);
        return BVHTree::partition(barycenters, p, r, f);
    }

    fn partition(
        barycenters: &mut [VectorNew4],
        p: usize,
        r: usize,
        f: impl Fn(&VectorNew4) -> f32,
    ) -> usize {
        // let x = 
        let mut i = p - 1;
        for j in p..r - 1 {
            let m = f(&barycenters[j]);
            let n = f(&barycenters[r]);
            if m < n{
                i += 1;
                barycenters.swap(i, j);
            }
        }
        barycenters.swap(i + 1, r);
        i + 1
    }
}
