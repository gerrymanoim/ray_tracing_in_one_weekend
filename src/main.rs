mod ray;
mod vec3;
use ray::Ray;
use vec3::{dot, unit_vector, write_color, Color, Point3, Vec3};

/// solve the quadratic equation for a sphere and return if we find more than one root
/// 0 roots - no sphere
/// 1 root - edge
/// 2 roots - through the sphere
fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = dot(&r.direction, &r.direction);
    let b = 2.0 * dot(&oc, &r.direction);
    let c = dot(&oc, &oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

/// returns the color of the background (a simple gradient)
fn ray_color(r: &Ray) -> Color {
    if hit_sphere(
        Point3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        },
        0.5,
        r,
    ) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    // lintear blend/interpolation
    // blend between startValue and endValue
    // blendedVale = (1-t)*startValue + t*endValue
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
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

            let pixel_color = ray_color(&r);
            write_color(pixel_color);
        }
    }
    eprintln!("Done");
}
