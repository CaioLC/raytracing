use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;

const IMG_WIDTH: u32 = 256;
const IMG_HEIGHT: u32 = 256;

fn main() -> io::Result<()> {
    let f = File::create("image2.ppm")?;
    let mut writer = BufWriter::new(f);
    write!(&mut writer, "P3\n{IMG_WIDTH} {IMG_HEIGHT}\n255\n")?; 
    // Render
    for j in 0..IMG_WIDTH {
        println!("Scanlines remaining: {:?}", (IMG_HEIGHT - j));
        for i in 0..IMG_HEIGHT {
            let r = i as f32 / IMG_WIDTH as f32;
            let g = j as f32 / IMG_HEIGHT as f32;
            let b: f32 = 0.0;
            
            let r_int = (r*255.999) as u32;
            let g_int = (g*255.999) as u32;
            let b_int = (b*255.999) as u32;
            write!(&mut writer, "{r_int} {g_int} {b_int} \n")?;
            // to_buffer(&mut buffer, r_int, g_int, b_int)
        }
    };
    println!("Complete.");
    Ok(())
}
