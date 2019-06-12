extern crate rtracer;
extern crate rand;
extern crate rayon;

use std::sync::Arc;

use rayon::prelude::*;

use itertools::iproduct;

use rtracer::{Vec3, Image, ColorRGB, Camera, Lambertian, Metal, Dielectric, Material};
use rtracer::Ray;
use rtracer::Hit;
use rtracer::HitList;
use rtracer::Sphere;
use rtracer::Scatter;

const MAX_RAY_DEPTH: u32 = 64;
const RAYS_FOR_PIXEL: u32 = 128;

fn color(ray: &Ray, scene: &HitList, depth: u32) -> ColorRGB {
    if let Some(rec) = scene.hit(ray, (0.00001f32, std::f32::MAX)) {
        if let Some(scattered) = rec.material.scatter(ray, &rec) {
            if depth < MAX_RAY_DEPTH {
                return scattered.attenuation * color(&scattered.ray, scene, depth + 1);
            }
        }
        Vec3::origin()
    } else {
        let unit_direction = ray.direction.make_unit();
        let t = 0.5f32 * (unit_direction.y() + 1f32);
        (1f32 - t) * Vec3::new(1f32, 1f32, 1f32) + t * Vec3::new(0.5f32, 0.7f32, 1f32)
    }
}

fn draw_scene(img: &mut Image, scene: &HitList, camera: &Camera) {
    let (width, height) = (img.width(), img.height());

    let ns = RAYS_FOR_PIXEL;

    iproduct!((0..height), (0..width))
        .zip(img.buf_mut().iter_mut())
        .par_bridge()
        .for_each(|((y, x), pixel)| {
            let mut total_color = ColorRGB::origin();

            for _ in 0..ns {
                let (u, v) = ((x as f32 + rand::random::<f32>()) / width as f32,
                              (y as f32 + rand::random::<f32>()) / height as f32);
                let ray = camera.get_ray((u, v));

                total_color += color(&ray, &scene, 0);
            }
            total_color /= ns as f32;
            total_color = total_color.gamma_correction(2f32);

            *pixel = total_color;
        });
}

enum Error {
    ArgParse,
    Io(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

fn test_scene_dielectric((width, height): (u32, u32)) -> (HitList, Camera) {
    let mut scene = HitList::new();

    let z = -1.2;
    let dist = 1.15;
    let floor_radius = 50.;

    // right
    scene.add(Box::new(Sphere::new(Vec3::new(dist, 0f32, z), 0.5f32,
                                   Arc::new(Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0))))));
    // center
    scene.add(Box::new(Sphere::new(Vec3::new(0f32, 0f32, z), 0.5f32,
                                   Arc::new(Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))))));
    // left
    scene.add(Box::new(Sphere::new(Vec3::new(-dist, 0f32, z), 0.5f32,
                                   Arc::new(Material::Dielectric(Dielectric::new(1.5))))));
    scene.add(Box::new(Sphere::new(Vec3::new(-dist, 0f32, z), -0.45f32,
                                   Arc::new(Material::Dielectric(Dielectric::new(1.5))))));
    // flor
    scene.add(Box::new(Sphere::new(Vec3::new(0f32, -floor_radius - 0.5f32, -1f32), floor_radius,
                                   Arc::new(Material::Metal(Metal::new(Vec3::new(0.0, 0.6, 0.0), 0.))))));

    let camera = Camera::new(Vec3::new(-2., 0.,0.), -Vec3::new_z(), Vec3::new_y(), 90., width as f32 / height as f32);

    (scene, camera)
}

fn run() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

//    let (width, height) = (1920, 1080);
    let (width, height) = (640, 480);
//    let (width, height) = (200, 100);
    let mut img = Image::new(width, height);

    let (scene, camera) = test_scene_dielectric((width, height));

    draw_scene(&mut img, &scene, &camera);

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
