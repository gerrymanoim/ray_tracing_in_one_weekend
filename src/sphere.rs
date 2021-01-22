use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot, Point3};
use crate::material::Material;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Rc<dyn Material>,

}

impl Sphere {
    // TODO - can I do something about these lifetimes?
    pub fn new(center: Point3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {center, radius, material}
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || root > t_max {
            // other root
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let r = HitRecord::from_t_ray_outward_normal(
            root,
            &r,
            &((r.at(root) - self.center) / self.radius),
            self.material.clone() //cloning creates a new reference to the same heap
        );

        return Some(r);
    }
}
