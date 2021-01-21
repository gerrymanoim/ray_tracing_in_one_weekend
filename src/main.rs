mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
mod camera;
mod color;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{unit_vector, Point3, Vec3};
use camera::{Camera};
use color::{Color, write_color};

use rand::Rng;

/// returns the color where the ray intersects the plane
fn ray_color<T: Hittable>(r: &Ray, world: T) -> Color {
    match world.hit(r, 0.0, f32::INFINITY) {
        Some(hr) => return 0.5 * (hr.normal + Color::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = unit_vector(r.direction);
            let t = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    // Image
    // easier to not flip x, y if we use rectangle
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

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
                let u = (w as f32 + rng.gen::<f32>()) / (image_width - 1) as f32;
                // where are we on the veritcal plane
                let v = (h  as f32 + rng.gen::<f32>()) / (image_height - 1)  as f32;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("Done");
}
