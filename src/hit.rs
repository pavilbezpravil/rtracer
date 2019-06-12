use crate::{Vec3, Scatter};
use crate::Ray;

pub struct HitRecord<'a> {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub scatter: &'a dyn Scatter,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, point: Vec3, normal: Vec3, scatter: &Scatter) -> HitRecord {
        debug_assert!(relative_eq!(normal.squared_length(), 1., epsilon = std::f32::EPSILON *  4.));
        HitRecord { t, point, normal, scatter }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min_max: (f32, f32)) -> Option<HitRecord>;
}