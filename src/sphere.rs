use std::sync::Arc;

use crate::{Vec3, Scatter};
use crate::Ray;
use crate::{Hit, HitRecord};
use crate::material::Material;

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
        let sphere = self;
        let oc = ray.origin - sphere.center;

        let a = ray.direction.squared_length();
        let b = Vec3::dot(&ray.direction, &oc);
        let c = Vec3::dot(&oc, &oc) - sphere.radius * sphere.radius;

        let discriminant = b * b - a * c;

        if discriminant < 0f32 {
            return None;
        }

        let discriminant_root = discriminant.sqrt();

        let t1 = (-b - discriminant_root) / a;
        let t2 = (-b + discriminant_root) / a;

        let t = if t1 < t_max && t1 > t_min {
            t1
        } else if t2 < t_max && t2 > t_min {
            t2
        } else {
            return None;
        };

        let mut point = ray.point_at_parameter(t);
        let normal = ((point - sphere.center) / sphere.radius).make_unit();
        point += normal * 50. * std::f32::EPSILON; // TODO:
        Some(HitRecord::new(t, point, normal, &*self.material))
    }
}
