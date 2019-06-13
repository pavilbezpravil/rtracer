use rand::distributions::{UnitSphereSurface, Distribution};

use crate::Vec3;
use crate::{Scatter, ScatteredRay};
use crate::{Ray, HitRecord};
use rand::Rng;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatter for Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        match self {
            Material::Lambertian(m) => m.scatter(ray, hit),
            Material::Metal(m) => m.scatter(ray, hit),
            Material::Dielectric(m) => m.scatter(ray, hit),
        }
    }
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
        if Vec3::dot(&reflected, &hit.normal) > 0f32 {
            return Some(ScatteredRay::new(Ray::new(hit.point, reflected + self.fuzz * random_in_unit_sphere()), self.albedo));
        }
        None
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub attenuation: Vec3,
    pub ref_idx: f32,
}

impl Dielectric {
    pub fn new(attenuation: Vec3, ref_idx: f32) -> Dielectric {
        Dielectric { attenuation, ref_idx }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatteredRay> {
        let (outward_normal, ni_over_nt, cosin) = if Vec3::dot(&ray.direction, &hit.normal) > 0. {
            (-hit.normal, self.ref_idx, self.ref_idx * Vec3::dot(&ray.direction, &hit.normal) / ray.direction.length())
        } else {
            (hit.normal, 1. / self.ref_idx, -Vec3::dot(&ray.direction, &hit.normal) / ray.direction.length())
        };

        let reflected = reflect(&ray.direction, &hit.normal);

        let dir = if let Some(refracted) = refract(&ray.direction, &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosin, self.ref_idx);
            if rand::thread_rng().gen::<f32>() < reflect_prob {
                reflected
            } else {
                refracted
            }
        } else {
            reflected
        };

        Some(ScatteredRay::new(Ray::new(hit.point, dir), self.attenuation))
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let sphere = UnitSphereSurface::new();
    let [x, y, z] = sphere.sample(&mut rand::thread_rng());
    [x as f32, y as f32, z as f32].into()
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2f32 * Vec3::dot(v, n) * *n
}

fn refract(v: &Vec3, n: &Vec3, n_ref: f32) -> Option<Vec3> {
    debug_assert!(relative_eq!(n.squared_length(), 1., epsilon = std::f32::EPSILON *  4.));

    let uv = v.make_unit();
    let cos_in = Vec3::dot(&uv, n);
    let cos2_out = 1. - n_ref * n_ref * (1. - cos_in * cos_in);
    if cos2_out > 0. {
        Some(n_ref * (uv - *n * cos_in) - *n * cos2_out.sqrt())
    } else {
        None
    }
}

fn schlick(cosin: f32, ref_idx: f32) -> f32 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosin).powi(5)
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