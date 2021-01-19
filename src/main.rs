mod ray;
mod vec3;
use ray::Ray;
use vec3::{unit_vector, write_color, Color, Point3, Vec3};

/// returns the color of the background (a simple gradient)
fn ray_color(r: &Ray) -> Color {
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);

    (1.0 - t)
        * Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
        + t * Color {
            x: 0.5,
            y: 0.7,
            z: 1.0,
        }
}

fn main() {
    // Image
    // easier to not flip x, y if we use rectangle
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

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
    let origin = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    // left right
    let horizontal = Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    // up down
    let vertical = Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    // in out
    let depth = Vec3 {
        x: 0.0,
        y: 0.0,
        z: focal_length,
    };
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

            let r = Ray {
                origin: origin,
                direction: lower_left_corner + u*horizontal + v*vertical - origin,
            };
            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
    eprintln!("Done");
}
