use crate::aabb::Aabb;
use crate::Vec3;
use crate::intersect::Intersect;
use crate::ray::Ray;
use crate::bounded::Bounded;

#[derive(Copy, Clone, Debug)]
pub struct Cube {
    aabb: Aabb,
}

impl Cube {
    pub fn new(center: Vec3, size: Vec3) -> Cube {
        Cube { aabb: Aabb::from_center_size(center, size) }
    }

    pub fn new_from_aabb(aabb: Aabb) -> Cube {
        Cube { aabb }
    }

    pub fn center(&self) -> Vec3 {
        self.aabb.center()
    }

    pub fn size(&self) -> Vec3 {
        self.aabb.size()
    }

    pub fn normal_at(&self, point: &Vec3) -> Vec3 {
        self.aabb.normal_at(point)
    }
}

impl Intersect for Cube {
    fn intersect(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<f32> {
        self.aabb.intersect(ray, (t_min, t_max))
    }
}

impl Bounded for Cube {
    fn aabb(&self) -> Aabb {
        self.aabb
    }
}
