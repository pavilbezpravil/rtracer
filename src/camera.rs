use crate::Vec3;
use crate::Ray;

pub struct Camera {
    pub origin: Vec3,
    pub upper_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, upper_left: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
        Camera { origin, upper_left, horizontal, vertical }
    }

    pub fn get_ray(&self, (u, v): (f32, f32)) -> Ray {
        debug_assert!(u >= 0f32 && u < 1.05f32);
        debug_assert!(v >= 0f32 && v < 1.05f32);
        Ray::new(self.origin, self.upper_left + u * self.horizontal + v * self.vertical)
    }
}