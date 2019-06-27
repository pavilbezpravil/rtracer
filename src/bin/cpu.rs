#![feature(duration_float)]

extern crate rand;

extern crate rtracer_core;

use rtracer_core::prelude::*;
use rtracer_core::image::{Image};

use rtracer_cpu::prelude::*;

use rtracer_cpu::ext_image::{ImageBuffer, Rgba};

use rand::Rng;

const MAX_RAY_DEPTH: u32 = 64;
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

fn test_scene_dielectric((width, height): (u32, u32)) -> (HitableList<Object>, Camera) {
    let mut scene = HitableList::new();

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

fn test_scene_triangle((width, height): (u32, u32)) -> (HitableList<Object>, Camera) {
    let mut scene = HitableList::new();

    let size = 1.;

    scene.add(Object::new_triangle(Triangle::new(size * Vec3::new(-1.0, 0., 0.), size * Vec3::new(1., 0., 0.), size * Vec3::new(0., 1., 0.)),
                                   Material::Lambertian(Lambertian::new(Vec3::new(0.37, 0.9, 0.02)))));

    let camera = Camera::new(Vec3::new_z(), -Vec3::new_z(), Vec3::new_y(), 90., width as f32 / height as f32);

    (scene, camera)
}

fn test_scene_disk((width, height): (u32, u32)) -> (HitableList<Object>, Camera) {
    let mut scene = HitableList::new();

    scene.add(Object::new_disk(Disk::new(Plane::new(-Vec3::new_z(), Vec3::new_z()), 1.),
                               Material::Lambertian(Lambertian::new(Vec3::new(0.37, 0.9, 0.02)))));

    let camera = Camera::new(Vec3::new_z(), -Vec3::new_z(), Vec3::new_y(), 90., width as f32 / height as f32);

    (scene, camera)
}

fn gen_spheres_in_cube(n: usize, size: f32) -> Vec<Object> {
    let mut objs = vec![];

    let radius = size / n as f32 / 3.;
    for ix in 0..n {
        let x = ix as f32 * radius * 3. + radius;
        for iy in 0..n {
            let y = iy as f32 * radius * 3. + radius;
            for iz in 0..n {
                let z = iz as f32 * radius * 3. + radius;

                let center = Vec3::new(x, y, z);
//                let material = Material::Lambertian(Lambertian::new(Vec3::new(rand::thread_rng().gen(), rand::thread_rng().gen(), rand::thread_rng().gen())));
                let material = Material::Metal(Metal::new(Vec3::new(rand::thread_rng().gen(), rand::thread_rng().gen(), rand::thread_rng().gen()), 0.));

                objs.push(Object::new_sphere(Sphere::new(center, radius),
                                             material));
            }
        }
    }

    objs
}

fn gen_random_spheres(n: usize, size: f32) -> Vec<Object> {
    let mut objs = vec![];

    {
        let radius = size * 25.;
        objs.push(Object::new_sphere(Sphere::new(Vec3::new(size / 2., -radius, -size / 2.), radius),
                                     Material::Lambertian(Lambertian::new(Vec3::new(0.37, 0.9, 0.02)))));
    }

    for i in 0..n {
        let radius = 1. + rand::thread_rng().gen::<f32>();
        let center = Vec3::new(rand::thread_rng().gen::<f32>() * size, radius + rand::thread_rng().gen::<f32>() * size / 2., -rand::thread_rng().gen::<f32>() * size);
        let material = Material::Lambertian(Lambertian::new(Vec3::new(rand::thread_rng().gen(), rand::thread_rng().gen(), rand::thread_rng().gen())));

        objs.push(Object::new_sphere(Sphere::new(center, radius),
                                     material));
    }

    objs
}

fn test_scene_with_random_spheres((width, height): (u32, u32), n: usize, size: f32) -> HitableList<Object> {
    let mut list = HitableList::new();
//    gen_random_spheres(n, size).into_iter().for_each(|s| list.add(s));
    gen_spheres_in_cube(n, size).into_iter().for_each(|s| list.add(s));

    list
}

fn test_scene_with_random_spheres_bvh((width, height): (u32, u32), n: usize, size: f32) -> BvhNode<Object> {
    use std::time::Instant;
    let start = Instant::now();
    let bvh = BvhNode::build(gen_spheres_in_cube(n, size).as_mut_slice());
    println!("bhv construct: {} sec", start.elapsed().as_secs_f32());

    bvh
}

fn run() {
    let args: Vec<String> = std::env::args().collect();

//    let (width, height) = (1920, 1080);
    let (width, height) = (640, 480);
    let (width, height) = (200, 100);
    let mut img = Image::new(width, height);

    let n = 5;
    let size = 20f32;
    let camera = Camera::new(Vec3::new(1., size / 1.5, -1.), Vec3::new(size / 2., 0., size / 2.), Vec3::new_y(), 90., width as f32 / height as f32);
//    let scene = test_scene_with_random_spheres((width, height), n, size);
    let scene = test_scene_with_random_spheres_bvh((width, height), n, size);
//    let (scene, camera) = test_scene_dielectric((width, height));
//    let (scene, camera) = test_scene_triangle((width, height));
//    let (scene, camera) = test_scene_disk((width, height));

    let mut renderer = CPURenderer::new(RAYS_FOR_PIXEL, MAX_RAY_DEPTH);

    renderer.render(&mut img, &camera, &scene);

    img.write_ppm(&mut std::fs::File::create("outputs/image.ppm").unwrap());

//    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &img.iter()).unwrap();
//    image.save("image.png").unwrap();
}

fn main() {
    run();
}
