use crate::Vec3;
use crate::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    pub fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        let sphere = self;
        let oc = ray.origin - sphere.center;

        let a = ray.direction.squared_length();
        let b = ray.direction.dot(&oc);
        let c = oc.dot(&oc) - sphere.radius * sphere.radius;

        let discriminant = b * b -  a * c;

        if discriminant > 0f32 {
            let discriminant_root = discriminant.sqrt();

            let t = (-b - discriminant_root) / a;
            if t < t_max && t > t_min {
                let point = ray.point_at_parameter(t);
                let normal = (point - sphere.center) / sphere.radius;
                return Some(HitRecord::new(t, &point, &normal))
            }

            let t = (-b + discriminant_root) / a;
            if t < t_max && t > t_min {
                let point = ray.point_at_parameter(t);
                let normal = (point - sphere.center) / sphere.radius;
                return Some(HitRecord::new(t, &point, &normal))
            }

            None
        } else {
            None
        }
    }
}

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
