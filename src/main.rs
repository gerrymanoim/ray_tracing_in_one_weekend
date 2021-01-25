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
use hittable_list::random_scene;
use ray::Ray;

use vec3::{unit_vector, Point3, Vec3};

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
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // TODO - I think my use of Arc is not correct

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperature,
        dist_to_focus,
    );
    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for h in (0..image_height).rev() {
        eprintln!("\r Scanlines remaining: {}", h);
        for w in 0..image_width {
            let pixel_color = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let u = (w as f32 + random::<f32>()) / (image_width - 1) as f32;
                    // where are we on the veritcal plane
                    let v = (h as f32 + random::<f32>()) / (image_height - 1) as f32;
                    let r = cam.get_ray(u, v);
                    ray_color(&r, &world, max_depth)
                })
                .reduce(|| Color::new(0.0, 0.0, 0.0), |a, b| a + b);

            //let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            // for s in 0..samples_per_pixel {
            //     // where are we on the horizonal plane
            //     let u = (w as f32 + random::<f32>()) / (image_width - 1) as f32;
            //     // where are we on the veritcal plane
            //     let v = (h as f32 + random::<f32>()) / (image_height - 1) as f32;
            //     let r = cam.get_ray(u, v);
            //     pixel_color += ray_color(&r, &world, max_depth);
            // }

            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done");
}
