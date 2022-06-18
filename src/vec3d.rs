use num_traits::Float;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone)]
pub struct Vec3d<T: Float> {
        pub x: T,
        pub y: T,
        pub z: T,
}

impl<T: Float> Vec3d<T> {
    pub fn new() -> Self {
        return Vec3d { x: T::zero(), y: T::zero(), z: T::zero() };
    }

    pub fn dot(&self, other: &Vec3d<T>) -> Vec3d<T> {
        return Vec3d { x: self.x * other.x,
                        y: self.y * other.y,
                        z: self.z * other.z }
    }

    pub fn norm(&self) -> T {
        return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
    }
}

impl<T: Float> Add for Vec3d<T> {
    type Output = Vec3d<T>;
    fn add(self, other: Vec3d<T>) -> Self::Output {
        return Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}

impl<T: Float> Sub for Vec3d<T> {
    type Output = Vec3d<T>;
    fn sub(self, other: Vec3d<T>) -> Self::Output {
        return Vec3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        };
    }
}