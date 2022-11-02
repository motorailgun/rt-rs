use std::fmt::Display;
use crate::{ray::Ray, color::Color, vec3d::Vec3d, utils::*};
use num_traits::Float;
use rand::{self, prelude::Distribution, distributions::Standard};

pub trait Material<T: Float> {
    fn reflect(&self, hit_point: Vec3d<T>) -> Ray<T>;
}

pub struct Lambertian<T: Float + Display> {
    pub color: Color<T>
}

impl<T> Material<T> for Lambertian<T> 
        where T: Float + Display, Standard: Distribution<T> {
    fn reflect(&self, hit_point: Vec3d<T>) -> Ray<T> {
        Ray { 
            start: hit_point,
            direction: random_vec_in_unit_sphere::<T>(),
        }
    }
}
