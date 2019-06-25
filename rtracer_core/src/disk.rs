use crate::plane::Plane;
use crate::ray:: Ray;
use crate::aabb::Aabb;
use crate::intersect::Intersect;
use crate::intersection::ray_disk_intersection;

#[derive(Copy, Clone, Debug)]
pub struct Disk {
    pub plane: Plane,
    pub radius: f32,
}

impl Disk {
    pub fn new(plane: Plane, radius: f32) -> Disk {
        Disk { plane, radius}
    }
}

impl Intersect for Disk {
    fn intersect(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<f32> {
        if let Some(t) = ray_disk_intersection(ray, self) {
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