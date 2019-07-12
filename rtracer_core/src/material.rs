use crate::Vec3;

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
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

impl From<Lambertian> for Material {
    fn from(l: Lambertian) -> Self {
        Material::Lambertian(l)
    }
}

impl From<Metal> for Material {
    fn from(m: Metal) -> Self {
        Material::Metal(m)
    }
}

impl From<Dielectric> for Material {
    fn from(d: Dielectric) -> Self {
        Material::Dielectric(d)
    }
}
