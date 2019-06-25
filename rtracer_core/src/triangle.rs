use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::Aabb;
use crate::intersect::Intersect;
use crate::intersection::ray_triangle_intersection;

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Triangle {
        Triangle { v0, v1, v2 }
    }

    pub fn normal(&self) -> Vec3 {
        (self.v1 - self.v0).cross(&(self.v2 - self.v0)).make_unit()
    }
}

impl Intersect for Triangle {
    fn intersect(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<f32> {
        if let Some(t) = ray_triangle_intersection(ray, self) {
            if t_min < t && t < t_max {
                return Some(t)
            }
        }

        None
    }

    fn aabb(&self) -> Aabb {
        unimplemented!()
    }
}
