use crate::prelude::*;
use rtracer_core::prelude::*;
use rand::Rng;

pub enum BvhNodeData<H: Hit + Copy + Clone> {
    Leaf(H),
    Node {
        left: Box<BvhNode<H>>,
        right: Box<BvhNode<H>>,
    },
}

pub struct BvhNode<H: Hit + Copy + Clone> {
    data: BvhNodeData<H>,
    aabb: Aabb,
}

impl<H: Hit + Copy + Clone> BvhNode<H> {
    pub fn build(objs: &mut [H]) -> BvhNode<H> {
        let sort_axis = rand::thread_rng().gen_range(0, 3);
        objs.sort_by(|a, b| a.aabb().min[sort_axis].partial_cmp(&b.aabb().min[sort_axis]).unwrap());

        let n = objs.len();
        assert!(n != 0, "cant build bvh from zero objects");

        let (left, right): (Box<H>, Box<H>);

        if n == 1 {
            BvhNode { data: BvhNodeData::Leaf(objs[0]), aabb: objs[0].aabb() }
        } else {
            let (mut l_objs, mut r_objs) = objs.split_at_mut(n / 2);

            let left = Box::new(BvhNode::build(&mut l_objs));
            let right = Box::new(BvhNode::build(&mut r_objs));
            let aabb = Aabb::union(&left.aabb(), &right.aabb());

            let data = BvhNodeData::Node { left, right };
            BvhNode { data, aabb }
        }
    }
}

impl<H: Hit + Copy + Clone> Hit for BvhNode<H> {
    fn hit(&self, ray: &Ray, t_min_max: (f32, f32)) -> Option<HitRecord> {
        if self.aabb().intersect(ray, t_min_max).is_none() {
            return None
        }

        match &self.data {
            BvhNodeData::Leaf(leaf) => {
                leaf.hit(ray, t_min_max)
            },
            BvhNodeData::Node { left, right} => {
                let l_hit = left.hit(ray, t_min_max);
                let r_hit = right.hit(ray, t_min_max);

                match (l_hit, r_hit) {
                    (Some(l_hit), Some(r_hit)) => {
                        if l_hit.t < r_hit.t {
                            Some(l_hit)
                        } else {
                            Some(r_hit)
                        }
                    },
                    (Some(l_hit), None) => {
                        Some(l_hit)
                    },
                    (None, Some(r_hit)) => {
                        Some(r_hit)
                    },
                    _ => None
                }
            },
        }
    }

    fn aabb(&self) -> Aabb {
        self.aabb
    }
}
