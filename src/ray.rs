use crate::vec3::Vec3;
use num_traits::Float;

#[derive(Clone, Copy)]
pub struct Ray<T: Float> {
    pub start: Vec3<T>,
    pub direction: Vec3<T>,
}
