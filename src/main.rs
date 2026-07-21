mod vec3;
mod colors;

use std::io;
use colors::Color;

fn main() {
    // Image 

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    
    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in  (0..IMAGE_HEIGHT).rev(){
       for i in 0..IMAGE_WIDTH  {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT- 1) as f64;
            let b = 0.25;

            let pixel_color = Color::new(r, g, b);
            colors::write_color(&mut io::stdout(), pixel_color);
       } 
    }
}
