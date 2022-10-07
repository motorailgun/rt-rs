use num_traits::Float;
use crate::{ray::Ray, vec3d::Vec3d};

pub trait Primitive {
    fn intersect<T: Float>(&self, incoming_ray: &Ray<T>) -> Option<Ray<T>>;
}

pub struct Sphere<T: Float, G: Primitive> {
    pub center: Vec3d<T>,
    pub radius: T,
    pub material: G,
}
