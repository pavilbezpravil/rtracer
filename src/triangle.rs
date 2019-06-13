use std::sync::Arc;

use crate::{Vec3, Scatter};
use crate::Ray;
use crate::{Hit, HitRecord};
use crate::material::Material;
use crate::intersection::ray_triangle_intersection;

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub material: Arc<Material>,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Arc<Material>) -> Triangle {
        Triangle { v0, v1, v2, material }
    }

    pub fn normal(&self) -> Vec3 {
        (self.v1 - self.v0).cross(&(self.v2 - self.v0)).make_unit()
    }
}

impl Hit for Triangle {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        if let Some(t) = ray_triangle_intersection(ray, self) {
            if t_min < t && t < t_max {
                let point = ray.point_at_parameter(t);
                return Some(HitRecord::new(t, point, self.normal(), &*self.material))
            }
        }

        None
    }
}
