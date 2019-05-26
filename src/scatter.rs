use crate::{Ray, Vec3, HitRecord};

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Vec3,
}

impl ScatteredRay {
    pub fn new(ray: Ray, attenuation: Vec3) -> ScatteredRay {
        ScatteredRay { ray, attenuation }
    }
}

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay>;
}