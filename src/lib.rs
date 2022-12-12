use std::{fmt::Display, ops::{Add, Neg, Mul, Div, Sub}};

use num_traits::{Float, PrimInt, Num};

pub struct Vec3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

type Color<T> = Vec3<T>;
type Point3<T> = Vec3<T>;

impl<T: Float> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Float> Vec3<T> {
    pub fn length_sq(&self) -> T {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> T {
        self.length_sq().sqrt()
    }
}

impl<T: Float> Color<T> {
    pub fn clamp(&self) -> Color<T> {
        let one = T::one();
        Color { 
            x: if self.x > one { one } else {self.x},
            y: if self.y > one { one } else {self.y},
            z: if self.z > one { one } else {self.z},
        }
    }
    
    pub fn to_int<U: PrimInt + Display>(&self) -> Color<U> {
        let max = T::from(U::max_value()).unwrap();
        let clamped = self.clamp();

        Color {
            x: U::from(clamped.x * max).unwrap(),
            y: U::from(clamped.y * max).unwrap(),
            z: U::from(clamped.z * max).unwrap(),
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
