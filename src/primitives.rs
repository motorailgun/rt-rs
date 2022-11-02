use num_traits::Float;
use crate::{ray::Ray, vec3d::Vec3d, material::Material};

pub trait Primitive {
    fn intersect<T: Float>(&self, incoming_ray: &Ray<T>) -> Option<Ray<T>>;
}

pub struct Sphere<T: Float, G: Material> {
    pub center: Vec3d<T>,
    pub radius: T,
    pub material: G,
}

impl<T: Float, G: Material> Sphere<T, G> {
    fn determine(&self) -> Option<Vec3d<T>>{
        
    }
}

impl<T: Float, G: Material> Primitive for Sphere<T, G> {
    fn intersect(&self, incoming_ray: &Ray<T>) -> Option<Ray<T>> {

    }
}
