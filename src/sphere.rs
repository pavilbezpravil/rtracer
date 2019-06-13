use crate::Vec3;
use crate::Ray;
use crate::Intersect;
use crate::intersection::ray_sphere_intersection;

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
}
