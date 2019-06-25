use crate::prelude::*;

#[derive(Copy, Clone)]
pub struct Object {
    pub primitive: Primitive,
    pub material: Material,
}

impl Object {
    pub fn new(primitive: Primitive, material: Material) -> Object {
        Object { primitive, material }
    }

    pub fn new_sphere(sphere: Sphere, material: Material) -> Object {
        Object::new(Primitive::Sphere(sphere), material)
    }

    pub fn new_plane(plane: Plane, material: Material) -> Object {
        Object::new(Primitive::Plane(plane), material)
    }

    pub fn new_cube(cube: Cube, material: Material) -> Object {
        Object::new(Primitive::Cube(cube), material)
    }

    pub fn new_triangle(triangle: Triangle, material: Material) -> Object {
        Object::new(Primitive::Triangle(triangle), material)
    }

    pub fn new_disk(disk: Disk, material: Material) -> Object {
        Object::new(Primitive::Disk(disk), material)
    }
}
