use rtracer_core::prelude::*;
use crate::hit::{HitRecord, Hit};

pub struct Scene<H: Hit> {
    hittable: Vec<H>,
}

impl<H: Hit> Scene<H> {
    pub fn new() -> Scene<H> {
        Scene { hittable: Vec::new() }
    }

    pub fn add(&mut self, obj: H) {
        self.hittable.push(obj)
    }
}

impl<H: Hit> Default for Scene<H> {
    fn default() -> Scene<H> {
        Scene::new()
    }
}

impl<H: Hit> Hit for Scene<H> {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f32, f32)) -> Option<HitRecord> {
        let mut ret = None;

        for el in &self.hittable {
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
}
