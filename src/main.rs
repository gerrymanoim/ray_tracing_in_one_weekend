mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;
use hittable::Hittable;
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use vec3::{unit_vector, write_color, Color, Point3, Vec3};

/// returns the color where the ray intersects the plane
fn ray_color<T: Hittable>(r: &Ray, world: T) -> Color {
    match world.hit(r, 0.0, f32::INFINITY) {
        Some(hr) => return 0.5 * (hr.normal * Color::new(1.0, 1.0, 1.0)),
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

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    // (-2, 1, -1)        (2, 1, -1)
    //     +----------------+
    //     |    u           |
    //     +---------->     |
    //     |           ^    |
    //     |           |v   |
    //     +----------------+
    // (-2, -1, -1)       (2, -1, -1)

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    // how far the plane is from the projection point
    let focal_length = 1.0;

    // Centered at 0,0,0 at the center of the image
    let origin = Point3::new(0.0, 0.0, 0.0);
    // left right
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // up down
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    // in out
    let depth = Vec3::new(0.0, 0.0, focal_length);

    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - depth;

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for h in (0..image_height).rev() {
        eprintln!("\r Scanlines remaining: {}", h);
        for w in 0..image_width {
            // where are we on the horizonal plane
            let u = (w as f32) / (image_width - 1) as f32;
            // where are we on the veritcal plane
            let v = (h as f32) / (image_height - 1) as f32;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&r, &world);
            write_color(pixel_color);
        }
    }
    eprintln!("Done");
}
