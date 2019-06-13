use std::sync::Arc;

use crate::{Vec3, Scatter, Plane};
use crate::Ray;
use crate::{Hit, HitRecord};
use crate::material::Material;
use crate::intersection::ray_disk_intersection;

pub struct Disk {
    pub plane: Plane,
    pub radius: f32,
}

impl Disk {
    pub fn new(plane: Plane, radius: f32) -> Disk {
        Disk { plane, radius}
    }
}

impl Hit for Disk {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        if let Some(t) = ray_disk_intersection(ray, self) {
            if t_min < t && t < t_max {
                let point = ray.point_at_parameter(t);
                return Some(HitRecord::new(t, point, self.plane.normal, &*self.plane.material))
            }
        }

        None
    }
}
