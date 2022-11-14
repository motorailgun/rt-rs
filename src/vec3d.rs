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

    pub fn dot(&self, other: &Vec3d<T>) -> T {
        return self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm_pow2(&self) -> T {
        self.dot(self)
    }

    pub fn norm(&self) -> T {
        self.norm_pow2().sqrt()
    }

    pub fn unitize(&self) -> Vec3d<T> {
        let norm = self.norm();
        return *self / norm;
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

impl<T: Float> Mul<T> for Vec3d<T> {
    type Output = Vec3d<T>;
    fn mul(self, other: T) -> Self::Output {
        return Vec3d {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        };
    }
}

impl<T: Float> Div<T> for Vec3d<T> {
    type Output = Vec3d<T>;
    fn div(self, other: T) -> Self::Output {
        return Vec3d {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        };
    }
}
