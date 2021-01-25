pub use crate::vec3::Vec3;

pub use Vec3 as Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f32;

    let r = clamp((pixel_color.x * scale).sqrt(), 0.0, 0.999);
    let g = clamp((pixel_color.y * scale).sqrt(), 0.0, 0.999);
    let b = clamp((pixel_color.z * scale).sqrt(), 0.0, 0.999);

    let ir = (256.0 * r) as u32;
    let ig = (256.0 * g) as u32;
    let ib = (256.0 * b) as u32;

    println!("{} {} {}", ir, ig, ib);
}

fn clamp(x: f32, x_min: f32, x_max: f32) -> f32 {
    if x < x_min {
        return x_min;
    } else if x > x_max {
        return x_max;
    } else {
        return x;
    }
}
