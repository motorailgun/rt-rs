use num_traits::Float;
use crate::{ray::Ray, vec3d::Vec3d, material::sphere_material};

pub struct HitRecord<'a, T: Float> {
    pub ray: Ray<T>,
    pub prim: Box<&'a dyn Primitive<T>>,
    pub t: T,
}

pub trait Primitive<T: Float> {
    fn intersect(&self, incoming_ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>>;
}

pub struct Sphere<T: Float, G: sphere_material::Material<T>> {
    pub center: Vec3d<T>,
    pub radius: T,
    pub material: G,
}

impl<T: Float, G: sphere_material::Material<T>> Primitive<T> for Sphere<T, G> {
    fn intersect(&self, incoming_ray: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>> {
        let center_to_origin = incoming_ray.start - self.center;
        let a = incoming_ray.direction.norm();
        let half_b = center_to_origin.dot(&incoming_ray.direction);
        let c = center_to_origin.norm() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < T::from(0.0).unwrap() {
            None
        } else {
            let mut t = (-half_b - discriminant.sqrt()) / a;
            if t < t_min || t_max < t {
                t = (-half_b + discriminant.sqrt()) / a;
                if t < t_min || t_max < t {
                    return None;
                }
            }
            let start = incoming_ray.start + incoming_ray.direction * t;

            Some(HitRecord {
                prim: Box::new(self),
                ray: Ray::<T> {
                    start,
                    direction: start - self.center,
                },
                t,
            })
        }
    }
}
