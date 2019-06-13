#[macro_use]
extern crate approx;

mod image;
mod vec3;
mod ray;
mod hit;
mod aabb;
mod sphere;
mod plane;
mod cube;
mod shape;
mod hit_list;
mod camera;
mod scatter;
mod material;
mod intersection;

pub use image::{Image, ColorRGB};
pub use vec3::Vec3;
pub use ray::Ray;
pub use hit::{HitRecord, Hit};
pub use hit_list::HitList;
pub use aabb::Aabb;
pub use sphere::Sphere;
pub use plane::Plane;
pub use cube::Cube;
pub use shape::Shape;
pub use camera::Camera;
pub use scatter::{Scatter, ScatteredRay};
pub use material::{Material, Lambertian, Metal, Dielectric};
pub use intersection::{ray_box_intersection, ray_sphere_intersection};
