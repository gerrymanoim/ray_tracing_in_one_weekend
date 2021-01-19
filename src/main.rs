mod vec3;
use vec3::{write_color, Color};

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);
    for h in (0..image_height).rev() {
        eprintln!("\r Scanlines remaining: {}", h);
        for w in 0..image_width {
            let pixel_color = Color {
                x: (w as f32) / (image_width - 1) as f32,
                y: (h as f32) / (image_height - 1) as f32,
                z: 0.25,
            };
            write_color(pixel_color);
        }
    }
    eprintln!("Done");
}
