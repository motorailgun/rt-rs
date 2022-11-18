use std::fmt::Display;

use crate::{vec3d::Vec3d, primitives::Sphere, color::Color};
use num_traits::Float;

pub trait Material<T: Float + Display> {
    fn reflect<U: Material<T>>(&self, prim: &Sphere<T, U>, hit_point: &Vec3d<T>) -> Vec3d<T>;
    fn color(&self) -> Color<T>;
}

pub mod sphere_material {
    use std::fmt::Display;
    use crate::{color::Color, vec3d::Vec3d, utils::*, primitives::Sphere};
    use num_traits::Float;
    use rand::{self, prelude::Distribution, distributions::Standard};
    pub struct Lambertian<T: Float + Display> {
        pub color: Color<T>
    }

    impl<T> super::Material<T> for Lambertian<T> 
    where T: Float + Display, Standard: Distribution<T> {
        #[allow(unused_variables)]
        fn reflect<U: super::Material<T>>(&self, prim: &Sphere<T, U>, hit_point: &Vec3d<T>) -> Vec3d<T> {
            random_vec_in_unit_sphere() + *hit_point - prim.center
            // todo: fix near-zero vectors
        }

        fn color(&self) -> Color<T> {
            self.color
        }
    }
}
