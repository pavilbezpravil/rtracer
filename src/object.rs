use crate::{Shape, Material, Cube, Sphere, Plane, Triangle, Disk};
use crate::Ray;
use crate::{Hit, HitRecord};
use crate::intersect::Intersect;

pub struct Object {
    pub shape: Shape,
    pub material: Material,
}

impl Object {
    pub fn new(shape: Shape, material: Material) -> Object {
        Object { shape, material }
    }

    pub fn new_sphere(sphere: Sphere, material: Material) -> Object {
        Object::new(Shape::Sphere(sphere), material)
    }

    pub fn new_plane(plane: Plane, material: Material) -> Object {
        Object::new(Shape::Plane(plane), material)
    }

    pub fn new_cube(cube: Cube, material: Material) -> Object {
        Object::new(Shape::Cube(cube), material)
    }

    pub fn new_triangle(triangle: Triangle, material: Material) -> Object {
        Object::new(Shape::Triangle(triangle), material)
    }

    pub fn new_disk(disk: Disk, material: Material) -> Object {
        Object::new(Shape::Disk(disk), material)
    }
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
