use crate::Vec3;
use crate::Ray;

pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

impl HitRecord {
    pub fn new(t: f32, point: &Vec3, normal: &Vec3) -> HitRecord {
        HitRecord { t, point: *point, normal: *normal }
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min_max: (f32, f32)) -> Option<HitRecord>;
}