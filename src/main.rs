mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod vec3;
use camera::Camera;
use color::{write_color, Color};
use hittable::Hittable;
use hittable_list::HittableList;
use material::{Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use vec3::{random_unit_vector, unit_vector, Point3};

use rand::prelude::*;
use rayon::prelude::*;
// tried a parallel task for samples for pixel but had issues with rc

/// returns the color where the ray intersects the plane
fn ray_color<T: Hittable>(r: &Ray, world: T, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    match world.hit(r, 0.001, f32::INFINITY) {
        Some(hr) => {
            match hr.material.scatter(r, &hr) {
                Some((attenuation, scattered)) => {
                    return attenuation * ray_color(&scattered, world, depth - 1)
                }
                None => {
                    // absorbed
                    return Color::new(0.0, 0.0, 0.0);
                }
            }
        }
        None => {
            let unit_direction = unit_vector(r.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    // Image
    // easier to not flip x, y if we use rectangle
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();

    // Materials
    let material_ground = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Rc::new(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });
    let material_right = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    // Objects in the world
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    // how far the plane is from the projection point
    let focal_length = 1.0;

    // Centered at 0,0,0 at the center of the image
    let cam = Camera::new();
    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for h in (0..image_height).rev() {
        eprintln!("\r Scanlines remaining: {}", h);
        for w in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for s in 0..samples_per_pixel {
                // where are we on the horizonal plane
                let u = (w as f32 + random::<f32>()) / (image_width - 1) as f32;
                // where are we on the veritcal plane
                let v = (h as f32 + random::<f32>()) / (image_height - 1) as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done");
}
