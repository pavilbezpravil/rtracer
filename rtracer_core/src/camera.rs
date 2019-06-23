use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct RaycastCamera {
    pub origin: Vec3,
    pub upper_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl RaycastCamera {
    pub fn from_camera(camera: &Camera) -> RaycastCamera {
        let theta = camera.vfov.to_radians();
        let half_height = (theta / 2.).tan();
        let half_width = camera.aspect * half_height;

        let w = (camera.lookfrom - camera.lookat).make_unit();
        let u = (camera.vup.cross(&w)).make_unit();
        let v = (w.cross(&u)).make_unit();

        let origin = camera.lookfrom;
        let upper_left = origin - half_width * u + half_height * v - w;
        let horizontal = 2. * half_width * u;
        let vertical = -2. * half_height * v;

        RaycastCamera { origin, upper_left, horizontal, vertical }
    }

    pub fn get_ray(&self, (u, v): (f32, f32)) -> Ray {
        debug_assert!(u >= 0f32 && u < 1.05f32);
        debug_assert!(v >= 0f32 && v < 1.05f32);
        Ray::new(self.origin, self.upper_left + u * self.horizontal + v * self.vertical - self.origin)
    }
}

pub struct Camera {
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub vfov: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        Camera { lookfrom, lookat, vup: vup.make_unit(), vfov, aspect }
    }

    pub fn translate(&mut self, trans: &Vec3) {
        self.lookfrom += *trans;
        self.lookat += *trans;
    }

    // TODO: add nalgebra
//    pub fn rotate_over_right(&mut self, angle: f32) {
//        let look = self.lookat - self.lookfrom;
//    }

    pub fn forward(&self) -> Vec3 {
        (self.lookat - self.lookfrom).make_unit()
    }

    pub fn backward(&self) -> Vec3 {
        -self.forward()
    }

    pub fn left(&self) -> Vec3 {
        self.vup.cross(&self.forward())
    }

    pub fn right(&self) -> Vec3 {
        -self.vup.cross(&self.forward())
    }
}