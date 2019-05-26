use rand::distributions::{UnitSphereSurface, Distribution};

use crate::Vec3;
use crate::{Scatter, ScatteredRay};
use crate::{Ray, HitRecord};

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let target = hit.normal + random_in_unit_sphere();
        Some(ScatteredRay::new(Ray::new(hit.point, target), self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo, fuzz: fuzz.min(1.).max(0.) }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let reflected = reflect(&ray.direction, &hit.normal);
        if reflected.dot(&hit.normal) > 0f32 {
            return Some(ScatteredRay::new(Ray::new(hit.point, reflected + self.fuzz * random_in_unit_sphere()), self.albedo));
        }
        None
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let sphere = UnitSphereSurface::new();
    let [x, y, z] = sphere.sample(&mut rand::thread_rng());
    [x as f32, y as f32, z as f32].into()
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2f32 * v.dot(&n) * *n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflect() {
        let v = Vec3::new(1., 0., 0.);
        assert_eq!(-v, reflect(&v, &Vec3::new(1., 0., 0.)));
        assert_eq!(v, reflect(&v, &Vec3::new(0., 1., 0.)));
    }
}