use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};

pub struct HitRecord {
    p: Point3,
    pub normal: Vec3,
    pub t: f32,
    front_face: bool,
}

impl HitRecord {
    // TODO better name
    // TODO docstring this
    pub fn from_t_ray_outward_normal(t: f32, r: &Ray, outward_normal: &Vec3) -> Self {
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
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
