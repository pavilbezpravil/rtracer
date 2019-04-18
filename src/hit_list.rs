use crate::Ray;
use crate::{Hit, HitRecord};

pub struct HitList {
    hittable: Vec<Box<dyn Hit>>,
}

impl HitList {
    pub fn new() -> HitList {
        HitList { hittable: Vec::new() }
    }

    pub fn add(&mut self, obj: Box<dyn Hit>) {
        self.hittable.push(obj)
    }
}

impl Default for HitList {
    fn default() -> HitList {
        HitList::new()
    }
}

impl Hit for HitList {
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
