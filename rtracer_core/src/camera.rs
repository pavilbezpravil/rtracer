use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub upper_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).make_unit();
        let u = (vup.cross(&w)).make_unit();
        let v = (w.cross(&u)).make_unit();

        let origin = lookfrom;
        let upper_left = origin - half_width * u + half_height * v - w;
        let horizontal = 2. * half_width * u;
        let vertical = -2. * half_height * v;

        Camera { origin, upper_left, horizontal, vertical }
    }

    pub fn get_ray(&self, (u, v): (f32, f32)) -> Ray {
        debug_assert!(u >= 0f32 && u < 1.05f32);
        debug_assert!(v >= 0f32 && v < 1.05f32);
        Ray::new(self.origin, self.upper_left + u * self.horizontal + v * self.vertical - self.origin)
    }
}