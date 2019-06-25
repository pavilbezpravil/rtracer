use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::aabb::Aabb;
use crate::intersect::Intersect;
use crate::intersection::ray_plane_intersection;

#[derive(Copy, Clone)]
pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(origin: Vec3, normal: Vec3) -> Plane {
        debug_assert!(relative_eq!(normal.squared_length(), 1., epsilon = std::f32::EPSILON *  4.));
        Plane { origin, normal }
    }
}

impl Intersect for Plane {
    fn intersect(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<f32> {
        if let Some(t) = ray_plane_intersection(ray, self) {
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
