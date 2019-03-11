use crate::Vec3;
use crate::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: &Vec3, radius: f32) -> Sphere {
        Sphere { center: *center, radius }
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_center(&mut self, center: &Vec3) {
        self.center = *center
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius
    }
}

pub fn hit_sphere(sphere: &Sphere, ray: &Ray) -> bool {
    let to_sphere = sphere.center - *ray.origin();
    let ray_direction = *ray.direction().clone().make_unit();

    let squared_dist = {
        let a = to_sphere.dot(&ray_direction);
        to_sphere.dot(&to_sphere) - a * a
    };

    squared_dist <= sphere.radius * sphere.radius
}