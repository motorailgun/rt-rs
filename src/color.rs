use std::fmt::Display;
use num_traits::{Float, PrimInt};
use super::vec3::Vec3;

pub type Color<T> = Vec3<T>;

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
