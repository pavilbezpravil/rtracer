use crate::prelude::*;

pub enum Shape {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Triangle(Triangle),
    Disk(Disk),
}

impl Intersect for Shape {
    fn intersect(&self, ray: &Ray, t_min_max: (f32, f32)) -> Option<f32> {
        match self {
            Shape::Plane(s) => s.intersect(ray, t_min_max),
            Shape::Sphere(s) => s.intersect(ray, t_min_max),
            Shape::Cube(s) => s.intersect(ray, t_min_max),
            Shape::Triangle(s) => s.intersect(ray, t_min_max),
            Shape::Disk(s) => s.intersect(ray, t_min_max),
        }
    }
}