use std::sync::Arc;

use crate::{Vec3, Scatter};
use crate::Ray;
use crate::{Hit, HitRecord};
use crate::material::Material;
use crate::intersection::ray_plane_intersection;

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
}

impl Plane {
    pub fn new(origin: Vec3, normal: Vec3, material: Arc<Material>) -> Plane {
        debug_assert!(relative_eq!(normal.squared_length(), 1., epsilon = std::f32::EPSILON *  4.));
        Plane { origin, normal, material }
    }
}

impl Hit for Plane {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        let t = ray_plane_intersection(ray, self);

        if t_min < t && t < t_max {
            let point = ray.point_at_parameter(t);
            Some(HitRecord::new(t, point, self.normal, &*self.material))
        } else {
            None
        }
    }
}
