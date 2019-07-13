use rtracer_core::prelude::*;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct BvhItem {
    pub aabb: Aabb,
    pub id: u32,
}

pub enum BvhNodeData {
    Leaf(BvhItem),
    Node {
        left: u32,
    },
}

pub struct BvhNode {
    data: BvhNodeData,
    escape: u32,
    aabb: Aabb,
}

impl BvhNode {
    pub fn build(nodes: &mut Vec<BvhNode>, objs: &mut [BvhItem], escape: u32) -> u32 {
        let sort_axis = rand::thread_rng().gen_range(0, 3);
        objs.sort_by(|a, b| a.aabb.min[sort_axis].partial_cmp(&b.aabb.min[sort_axis]).unwrap());

        let n = objs.len();
        assert!(n != 0, "cant build bvh from zero objects");

        if n == 1 {
            nodes.push(BvhNode { data: BvhNodeData::Leaf(objs[0]), aabb: objs[0].aabb, escape });
        } else {
            let (mut l_objs, mut r_objs) = objs.split_at_mut(n / 2);

            let right = BvhNode::build(nodes, &mut r_objs, escape);
            let left = BvhNode::build(nodes, &mut l_objs, right);

            let aabb = Aabb::union(&nodes[left as usize].aabb, &nodes[right as usize].aabb);

            let data = BvhNodeData::Node { left };
            nodes.push(BvhNode { data, aabb, escape });
        }
        (nodes.len() - 1) as u32
    }
}

pub struct Bvh {
    nodes: Vec<BvhNode>,
}

impl Bvh {
    pub fn build(objs: &mut [BvhItem]) -> Bvh {
        let mut nodes = vec![];
        BvhNode::build(&mut nodes, objs, std::u32::MAX);

        Bvh { nodes }
    }

    pub fn to_gpu(&self) -> Vec<f32> {
        let mut data = vec![];

        let mut cur_node = 0;
        while cur_node != std::u32::MAX as usize {
            let node = &self.nodes[cur_node];

            data.extend(&node_to_gpu(node));

            match node.data {
                BvhNodeData::Leaf(item)=> {
                    cur_node = node.escape as usize;
                },
                BvhNodeData::Node { left } => {
                    cur_node = left as usize;
                },
            }
        }

        data
    }
}

fn node_to_gpu(node: &BvhNode) -> [f32; 8] {
    match node.data {
        BvhNodeData::Leaf(item) => {
            [
                0., 0., 0., (std::u32::MAX as f32),
                0., 0., 0., item.id as f32,
            ]
        },
        BvhNodeData::Node { left } => {
            let aabb = node.aabb;
            [
                aabb.min[0], aabb.min[1], aabb.min[2], left as f32,
                aabb.max[0], aabb.max[1], aabb.max[2], node.escape as f32,
            ]
        },
    }
}

