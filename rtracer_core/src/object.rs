use crate::prelude::*;

#[derive(Copy, Clone)]
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
