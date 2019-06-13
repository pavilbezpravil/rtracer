use crate::Vec3;

pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Aabb {
        Aabb { min, max }
    }

    pub fn new_from_center_size(center: Vec3, size: Vec3) -> Aabb {
        let hsize = size / 2.;
        let min = center - hsize;
        let max = center + hsize;
        Aabb { min, max }
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
}

fn aabb_noraml_at(aabb: &Aabb, point: &Vec3) -> Vec3 {
    const BIAS: f32 = 1. - 8. * std::f32::EPSILON;

    let local_point = *point - aabb.center();
    let hsize = aabb.size() / 2.;

    let d = Vec3::new(local_point.x() / hsize.x(),
                      local_point.y() / hsize.y(),
                      local_point.z() / hsize.z());

    if d.x().abs() > BIAS {
        d.x().signum() * Vec3::new_x()
    } else if d.y().abs() > BIAS {
        d.y().signum() * Vec3::new_y()
    } else {
        d.z().signum() * Vec3::new_z()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_at_simple() {
        let pos = Vec3::new(3., 2., -7.);

        let aabb = Aabb::new_from_center_size( pos, 2. * Vec3::unit());

        assert_eq!(aabb.normal_at(&(pos + Vec3::new_x())), Vec3::new_x());
        assert_eq!(aabb.normal_at(&(pos -Vec3::new_x())), -Vec3::new_x());

        assert_eq!(aabb.normal_at(&(pos + Vec3::new_y())), Vec3::new_y());
        assert_eq!(aabb.normal_at(&(pos - Vec3::new_y())), -Vec3::new_y());

        assert_eq!(aabb.normal_at(&(pos + Vec3::new_z())), Vec3::new_z());
        assert_eq!(aabb.normal_at(&(pos - Vec3::new_z())), -Vec3::new_z());
    }

    #[test]
    fn test_normal_at_hard() {
        let aabb = Aabb::new_from_center_size(Vec3::origin(), 2. * Vec3::unit());

        assert_eq!(aabb.normal_at(&Vec3::new(1., 0.4, -0.2)), Vec3::new_x());
        assert_eq!(aabb.normal_at(&Vec3::new(-1., 0.97, 0.89)), -Vec3::new_x());
        assert_eq!(aabb.normal_at(&Vec3::new(1., -0.97, 0.89)), Vec3::new_x());
    }
}
