use crate::{Sphere, Hit};
use crate::Plane;
use crate::Ray;
use crate::HitRecord;

pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
}

impl Hit for Shape {
    fn hit(&self, ray: &Ray, t_min_max: (f32, f32)) -> Option<HitRecord> {
        match self {
            Shape::Plane(s) => s.hit(ray, t_min_max),
            Shape::Sphere(s) => s.hit(ray, t_min_max),
        }
    }
}