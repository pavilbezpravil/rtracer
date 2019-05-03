use std::sync::Arc;

use crate::{Vec3, Scatter};
use crate::Ray;
use crate::{Hit, HitRecord};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Arc<dyn Scatter + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Scatter + Send + Sync>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        let sphere = self;
        let oc = ray.origin - sphere.center;

        let a = ray.direction.squared_length();
        let b = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - sphere.radius * sphere.radius;

        let discriminant = b * b - a * c;

        if discriminant < 0f32 {
            return None;
        }

        let discriminant_root = discriminant.sqrt();

        let t = (-b - discriminant_root) / a;
        if t < t_max && t > t_min {
            let point = ray.point_at_parameter(t);
            let normal = (point - sphere.center) / sphere.radius;
            return Some(HitRecord::new(t, point, normal, &*self.material));
        }

        let t = (-b + discriminant_root) / a;
        if t < t_max && t > t_min {
            let point = ray.point_at_parameter(t);
            let normal = (point - sphere.center) / sphere.radius;
            return Some(HitRecord::new(t, point, normal, &*self.material));
        }

        None
    }
}
