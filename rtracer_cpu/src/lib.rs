#[macro_use]
extern crate approx;

pub extern crate image as ext_image;

extern crate rtracer_core;

pub mod prelude;

mod hit;
mod hitable_list;
mod scatter;
mod renderer_cpu;
