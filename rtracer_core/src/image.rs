use std::ops::{Index, IndexMut};
use std::io::Write;

use crate::vec3::Vec3;

pub type ColorRGB = Vec3;

impl ColorRGB {
    pub fn from_rgb(r: f32, g: f32, b: f32) -> ColorRGB {
        ColorRGB::new(r, g, b)
    }

    pub fn r(&self) -> f32 {
        self.x()
    }

    pub fn g(&self) -> f32 {
        self.y()
    }

    pub fn b(&self) -> f32 {
        self.z()
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        debug_assert!(self.r() >= 0f32 && self.r() <= 1f32);
        debug_assert!(self.g() >= 0f32 && self.g() <= 1f32);
        debug_assert!(self.b() >= 0f32 && self.b() <= 1f32);

        ((self.r() * 255.99f32) as u8,
         (self.g() * 255.99f32) as u8,
         (self.b() * 255.99f32) as u8)
    }

    pub fn gamma_correction(&self, gamma: f32) -> Vec3 {
        Vec3::new(self.x().powf(1. / gamma),
                  self.y().powf(1. / gamma),
                  self.z().powf(1. / gamma))
    }
}

pub struct Image {
    width: u32,
    height: u32,
    // TODO: заменить на ColorRGB<u8>
    img: Vec<ColorRGB>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image::with_background(width, height, ColorRGB::new(0f32, 0f32, 0f32))
    }

    pub fn with_background(width: u32, height: u32, c: ColorRGB) -> Image {
        Image { width, height, img: vec![c; (width * height) as usize] }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn buf_mut(&mut self) -> &mut Vec<ColorRGB> {
        &mut self.img
    }

    pub fn write_ppm<T>(&self, file: &mut T) -> Result<(), std::io::Error>
        where T: Write {
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;
        for pixel in &self.img {
            let (r, g, b) = pixel.to_u8();
            writeln!(file, "{} {} {}", r, g, b)?;
        }
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &ColorRGB> {
        self.img.iter()
    }
}

impl Index<(u32, u32)> for Image {
    type Output = ColorRGB;

    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        debug_assert!(y < self.height);
        debug_assert!(x < self.width);
        &self.img[(y * self.width + x) as usize]
    }
}

impl IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        debug_assert!(y < self.height);
        debug_assert!(x < self.width);
        &mut self.img[(y * self.width + x) as usize]
    }
}
