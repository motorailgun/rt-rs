use crate::vec3d::Vec3d;
use num_traits::Float;

pub struct Ray<T: Float> {
    start: Vec3d<T>,
    direction: Vec3d<T>,
}