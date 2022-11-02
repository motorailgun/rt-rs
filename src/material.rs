use std::fmt::Display;
use crate::{ray::Ray, color::Color, vec3d::Vec3d};
use num_traits::Float;
use rand::{self, Rng, prelude::Distribution, distributions::Standard};

pub trait Material<T: Float> {
    fn reflect(&self, incoming_ray: &Ray<T>, hit_point: Vec3d<T>) -> Ray<T>;
}

pub struct Lambertian<T: Float + Display> {
    pub color: Color<T>
}

fn random_vec_in_unit_sphere<T>() -> Vec3d<T> 
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

impl<T> Material<T> for Lambertian<T> 
        where T: Float + Display, Standard: Distribution<T> {
    fn reflect(&self, incoming_ray: &Ray<T>, hit_point: Vec3d<T>) -> Ray<T> {
        Ray { 
            start: hit_point,
            direction: random_vec_in_unit_sphere::<T>(),
        }
    }
}
