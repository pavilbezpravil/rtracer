#[macro_use]
extern crate approx;

extern crate nalgebra as na;

pub mod prelude;

pub mod image;
pub mod intersection;

pub type Vec3 = na::Vector3<f32>;
pub type Vec2 = na::Vector2<f32>;
pub type Vec2i = na::Vector2<i32>;
pub type Vec2ui = na::Vector2<u32>;
pub type Mat3 = na::Matrix3<f32>;
pub type Mat4 = na::Matrix4<f32>;

//mod vec3;
mod ray;
mod intersect;
mod aabb;
mod sphere;
mod plane;
mod cube;
mod triangle;
mod disk;
mod primitive;
mod object;
mod camera;
mod material;
mod scene_data;
