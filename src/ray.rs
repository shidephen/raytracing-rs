use ndarray::prelude::*;
use std::fs::File;
use std::fmt;
use std::io::{Error, Write};

struct Ray {

    A: Array1<f32>,
    B: Array1<f32>
}

impl Ray {
    fn new_from_points(a :&Array1<f32>, b: &Array1<f32>) -> Self {
        Ray { A: a.clone(), B: b.clone() }
    }

    fn new() -> Self {
        Ray { A: Array1::<f32>::zeros((3,)), B: Array1::<f32>::zeros((3,)) }
    }

    fn origin(&self) -> &Array1<f32> {
        &self.A
    }

    fn direction(&self) -> &Array1<f32> {
        &self.B
    }

    fn point_at_parameter(&self, t: f32) -> Array1<f32> {
        let x = &self.B * t;
        &self.A + &x
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[({}, {}, {}), ({}, {}, {})]", self.A[0], self.A[1], self.A[2], self.B[0], self.B[1], self.B[2])
    }
}

fn main() -> Result<(), Error>{
    let mut out_f = File::create("ray.ppm")?;
    let nx = 200;
    let ny = 100;

    out_f.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes())?;

    let r = Ray::new();
    println!("Ray: {}", r);
    Ok(())
}
