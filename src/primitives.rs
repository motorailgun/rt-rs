use num_traits::Float;
use crate::{ray::Ray, vec3d::Vec3d, material::Material};

pub trait Primitive<T: Float> {
    fn intersect(&self, incoming_ray: &Ray<T>) -> Option<Ray<T>>;
}

pub struct Sphere<T: Float, G: Material<T>> {
    pub center: Vec3d<T>,
    pub radius: T,
    pub material: G,
}

impl<T: Float, G: Material<T>> Primitive<T> for Sphere<T, G> {
    fn intersect(&self, incoming_ray: &Ray<T>) -> Option<Ray<T>> {
        let center_to_origin = incoming_ray.start - self.center;
        let a = incoming_ray.direction.norm();
        let half_b = center_to_origin.dot(&incoming_ray.direction);
        let c = center_to_origin.norm() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < T::from(0.0)? {
            None
        } else {
            Some(Ray::<T> {
                start: incoming_ray.start + incoming_ray.direction * discriminant,
                direction: self.material.reflect(&(incoming_ray.start + incoming_ray.direction * discriminant))
            })
        }
    }
}
