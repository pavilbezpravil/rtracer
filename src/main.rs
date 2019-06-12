extern crate rtracer;
extern crate rand;
extern crate rayon;

use std::sync::Arc;

use rayon::prelude::*;

use itertools::iproduct;

use rtracer::{Vec3, Image, ColorRGB, Camera, Lambertian, Metal, Dielectric};
use rtracer::Ray;
use rtracer::Hit;
use rtracer::HitList;
use rtracer::Sphere;

const MAX_RAY_DEPTH: u32 = 50;

fn color(ray: &Ray, scene: &HitList, depth: u32) -> ColorRGB {
    if let Some(rec) = scene.hit(ray, (0.00001f32, std::f32::MAX)) {
        if let Some(scattered) = rec.scatter.scatter(ray, &rec) {
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

fn draw_scene(img: &mut Image, scene: &HitList) {
    let (width, height) = (img.width(), img.height());

    let ul = Vec3::new(-2f32, 1f32, -1f32);
    let horizontal = 4f32 * Vec3::new_x();
    let vertical = -2f32 * Vec3::new_y();

    let camera_pos = Vec3::origin();
    let camera = Camera::new(camera_pos, ul, horizontal, vertical);

    let ns = 100u32;

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

fn run() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

//    let (width, height) = (1920, 1080);
//    let (width, height) = (640, 480);
    let (width, height) = (200, 100);
    let mut img = Image::new(width, height);

    let mut scene = HitList::new();

    let z = -1.2;
    let dist = 1.15;
    let floor_radius = 25.;

    // right
    scene.add(Box::new(Sphere::new(Vec3::new(dist, 0f32, z), 0.5f32,
                                   Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)))));
    // center
    scene.add(Box::new(Sphere::new(Vec3::new(0f32, 0f32, z), 0.5f32,
                                   Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))))));
    // left
    scene.add(Box::new(Sphere::new(Vec3::new(-dist, 0f32, z), 0.5f32,
                                   Arc::new(Dielectric::new(1.5)))));
    scene.add(Box::new(Sphere::new(Vec3::new(-dist, 0f32, z), -0.45f32,
                                   Arc::new(Dielectric::new(1.5)))));
    // flor
    scene.add(Box::new(Sphere::new(Vec3::new(0f32, -floor_radius - 0.5f32, -1f32), floor_radius,
                                   Arc::new(Metal::new(Vec3::new(0.1, 0.8, 0.1), 0.0)))));

    draw_scene(&mut img, &scene);

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
