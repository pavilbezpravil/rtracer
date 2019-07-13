use crate::Vec3;
use crate::ray::Ray;
use crate::intersect::Intersect;
use crate::intersection::ray_aabb_intersection;
use crate::bounded::Bounded;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min, max }
    }

    pub fn from_center_size(center: Vec3, size: Vec3) -> Aabb {
        let hsize = size / 2.;
        let min = center - hsize;
        let max = center + hsize;
        Aabb { min, max }
    }

    pub fn empty() -> Aabb {
        let min = std::f32::MIN;
        let max = std::f32::MAX;
        Aabb::new(Vec3::from_element(max), Vec3::from_element(min))
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) / 2.
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }

    pub fn normal_at(&self, point: &Vec3) -> Vec3 {
        aabb_noraml_at(self, point)
    }

    pub fn add_point(&mut self, p: &Vec3) {
        if p.x < self.min.x {
            self.min.x = p.x;
        }

        if p.y < self.min.y {
            self.min.y = p.y;
        }

        if p.z < self.min.z {
            self.min.z = p.z;
        }

        if p.x > self.max.x {
            self.max.x = p.x;
        }

        if p.y > self.max.y {
            self.max.y = p.y;
        }

        if p.z > self.max.z {
            self.max.z = p.z;
        }
    }

    pub fn add_aabb(&mut self, other: &Aabb) {
        *self = Aabb::union(self, other);
    }

    pub fn union(a: &Aabb, b: &Aabb) -> Aabb {
        Aabb {
            min: Vec3::new(
            a.min.x.min(b.min.x),
            a.min.y.min(b.min.y),
            a.min.z.min(b.min.z)),
            max: Vec3::new(
                a.max.x.max(b.max.x),
                a.max.y.max(b.max.y),
                a.max.z.max(b.max.z))
        }
    }
}

fn aabb_noraml_at(aabb: &Aabb, point: &Vec3) -> Vec3 {
    const BIAS: f32 = 1. - 8. * std::f32::EPSILON;

    let local_point = *point - aabb.center();
    let hsize = aabb.size() / 2.;

    let d = Vec3::new(local_point.x / hsize.x,
                      local_point.y / hsize.y,
                      local_point.z / hsize.z);

    if d.x.abs() > BIAS {
        d.x.signum() * Vec3::x()
    } else if d.y.abs() > BIAS {
        d.y.signum() * Vec3::y()
    } else {
        d.z.signum() * Vec3::z()
    }
}

impl Intersect for Aabb {
    fn intersect(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<f32> {
        if let Some((it_t_min, it_t_max)) = ray_aabb_intersection(ray, &self) {
            let t =
                if it_t_min > 0. {
                    it_t_min
                } else {
                    it_t_max
                };

            if t_min < t && t < t_max {
                return Some(t);
            }
        }

        None
    }
}

impl Bounded for Aabb {
    fn aabb(&self) -> Aabb {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_at_simple() {
        let pos = Vec3::new(3., 2., -7.);

        let aabb = Aabb::from_center_size(pos, 2. * Vec3::unit());

        assert_eq!(aabb.normal_at(&(pos + Vec3::new_x())), Vec3::new_x());
        assert_eq!(aabb.normal_at(&(pos -Vec3::new_x())), -Vec3::new_x());

        assert_eq!(aabb.normal_at(&(pos + Vec3::new_y())), Vec3::new_y());
        assert_eq!(aabb.normal_at(&(pos - Vec3::new_y())), -Vec3::new_y());

        assert_eq!(aabb.normal_at(&(pos + Vec3::new_z())), Vec3::new_z());
        assert_eq!(aabb.normal_at(&(pos - Vec3::new_z())), -Vec3::new_z());
    }

    #[test]
    fn test_normal_at_hard() {
        let aabb = Aabb::from_center_size(Vec3::origin(), 2. * Vec3::unit());

        assert_eq!(aabb.normal_at(&Vec3::new(1., 0.4, -0.2)), Vec3::new_x());
        assert_eq!(aabb.normal_at(&Vec3::new(-1., 0.97, 0.89)), -Vec3::new_x());
        assert_eq!(aabb.normal_at(&Vec3::new(1., -0.97, 0.89)), Vec3::new_x());
    }
}
