use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use std::sync::Arc;

use rand::prelude::*;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
    pub fn add(&mut self, other: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(other);
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

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    )));

    let mut rng = rand::thread_rng();

    for a in (-11..11).into_iter() {
        for b in (-11..11).into_iter() {
            let choose_mat: f32 = rng.gen();
            let center = Point3::new(
                a as f32 + (0.9 * rng.gen::<f32>()),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            // make a bunch of small spheres
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Arc::new(Lambertian { albedo });
                    Sphere::new(center, 0.2, material)
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::bounded_random(&mut rng, 0.5, 1.0);
                    let fuzz = rng.gen();
                    let material = Arc::new(Metal { albedo, fuzz });
                    Sphere::new(center, 0.2, material)
                } else {
                    //glass
                    let material = Arc::new(Dielectric { ir: 1.5 });
                    Sphere::new(center, 0.2, material)
                };
                world.add(Arc::new(sphere))
            };
        }
    }
    // add 3 big spheres

    let material_1 = Arc::new(Dielectric { ir: 1.5 });
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world
}
