mod vec3;

use log::{log, Level};

fn main() {
    // Define image resolution
    let image_width = 256;
    let image_height = 256;



    // Render image in ppm format
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        log!(Level::Info, "\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.0f32;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    log!(Level::Info, "Done!");



}
