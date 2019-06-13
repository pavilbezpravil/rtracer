use crate::{Aabb, Vec3};
use crate::Intersect;
use crate::Ray;
use crate::intersection::ray_box_intersection;

pub struct Cube {
    aabb: Aabb,
}

impl Cube {
    pub fn new(center: Vec3, size: Vec3) -> Cube {
        Cube { aabb: Aabb::new_from_center_size(center, size) }
    }

    pub fn new_from_aabb(aabb: Aabb) -> Cube {
        Cube { aabb }
    }

    pub fn center(&self) -> Vec3 {
        self.aabb.center()
    }

    pub fn size(&self) -> Vec3 {
        self.aabb.size()
    }

    pub fn normal_at(&self, point: &Vec3) -> Vec3 {
        self.aabb.normal_at(point)
    }
}

impl Intersect for Cube {
    fn intersect(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<f32> {
        if let Some((it_t_min, it_t_max)) = ray_box_intersection(ray, &self.aabb) {
            let t =
                if it_t_min > 0. {
                    it_t_min
                } else {
                    it_t_max
                };

            if t_min < t && t < t_max {
                return Some(t);
            }
        }

        None
    }
}
