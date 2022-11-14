pub mod sphere_material {
    use std::fmt::Display;
    use crate::{color::Color, vec3d::Vec3d, utils::*, primitives::Sphere};
    use num_traits::Float;
    use rand::{self, prelude::Distribution, distributions::Standard};

    pub trait Material<T: Float> {
        fn reflect<U: Material<T>>(&self, prim: &Sphere<T, U>, hit_point: &Vec3d<T>) -> Vec3d<T>;
    }
    pub struct Lambertian<T: Float + Display> {
        pub color: Color<T>
    }

    impl<T> Material<T> for Lambertian<T> 
    where T: Float + Display, Standard: Distribution<T> {
        #[allow(unused_variables)]
        fn reflect<U: Material<T>>(&self, prim: &Sphere<T, U>, hit_point: &Vec3d<T>) -> Vec3d<T> {
            random_vec_in_unit_sphere()
            // todo: must fix this to emit outgoing vector using normal
        }
    }
}
