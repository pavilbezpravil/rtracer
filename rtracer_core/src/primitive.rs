use crate::prelude::*;

#[derive(Copy, Clone)]
pub enum Primitive {
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Triangle(Triangle),
    Disk(Disk),
}

impl Intersect for Primitive {
    fn intersect(&self, ray: &Ray, t_min_max: (f32, f32)) -> Option<f32> {
        match self {
            Primitive::Plane(s) => s.intersect(ray, t_min_max),
            Primitive::Sphere(s) => s.intersect(ray, t_min_max),
            Primitive::Cube(s) => s.intersect(ray, t_min_max),
            Primitive::Triangle(s) => s.intersect(ray, t_min_max),
            Primitive::Disk(s) => s.intersect(ray, t_min_max),
        }
    }

    fn aabb(&self) -> Aabb {
        match self {
            Primitive::Plane(s) => s.aabb(),
            Primitive::Sphere(s) => s.aabb(),
            Primitive::Cube(s) => s.aabb(),
            Primitive::Triangle(s) => s.aabb(),
            Primitive::Disk(s) => s.aabb(),
        }
    }
}