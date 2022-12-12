use std::{fmt::Display, ops::{Add, Neg, Mul, Div, Sub}};

use num_traits::{Float, PrimInt, Num};

pub struct Vec3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Color<T> = Vec3<T>;
pub type Point3<T> = Vec3<T>;

impl<T: Float> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Float> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Float> Sub for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl<T: Float> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Float> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Float> Vec3<T> {
    pub fn length_sq(&self) -> T {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> T {
        self.length_sq().sqrt()
    }
}

impl<T: Float> Color<T> {
    pub fn clamp(&self) -> Color<T> {
        let one = T::one();
        Color { 
            x: if self.x > one { one } else {self.x},
            y: if self.y > one { one } else {self.y},
            z: if self.z > one { one } else {self.z},
        }
    }
    
    pub fn to_int<U: PrimInt + Display>(&self) -> Color<U> {
        let max = T::from(U::max_value()).unwrap();
        let clamped = self.clamp();

        Color {
            x: U::from(clamped.x * max).unwrap(),
            y: U::from(clamped.y * max).unwrap(),
            z: U::from(clamped.z * max).unwrap(),
        }
    }
}
