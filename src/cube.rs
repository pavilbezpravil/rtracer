use crate::{Aabb, Vec3, Material};
use crate::{Hit, HitRecord};
use crate::Scatter;
use crate::Ray;
use crate::intersection::ray_box_intersection;
use std::sync::Arc;

pub struct Cube {
    aabb: Aabb,
    material: Arc<Material>,
}

impl Cube {
    pub fn new(center: Vec3, size: Vec3, material: Arc<Material>) -> Cube {
        Cube { aabb: Aabb::new_from_center_size(center, size), material }
    }

    pub fn new_from_aabb(aabb: Aabb, material: Arc<Material>) -> Cube {
        Cube { aabb, material }
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

impl Hit for Cube {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        debug_assert!(t_min >= 0.);

        if let Some((it_t_min, it_t_max)) = ray_box_intersection(ray, &self.aabb) {
            let t =
                if it_t_min > 0. {
                    it_t_min
                } else {
                    it_t_max
                };

            if t_min < t && t < t_max {
                let mut point = ray.point_at_parameter(t);
                let normal = self.normal_at(&point);
                point += normal * 50. * std::f32::EPSILON; // TODO:
                return Some(HitRecord::new(t, point, normal, &*self.material));
            }
        }

        None
    }
}
