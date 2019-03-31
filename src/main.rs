extern crate rtracer;

use rtracer::{Vec3, Image, ColorRGB};
use rtracer::Ray;
use rtracer::Sphere;

fn color(ray: &Ray, scene: &Sphere) -> ColorRGB {
    let sphere = scene;

    if let Some(hr) = sphere.hit(ray, (0.001f32, std::f32::MAX)) {
        return 0.5f32 * (hr.normal + Vec3::new(1f32, 1f32, 1f32));
    }

    let unit_direction = *ray.direction.clone().make_unit();
    let t = 0.5f32 * (unit_direction.y() + 1f32);
    (1f32 - t) * Vec3::new(1f32, 1f32, 1f32) + t * Vec3::new(0.5f32, 0.7f32, 1f32)
}

fn draw_sphere(img: &mut Image) {
    let (width, height) = (img.width(), img.height());

    let hl = Vec3::new(-2f32, 1f32, -1f32);
    let horizontal = 4f32 * Vec3::new_x();
    let vertical = -2f32 * Vec3::new_y();

    let camera_pos = Vec3::origin();

    let sphere = Sphere::new(Vec3::new(0f32, 0f32, -1f32), 0.5f32);

    for j in (0..height).rev() {
        for i in 0..width {
            let (u, v) = (i as f32 / width as f32, j as f32 / height as f32);
            let ray = Ray::new(camera_pos, hl + u * horizontal + v * vertical);

            let c = color(&ray, &sphere);

            img[(i, j)] = c;
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

    draw_sphere(&mut img);

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
