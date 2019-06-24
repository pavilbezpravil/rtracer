use rtracer_core::prelude::*;
use crate::hit::{HitRecord, Hit};

pub struct HitableList<H: Hit> {
    hitable: Vec<H>,
}

impl<H: Hit> HitableList<H> {
    pub fn new() -> HitableList<H> {
        HitableList { hitable: Vec::new() }
    }

    pub fn add(&mut self, obj: H) {
        self.hitable.push(obj)
    }
}

impl<H: Hit> Default for HitableList<H> {
    fn default() -> HitableList<H> {
        HitableList::new()
    }
}

impl<H: Hit> Hit for HitableList<H> {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        let mut ret = None;

        for el in &self.hitable {
            let record = el.hit(ray, (t_min, t_max));

            match ret {
                None => ret = record,
                Some(ref inner) => if let Some(record) = record {
                    if record.t < inner.t {
                        ret = Some(record);
                    }
                }
            }
        }

        ret
    }

    fn aabb(&self) -> Aabb {
        self.hitable.iter().fold(Aabb::empty(), |mut aabb, hitable| { aabb.add_aabb(&hitable.aabb()); aabb })
    }
}
