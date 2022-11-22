use crate::vec3d::Vec3d;
use num_traits::Float;

#[derive(Debug, Clone, Copy)]
pub struct Ray<T: Float> {
    pub start: Vec3d<T>,
    pub direction: Vec3d<T>,
}
