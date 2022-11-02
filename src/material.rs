use std::fmt::Display;
use crate::color::Color;
use num_traits::Float;

pub trait Material {}

pub struct Lambertian<T: Float + Display> {
    pub color: Color<T>
}

impl<T: Float + Display> Material for Lambertian<T>{}
