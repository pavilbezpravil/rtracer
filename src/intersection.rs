use crate::{Ray, Vec3, Sphere, Plane, Aabb};

pub fn ray_sphere_intersection(ray: &Ray, sphere: &Sphere) -> Option<f32> {
    let oc = ray.origin - sphere.center;

    let a = ray.direction.squared_length();
    let b = Vec3::dot(&ray.direction, &oc);
    let c = Vec3::dot(&oc, &oc) - sphere.radius * sphere.radius;

    let discriminant = b * b - a * c;

    if discriminant < 0f32 {
        return None;
    }

    let discriminant_root = discriminant.sqrt();

    let t1 = (-b - discriminant_root) / a;
    let t2 = (-b + discriminant_root) / a;

    if t1 > 0. {
        Some(t1)
    } else if t2 > 0. {
        Some(t2)
    } else {
        None
    }
}

pub fn ray_plane_intersection(ray: &Ray, plane: &Plane) -> f32 {
    let ray_perpendicular_component = Vec3::dot(&plane.normal, &ray.direction);

    if ray_perpendicular_component.abs() < 2. * std::f32::EPSILON {
        if ray_perpendicular_component > 0. {
            std::f32::MAX
        } else {
            std::f32::MIN
        }
    } else {
        let perpendicular_traverse_distance = Vec3::dot(&plane.normal, &(plane.origin - ray.origin));
        perpendicular_traverse_distance / ray_perpendicular_component
    }
}

pub fn ray_box_intersection(ray: &Ray, aabb: &Aabb) -> Option<(f32, f32)> {
    let box_min = aabb.min;
    let box_max = aabb.max;

    let ray_pos = ray.origin;

    let inv_dir = Vec3::new(1. / ray.direction.x(), 1. / ray.direction.y(), 1. / ray.direction.z());

    let mut tmin;
    let mut tmax;

    let lo = inv_dir.x() * (box_min.x() - ray_pos.x());
    let hi = inv_dir.x() * (box_max.x() - ray_pos.x());

    tmin = lo.min(hi);
    tmax = lo.max(hi);

    let lo1 = inv_dir.y() * (box_min.y() - ray_pos.y());
    let hi1 = inv_dir.y() * (box_max.y() - ray_pos.y());

    tmin = tmin.max(lo1.min(hi1));
    tmax = tmax.min(lo1.max(hi1));

    let lo2 = inv_dir.z() * (box_min.z() - ray_pos.z());
    let hi2 = inv_dir.z() * (box_max.z() - ray_pos.z());

    tmin = tmin.max(lo2.min(hi2));
    tmax = tmax.min(lo2.max(hi2));

    if (tmin <= tmax) && (tmax > 0.) {
        Some((tmin, tmax))
    } else {
        None
    }
}