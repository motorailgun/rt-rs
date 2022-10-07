use std::fmt::Display;
use crate::{ray::Ray, color::Color, vec3d::Vec3d};
use num_traits::Float;

pub trait Material<T: Float> {
    fn reflect(&self, incoming_ray: &Ray<T>, hit_point: Vec3d<T>) -> Ray<T>;
}

pub struct Lambertian<T: Float + Display> {
    pub color: Color<T>
}

fn random_vec_in_unit_sphere<T: Float + Display>(){
    
}

impl<T: Float + Display> Material<T> for Lambertian<T> {
    fn reflect(&self, incoming_ray: &Ray<T>, hit_point: Vec3d<T>) -> Ray<T> {
        
    }
}
