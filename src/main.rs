use std::fmt::Display;

use num_traits::{Float, PrimInt, Num};

pub struct Color<T: Num> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Float> Color<T> {
    pub fn clamp(&self) -> Color<T> {
        let one = T::one();
        Color { 
            r: if self.r > one { one } else {self.r},
            g: if self.g > one { one } else {self.g},
            b: if self.b > one { one } else {self.b},
        }
    }
    
    pub fn to_int<U: PrimInt + Display>(&self) -> Color<U> {
        let max = T::from(U::max_value()).unwrap();
        let clamped = self.clamp();

        Color {
            r: U::from(clamped.r * max).unwrap(),
            g: U::from(clamped.g * max).unwrap(),
            b: U::from(clamped.b * max).unwrap(),
        }
    }
}

fn out_image<T: Float, U: PrimInt + Display>(canvas: &Vec<Vec<Color<T>>>, width: usize, height: usize){
    let level = U::max_value().to_u64().unwrap() + 1u64;
    println!("P3");
    println!("{} {}", width, height);
    println!("{}", level);

    for line in canvas.iter() {
        for pixel in line.iter() {
            let pix_i = pixel.to_int::<U>();
            println!("{} {} {} ", pix_i.r, pix_i.g, pix_i.b);
        }
        print!("");
    }
}

fn main() {
    let aspect_ratio: f64 = 16. / 9.;
    let image_width: usize = 400;
    let image_height = (400 as f64 / aspect_ratio) as usize;

    let canvas: Vec<Vec<Color<f64>>>;

    
}
