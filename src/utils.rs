use std::{fmt::Display, any::type_name};
use crate::vec3d::Vec3d;
use num_traits::Float;
use rand::{self, Rng, prelude::Distribution, distributions::Standard};

pub fn random_vec_in_unit_sphere<T>() -> Vec3d<T> 
        where T: Float + Display, Standard: Distribution<T> {
    let mut rng = rand::thread_rng();
    let two_pi = match T::from(3.14159265358979f64 * 2.) {
        Some(num)=> num,
        None => panic!("there's no known conversion from f64 to T"),
    };
    let pi = match T::from(3.14159265358979f64) {
        Some(num)=> num,
        None => panic!("there's no known conversion from f64 to T"),
    };

    let theta: T = rng.gen(); // 水平方面: [0, 2π]
    let phi: T = rng.gen(); // 鉛直方面: [0, π]

    let x: T = (pi * phi).cos() * (two_pi * theta).cos();
    let y: T = (pi * phi).sin();
    let z: T = (pi * phi).cos() * (two_pi * theta).sin();

    Vec3d { x, y, z }
}

pub trait FloatU: Float {
    fn f64(self) -> f64 {
        self.to_f64().expect(&format!("No known comversion from {} to f64", type_name::<Self>()))
    }
    fn f32(self) -> f32 {
        self.to_f32().expect(&format!("No known comversion from {} to f32", type_name::<Self>()))
    }
    fn tt<T: Float>(self) -> T {
        T::from(self).expect(&format!("No known comversion from {} to {}", type_name::<T>(), type_name::<Self>()))
    }
}

impl FloatU for f64 {}
impl FloatU for f32 {}