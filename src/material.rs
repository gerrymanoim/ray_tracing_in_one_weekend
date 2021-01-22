use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, unit_vector};

pub trait Material {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hr.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            // if they randomly cancel out
            scatter_direction = hr.normal
        }
        return Some((self.albedo, Ray::new(hr.p, scatter_direction)));
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let reflected =
            reflect(&unit_vector(r_in.direction), &hr.normal) + self.fuzz * random_in_unit_sphere();
        let scattered = Ray::new(hr.p, reflected);
        if dot(&scattered.direction, &hr.normal) < 0.0 {
            None
        } else {
            Some((self.albedo, scattered))
        }
    }
}
