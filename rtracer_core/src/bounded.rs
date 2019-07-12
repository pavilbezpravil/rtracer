use crate::aabb::Aabb;

pub trait Bounded  {
    fn aabb(&self) -> Aabb;
}
