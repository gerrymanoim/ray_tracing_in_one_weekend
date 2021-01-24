use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector};
use rand::prelude::*;

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

pub struct Dielectric {
    pub ir: f32, //index of refraction
}

impl Dielectric {
    fn reflectance(&self, cosine: f32, ref_idx: f32) -> f32 {
        // Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hr.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = unit_vector(r_in.direction);

        let cos_theta = dot(&-unit_direction, &hr.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = (refraction_ratio * sin_theta > 1.0)
            || self.reflectance(cos_theta, refraction_ratio) > random();

        let direction = if cannot_refract {
            reflect(&unit_direction, &hr.normal)
        } else {
            refract(unit_direction, hr.normal, refraction_ratio)
        };

        let scattered = Ray::new(hr.p, direction);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
