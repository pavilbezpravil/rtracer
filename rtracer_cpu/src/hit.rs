use rtracer_core::prelude::*;

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

impl Hit for Object {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        if let Some(t) = self.shape.intersect(ray, (t_min, t_max)) {
            let point = ray.point_at_parameter(t);

            let normal = match &self.shape {
                Shape::Sphere(s) => s.normal_at(&point),
                Shape::Plane(s) => s.normal,
                Shape::Cube(s) => s.normal_at(&point),
                Shape::Triangle(s) => s.normal(),
                Shape::Disk(s) => s.plane.normal,
            };

            Some(HitRecord::new(t, point, normal, &self.material))
        } else {
            None
        }
    }
}