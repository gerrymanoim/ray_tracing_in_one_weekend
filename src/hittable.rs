use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use crate::material::Material;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    front_face: bool,
    pub material: Rc<dyn Material>
}

impl HitRecord {
    // TODO better name
    // TODO docstring this
    pub fn from_t_ray_outward_normal(t: f32, r: &Ray, outward_normal: &Vec3, material: Rc<dyn Material>) -> Self {
        let p = r.at(t);
        let front_face = dot(&r.direction, outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
