extern crate rtracer;
extern crate rand;

use rand::distributions::{UnitSphereSurface, Distribution};

use rtracer::{Vec3, Image, ColorRGB, Camera};
use rtracer::Ray;
use rtracer::Hit;
use rtracer::HitList;
use rtracer::Sphere;

fn random_in_unit_sphere() -> Vec3 {
    let sphere = UnitSphereSurface::new();
    let ns = sphere.sample(&mut rand::thread_rng());;
    [ns[0] as f32, ns[1] as f32, ns[2] as f32].into()
}

fn color(ray: &Ray, scene: &HitList) -> ColorRGB {
    if let Some(rec) = scene.hit(ray, (0.00001f32, std::f32::MAX)) {
        let target = rec.normal + random_in_unit_sphere();
        return 0.5f32 * color(&Ray::new(rec.point, target), scene);
    }

    let unit_direction = *ray.direction.clone().make_unit();
    let t = 0.5f32 * (unit_direction.y() + 1f32);
    (1f32 - t) * Vec3::new(1f32, 1f32, 1f32) + t * Vec3::new(0.5f32, 0.7f32, 1f32)
}

fn draw_scene(img: &mut Image, scene: &HitList) {
    let (width, height) = (img.width(), img.height());

    let ul = Vec3::new(-2f32, 1f32, -1f32);
    let horizontal = 4f32 * Vec3::new_x();
    let vertical = -2f32 * Vec3::new_y();

    let camera_pos = Vec3::origin();
    let camera = Camera::new(camera_pos, ul, horizontal, vertical);

    let ns = 1000u32;

    for j in (0..height).rev() {
        for i in 0..width {
            let mut total_color = ColorRGB::origin();

            for _ in 0..ns {
                let (u, v) = ((i as f32 + rand::random::<f32>()) / width as f32,
                                        (j as f32 + rand::random::<f32>()) / height as f32);
                let ray = camera.get_ray((u, v));

                total_color += color(&ray, &scene);
            }
            total_color /= ns as f32;
            total_color = total_color.gamma_correction(2f32);

            img[(i, j)] = total_color;
        }
    }
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

    let (width, height) = (200, 100);
    let mut img = Image::new(width, height);

    let mut scene = HitList::new();
    scene.add( Box::new(Sphere::new(Vec3::new(0f32, 0f32, -1f32), 0.5f32)));
    scene.add( Box::new(Sphere::new(Vec3::new(-1f32, 0f32, -1f32), 0.5f32)));
    scene.add( Box::new(Sphere::new(Vec3::new(1f32, 0f32, -1f32), 0.5f32)));
    scene.add( Box::new(Sphere::new(Vec3::new(0f32, -100.5f32, -1f32), 100f32)));

    draw_scene(&mut img, &scene);

    match args.len() {
        1 => img.write_ppm(&mut std::io::stdout().lock())?,
        2 => img.write_ppm(&mut std::fs::File::create(&args[1])?)?,
        _ => return Err(Error::ArgParse),
    };

    Ok(())
}

fn main() {
    let exit_code = match run() {
        Ok(_) => 0,
        Err(err) => {
            match err {
                Error::ArgParse => {
                    eprintln!("wrong number params, expect 1 or 2.");
                    1
                },
                Error::Io(err) => {
                    eprintln!("{}", err);
                    2
                },
            }
        }
    };

    std::process::exit(exit_code);
}
