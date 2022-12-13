use std::fmt::Display;
#[allow(unused_imports)]
use num_traits::{Float, PrimInt, Num};
use rt_rs::prelude::*;

fn out_image<T: Float, U: PrimInt + Display>(canvas: &Vec<Vec<Color<T>>>, width: usize, height: usize){
    let level = U::max_value().to_u64().unwrap() + 1u64;
    println!("P3");
    println!("{} {}", width, height);
    println!("{}", level);

    for line in canvas.iter() {
        for pixel in line.iter() {
            let pix_i = pixel.to_int::<U>();
            print!("{} {} {} ", pix_i.x, pix_i.y, pix_i.z);
        }
        println!("");
    }
}

fn render<T: Float>(width: usize, height: usize) -> Vec<Vec<Color<T>>> {
    (0..height).collect::<Vec<usize>>().iter().enumerate().map(|(y, _)| {
        (0..width).collect::<Vec<usize>>().iter().enumerate().map(|(x, _)| -> Color<T> {
            let r = T::from(y).unwrap() / T::from(height).unwrap();
            let g = T::from(x).unwrap() / T::from(width).unwrap();
            let b = T::from(0.25).unwrap();

            Color{x: r, y: g, z: b}
        }).collect()
    }).collect()
}

fn main() {
    #[allow(non_camel_case_types)]
    type float_t = f64;

    let aspect_ratio: float_t = 16. / 9.;
    let image_width: usize = 400;
    let image_height = (400 as float_t / aspect_ratio) as usize;

    let canvas: Vec<Vec<Color<float_t>>> = render(image_width, image_height);
    out_image::<float_t, u8>(&canvas, image_width, image_height);
}
