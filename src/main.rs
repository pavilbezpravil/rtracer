extern crate rand;

extern crate rtracer_core;

use rtracer_core::prelude::*;
use rtracer_core::image::{Image, ColorRGB};

use rtracer_cpu::prelude::*;

const MAX_RAY_DEPTH: u32 = 32;
const RAYS_FOR_PIXEL: u32 = 32;

enum Error {
    ArgParse,
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

fn test_scene_dielectric((width, height): (u32, u32)) -> (Scene<Object>, Camera) {
    let mut scene = Scene::new();

    let z = -1.2;
    let dist = 1.15;

    // right
    scene.add(Object::new_sphere(Sphere::new(Vec3::new(dist, 0f32, z), 0.5f32),
                                 Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0))));
    // center
    scene.add(Object::new_sphere(Sphere::new(Vec3::new(0f32, 0f32, z), 0.5f32),
                                 Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))));
    // left
    scene.add(Object::new_sphere(Sphere::new(Vec3::new(-dist, 0f32, z), 0.5f32),
                                 Material::Dielectric(Dielectric::new(Vec3::unit(), 1.5))));
    scene.add(Object::new_sphere(Sphere::new(Vec3::new(-dist, 0f32, z), -0.45f32),
                                 Material::Dielectric(Dielectric::new(Vec3::unit(), 1.5))));

    // flor
    scene.add(Object::new_plane(Plane::new(-1. * Vec3::new_y(), Vec3::new_y()),
                                Material::Lambertian(Lambertian::new(Vec3::new(0.0, 0.6, 0.0)))));
//                                               Arc::new(Material::Metal(Metal::new(Vec3::new(0.0, 0.6, 0.0), 0.)))))));

    // big cube
    scene.add(Object::new_cube(Cube::new(Vec3::new(4.0, 0., 0.), 4. * Vec3::unit()),
                               Material::Metal(Metal::new(Vec3::new(0.37, 0.15, 0.02), 0.))));

    let camera = Camera::new(Vec3::new(-2., 0.75, 0.25), -Vec3::new_z(), Vec3::new_y(), 90., width as f32 / height as f32);

    (scene, camera)
}

fn test_scene_triangle((width, height): (u32, u32)) -> (Scene<Object>, Camera) {
    let mut scene = Scene::new();

    let size = 1.;

    scene.add(Object::new_triangle(Triangle::new(size * Vec3::new(-1.0, 0., 0.), size * Vec3::new(1., 0., 0.), size * Vec3::new(0., 1., 0.)),
                                   Material::Lambertian(Lambertian::new(Vec3::new(0.37, 0.9, 0.02)))));

    let camera = Camera::new(Vec3::new_z(), -Vec3::new_z(), Vec3::new_y(), 90., width as f32 / height as f32);

    (scene, camera)
}

fn test_scene_disk((width, height): (u32, u32)) -> (Scene<Object>, Camera) {
    let mut scene = Scene::new();

    scene.add(Object::new_disk(Disk::new(Plane::new(-Vec3::new_z(), Vec3::new_z()), 1.),
                               Material::Lambertian(Lambertian::new(Vec3::new(0.37, 0.9, 0.02)))));

    let camera = Camera::new(Vec3::new_z(), -Vec3::new_z(), Vec3::new_y(), 90., width as f32 / height as f32);

    (scene, camera)
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

//    let (width, height) = (1920, 1080);
//    let (width, height) = (640, 480);
    let (width, height) = (200, 100);
    let mut img = Image::new(width, height);

    let (scene, camera) = test_scene_dielectric((width, height));
//    let (scene, camera) = test_scene_triangle((width, height));
//    let (scene, camera) = test_scene_disk((width, height));

    let mut renderer = CPURenderer::new(RAYS_FOR_PIXEL, MAX_RAY_DEPTH);
    renderer.scene = scene;

    renderer.render(&mut img, &camera);

    Ok(match args.len() {
        1 => img.write_ppm(&mut std::io::stdout().lock())?,
        2 => img.write_ppm(&mut std::fs::File::create(&args[1])?)?,
        _ => return Err(Error::ArgParse),
    })
}

fn main() {
    let exit_code = match run() {
        Ok(_) => 0,
        Err(Error::ArgParse) => {
            eprintln!("wrong number params, expect 1 or 2.");
            1
        }
        Err(Error::Io(err)) => {
            eprintln!("{}", err);
            2
        }
    };

    std::process::exit(exit_code);
}
