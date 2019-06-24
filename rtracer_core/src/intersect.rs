use crate::ray::Ray;
use crate::aabb::Aabb;

pub trait Intersect {
    fn intersect(&self, ray: &Ray, t_min_max: (f32, f32)) -> Option<f32>;
    fn aabb(&self) -> Aabb;
}