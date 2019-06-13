use crate::{Sphere, Hit, Triangle, Disk};
use crate::Plane;
use crate::Ray;
use crate::HitRecord;
use crate::cube::Cube;

pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Triangle(Triangle),
    Disk(Disk),
}

impl Hit for Shape {
    fn hit(&self, ray: &Ray, t_min_max: (f32, f32)) -> Option<HitRecord> {
        match self {
            Shape::Plane(s) => s.hit(ray, t_min_max),
            Shape::Sphere(s) => s.hit(ray, t_min_max),
            Shape::Cube(s) => s.hit(ray, t_min_max),
            Shape::Triangle(s) => s.hit(ray, t_min_max),
            Shape::Disk(s) => s.hit(ray, t_min_max),
        }
    }
}