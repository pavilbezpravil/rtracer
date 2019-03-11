use std::ops::{Index, IndexMut};

use crate::Vec3;

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

    pub fn set_r(&mut self, val: f32) {
        self.set_x(val)
    }

    pub fn set_g(&mut self, val: f32) {
        self.set_y(val)
    }

    pub fn set_b(&mut self, val: f32) {
        self.set_z(val)
    }

    pub fn to_u8(&self) -> (u8, u8, u8) {
        assert!(self.r() >= 0f32 && self.r() <= 1f32);
        assert!(self.g() >= 0f32 && self.g() <= 1f32);
        assert!(self.b() >= 0f32 && self.b() <= 1f32);

        ((self.r() * 255.99f32) as u8,
         (self.g() * 255.99f32) as u8,
         (self.b() * 255.99f32) as u8)
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
        Image::with_background(width, height, &ColorRGB::new(0f32, 0f32, 0f32))
    }

    pub fn with_background(width: u32, height: u32, c: &ColorRGB) -> Image {
        Image { width, height, img: vec![*c; (width * height) as usize] }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn print_as_ppm(&self) {
        println!("P3");
        println!("{} {}", self.width, self.height);
        println!("255");
        for pixel in &self.img {
            let (r, g, b) = pixel.to_u8();
            println!("{} {} {}", r, g, b);
        }
    }
}

impl Index<(u32, u32)> for Image {
    type Output = ColorRGB;

    fn index(&self, index: (u32, u32)) -> &Self::Output {
        let (x, y) = index;
        &self.img[(y * self.width + x) as usize]
    }
}

impl IndexMut<(u32, u32)> for Image {
    fn index_mut(&mut self, index: (u32, u32)) -> &mut Self::Output {
        let (x, y) = index;
        &mut self.img[(y * self.width + x) as usize]
    }
}
