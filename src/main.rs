extern crate rtracer;

use rtracer::{Vec3, Image, ColorRGB};
use rtracer::Ray;
use rtracer::{Sphere, hit_sphere};

fn color(ray: &Ray, scene: &Sphere) -> ColorRGB {
    let sphere = scene;

    if hit_sphere(&sphere, &ray) {
        return ColorRGB::from_rgb(1f32, 0f32, 0f32)
    }

    let unit_direction = *ray.direction().clone().make_unit();
    let t = 0.5f32 * (unit_direction.y() + 1f32);
    (1f32 - t) * Vec3::new(1f32, 1f32, 1f32) + t * Vec3::new(0.5f32, 0.7f32, 1f32)
}

fn draw_sphere(img: &mut Image) {
    let (width, height) = (img.width(), img.height());

    let hl = Vec3::new(-2f32, 1f32, -1f32);
    let horizontal = 4f32 * Vec3::new_x();
    let vertical = -2f32 * Vec3::new_y();

    let camera_pos = Vec3::origin();

    let sphere = Sphere::new(&Vec3::new(0f32, 0f32, -1f32), 0.5f32);

    for j in (0..height).rev() {
        for i in 0..width {
            let (u, v) = (i as f32 / width as f32, j as f32 / height as f32);
            let ray = Ray::new(&camera_pos, &(hl + u * horizontal + v * vertical));

            let c = color(&ray, &sphere);

            img[(i, j)] = c;

        }
    }
}

fn main() {
    let (width, height) = (200, 100);
    let mut img = Image::new(width, height);

    draw_sphere(&mut img);

    img.print_as_ppm();
}
