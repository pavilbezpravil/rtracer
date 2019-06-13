extern crate rayon;

use rayon::prelude::*;

use itertools::iproduct;

use crate::scene::Scene;
use crate::hit::Hit;
use rtracer_core::image::{Image, ColorRGB};
use rtracer_core::prelude::{Vec3, Ray, Object, Camera};
use crate::scatter::Scatter;

pub struct CPURenderer<T: Hit + Sync + Send> {
    rays_for_pixel: u32,
    max_ray_depth: u32,
    pub scene: Scene<T>,
}

impl<T: Hit + Sync + Send> CPURenderer<T> {
    pub fn new(rays_for_pixel: u32, max_ray_depth: u32) -> CPURenderer<T> {
        CPURenderer { rays_for_pixel, max_ray_depth, scene: Scene::default() }
    }

    pub fn render(&self, image: &mut Image, camera: &Camera) {
        let (width, height) = (image.width(), image.height());

        iproduct!((0..height), (0..width))
            .zip(image.buf_mut().iter_mut())
            .par_bridge()
            .for_each(|((y, x), pixel)| {
                let mut total_color = ColorRGB::origin();

                for _ in 0..self.rays_for_pixel {
                    let (u, v) = ((x as f32 + rand::random::<f32>()) / width as f32,
                                  (y as f32 + rand::random::<f32>()) / height as f32);
                    let ray = camera.get_ray((u, v));

                    total_color += self.color(&ray, 0);
                }
                total_color /= self.rays_for_pixel as f32;
                total_color = total_color.gamma_correction(2f32);

                *pixel = total_color;
            });
    }

    fn color(&self, ray: &Ray, depth: u32) -> ColorRGB {
        if let Some(rec) = self.scene.hit(ray, (1e-5, std::f32::MAX)) {
            if let Some(scattered) = rec.material.scatter(ray, &rec) {
                if depth < self.max_ray_depth {
                    return scattered.attenuation * self.color(&scattered.ray, depth + 1);
                }
            }
            Vec3::origin()
        } else {
            let unit_direction = ray.direction.make_unit();
            let t = 0.5f32 * (unit_direction.y() + 1f32);
            (1f32 - t) * Vec3::new(1f32, 1f32, 1f32) + t * Vec3::new(0.5f32, 0.7f32, 1f32)
        }
    }
}

fn normal_to_color(normal: &Vec3) -> Vec3 {
    (Vec3::new(1., 1., 1.) + *normal) * 0.5
}