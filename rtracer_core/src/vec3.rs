use std::ops::Neg;
use std::ops::{Add, AddAssign};
use std::ops::{Sub, SubAssign};
use std::ops::{Mul, MulAssign};
use std::ops::{Div, DivAssign};

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vec3([f32; 3]);

impl Vec3 {
    pub const I: Vec3 = Vec3([1., 0., 0.]);
    pub const J: Vec3 = Vec3([0., 1., 0.]);
    pub const K: Vec3 = Vec3([0., 0., 1.]);

    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3([x, y, z])
    }

    pub fn new_x() -> Vec3 {
        Vec3::I
    }

    pub fn new_y() -> Vec3 {
        Vec3::J
    }

    pub fn new_z() -> Vec3 {
        Vec3::K
    }

    pub fn origin() -> Vec3 {
        Vec3::new(0., 0., 0.)
    }

    pub fn unit() -> Vec3 {
        Vec3::new(1., 1., 1.)
    }

    pub fn x(&self) -> f32 {
        self.0[0]
    }

    pub fn y(&self) -> f32 {
        self.0[1]
    }

    pub fn z(&self) -> f32 {
        self.0[2]
    }

    pub fn as_array(&self) -> [f32; 3] {
        self.0
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn dot(lhs: &Vec3, rhs: &Vec3) -> f32 {
        lhs.x() * rhs.x() + lhs.y() * rhs.y() + lhs.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x(),
        )
    }

    pub fn try_make_unit(&self) -> Option<Vec3> {
        if self.length() < 2. * std::f32::EPSILON {
            None
        } else {
            let mut unit = *self;
            unit /= self.length();
            Some(unit)
        }
    }

    pub fn make_unit(&self) -> Vec3 {
        debug_assert!(self.length() > 2. * std::f32::EPSILON);
        let mut unit = *self;
        unit /= self.length();
        unit
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let mut ret = self;
        ret += rhs;
        ret
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0[0] += rhs.x();
        self.0[1] += rhs.y();
        self.0[2] += rhs.z();
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        let mut ret = self;
        ret -= rhs;
        ret
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0[0] -= rhs.x();
        self.0[1] -= rhs.y();
        self.0[2] -= rhs.z();
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut ret = self;
        ret *= rhs;
        ret
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.0[0] *= rhs.x();
        self.0[1] *= rhs.y();
        self.0[2] *= rhs.z();
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scale: f32) -> Self::Output {
        let mut ret = self;
        ret *= scale;
        ret
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        let mut v = v;
        v *= self;
        v
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scale: f32) {
        self.0[0] *= scale;
        self.0[1] *= scale;
        self.0[2] *= scale;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scale: f32) -> Self::Output {
        let mut ret = self;
        ret /= scale;
        ret
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scale: f32) {
        self.0[0] /= scale;
        self.0[1] /= scale;
        self.0[2] /= scale;
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(ns: [f32; 3]) -> Vec3 {
        Vec3 { 0: ns }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let v1 = Vec3::new(1f32, 2f32, 3f32);

        assert_eq!(v1.x(), 1f32);
        assert_eq!(v1.y(), 2f32);
        assert_eq!(v1.z(), 3f32);
    }

    #[test]
    fn test_neg() {
        let v1 = Vec3::new(1f32, 2f32, 3f32);
        let v_neg = -v1;

        assert_eq!(v_neg, Vec3::new(-1f32, -2f32, -3f32));
    }

    #[test]
    fn test_add() {
        let mut v1 = Vec3::new(1f32, 2f32, 3f32);
        let v2 = Vec3::new(10f32, 20f32, 30f32);
        let res = Vec3::new(11f32, 22f32, 33f32);

        assert_eq!(v1 + v2, res);
        v1 += v2;
        assert_eq!(v1, res);
    }

    #[test]
    fn test_sub() {
        let mut v1 = Vec3::new(1f32, 2f32, 3f32);
        let v2 = Vec3::new(10f32, 20f32, 30f32);
        let res = Vec3::new(-9f32, -18f32, -27f32);

        assert_eq!(v1 - v2, res);
        v1 -= v2;
        assert_eq!(v1, res);
    }

    #[test]
    fn test_mul() {
        let mut v1 = Vec3::new(1f32, 2f32, 3f32);
        let v2 = Vec3::new(10f32, 20f32, 30f32);
        let res = Vec3::new(10f32, 40f32, 90f32);

        assert_eq!(v1 * v2, res);
        assert_eq!(v2 * v1, res);
        v1 *= v2;
        assert_eq!(v1, res);
    }

    #[test]
    fn test_mull() {
        let mut v1 = Vec3::new(1f32, 2f32, 3f32);
        let rhs = 5f32;
        let res = Vec3::new(5f32, 10f32, 15f32);

        assert_eq!(v1 * rhs, res);
        assert_eq!(rhs * v1, res);
        v1 *= rhs;
        assert_eq!(v1, res);
    }

    #[test]
    fn test_div() {
        let mut v1 = Vec3::new(5f32, 15f32, 45f32);
        let rhs = 5f32;
        let res = Vec3::new(1f32, 3f32, 9f32);

        assert_eq!(v1 / rhs, res);
        v1 /= rhs;
        assert_eq!(v1, res);
    }

    #[test]
    fn test_cross_product() {
        let i = Vec3::new(1.0, 0.0, 0.0);
        let j = Vec3::new(0.0, 1.0, 0.0);
        let k = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(Vec3::I, i);
        assert_eq!(Vec3::J, j);
        assert_eq!(Vec3::K, k);
        assert_eq!(i.cross(&j), k);
        assert_eq!(j.cross(&k), i);
        assert_eq!(k.cross(&i), j);
        assert_eq!(j.cross(&i), -k);
        assert_eq!(k.cross(&j), -i);
        assert_eq!(i.cross(&k), -j);
        assert_eq!((i * 2.0).cross(&(j * 2.0)), k * 4.0);
        assert_eq!((j * 2.0).cross(&(k * 2.0)), i * 4.0);
        assert_eq!((k * 2.0).cross(&(i * 2.0)), j * 4.0);
    }

    #[test]
    fn test_dot_product() {
        let v1 = Vec3::new(1f32, 2f32, 3f32);
        let v2 = Vec3::new(10f32, 20f32, 30f32);
        let res = (10 + 40 + 90) as f32;

        assert_eq!(Vec3::dot(&v1, &v2), res);
        assert_eq!(Vec3::dot(&v2, &v1), res);
    }

    #[test]
    fn test_length() {
        assert_eq!(Vec3::new(1f32, 0f32, 0f32).squared_length(), 1f32);
        assert_eq!(Vec3::new(1f32, 0f32, 0f32).length(), 1f32);
        assert_eq!(Vec3::new(1f32, 2f32, 3f32).squared_length(), 14f32);
        assert_eq!(Vec3::new(1f32, 2f32, 3f32).length(), 14f32.sqrt());
        assert_eq!(Vec3::new(4f32, 2f32, 3f32).squared_length(), 29f32);
        assert_eq!(Vec3::new(4f32, 2f32, 3f32).length(), 29f32.sqrt());
    }

    // TODO: погуглив я не нашел нормального способа сравнивать float'ы
    fn float_cmp(a: f32, b: f32) -> bool {
        (a - b).abs() < 2f32 * ::std::f32::EPSILON
    }

    #[test]
    fn test_make_unit() {
        assert!(float_cmp(Vec3::new(1f32, 0f32, 0f32).make_unit().length(), 1f32));
        assert!(float_cmp(Vec3::new(1f32, 2f32, 3f32).make_unit().length(), 1f32));
        assert!(float_cmp(Vec3::new(4f32, 2f32, 3f32).make_unit().length(), 1f32));
    }
}
