use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let mut out_f = File::create("image.ppm")?;
    let nx = 200;
    let ny = 100;
    out_f.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    for j in (0..ny).rev() {
        for i in 0..nx {

            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2f32;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;
            out_f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
        }
    }

    Ok(())
}
