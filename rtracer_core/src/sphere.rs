use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::Aabb;
use crate::intersect::Intersect;
use crate::intersection::ray_sphere_intersection;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn normal_at(&self, point: &Vec3) -> Vec3 {
        (*point - self.center).make_unit()
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<f32> {
        if let Some(t) = ray_sphere_intersection(ray, self) {
            if t_min < t && t < t_max {
                return Some(t)
            }
        }

        None
    }

    fn aabb(&self) -> Aabb {
        Aabb::from_center_size(self.center, Vec3::from_value(self.radius * 2.))
    }
}
