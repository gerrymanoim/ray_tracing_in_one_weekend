use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use std::rc::Rc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    //alternatively I could pass the lifetime around?
    pub fn add<T: Hittable + 'static>(&mut self, other: T) {
        self.objects.push(Rc::new(other));
    }
}

impl Hittable for &HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut last_hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            match obj.hit(r, t_min, closest_so_far) {
                Some(hr) => {
                    closest_so_far = hr.t;
                    last_hit_record = Some(hr);
                }
                None => (),
            }
        }

        last_hit_record
    }
}
