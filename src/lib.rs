#[macro_use]
extern crate approx;

mod image;
mod vec3;
mod ray;
mod hit;
mod intersect;
mod aabb;
mod sphere;
mod plane;
mod cube;
mod triangle;
mod disk;
mod shape;
mod object;
mod hit_list;
mod camera;
mod scatter;
mod material;
mod intersection;

pub mod prelude;

pub use image::{Image, ColorRGB};
pub use vec3::Vec3;
pub use ray::Ray;
pub use hit::{HitRecord, Hit};
pub use intersect::Intersect;
pub use hit_list::HitList;
pub use aabb::Aabb;
pub use sphere::Sphere;
pub use plane::Plane;
pub use cube::Cube;
pub use triangle::Triangle;
pub use disk::Disk;
pub use shape::Shape;
pub use object::Object;
pub use camera::Camera;
pub use scatter::{Scatter, ScatteredRay};
pub use material::{Material, Lambertian, Metal, Dielectric};
pub use intersection::{ray_box_intersection, ray_sphere_intersection, ray_disk_intersection};
