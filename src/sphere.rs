use std::sync::Arc;

use crate::{Vec3, Scatter};
use crate::Ray;
use crate::{Hit, HitRecord};
use crate::material::Material;
use crate::intersection::ray_sphere_intersection;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<Material>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        debug_assert!(t_min >= 0.);

        if let Some(t) = ray_sphere_intersection(ray, self) {
            if t_min < t && t < t_max {
                let mut point = ray.point_at_parameter(t);
                let normal = ((point - self.center) / self.radius).make_unit();
                point += normal * 50. * std::f32::EPSILON; // TODO:
                return Some(HitRecord::new(t, point, normal, &*self.material))
            }
        }

        None
    }
}
