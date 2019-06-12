use crate::{Vec3, Scatter};
use crate::Ray;
use crate::material::Material;

pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl HitRecord {
    pub fn new(t: f32, point: Vec3, normal: Vec3, material: &Material) -> HitRecord {
        debug_assert!(relative_eq!(normal.squared_length(), 1., epsilon = std::f32::EPSILON *  4.));
        HitRecord { t, point, normal, material: material.clone() }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min_max: (f32, f32)) -> Option<HitRecord>;
}