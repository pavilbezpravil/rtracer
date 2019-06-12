use std::sync::Arc;

use crate::{Vec3, Scatter};
use crate::Ray;
use crate::{Hit, HitRecord};
use crate::material::Material;

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, material: Arc<Material>) -> Plane {
        debug_assert!(relative_eq!(normal.squared_length(), 1., epsilon = std::f32::EPSILON *  4.));
        Plane { point, normal, material }
    }
}

impl Hit for Plane {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        let ray_perpendicular_component = Vec3::dot(&self.normal, &ray.direction);

        let t =
            if ray_perpendicular_component.abs() < 2. * std::f32::EPSILON {
                std::f32::MAX
            } else {
                let perpendicular_traverse_distance = Vec3::dot(&self.normal, &(self.point - ray.origin));
                perpendicular_traverse_distance / ray_perpendicular_component
            };

        if t_min < t && t < t_max {
            let point = ray.point_at_parameter(t);
            Some(HitRecord::new(t, point, self.normal, &*self.material))
        } else {
            None
        }
    }
}
