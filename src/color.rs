use super::vec3::Vec3;
use num_traits::{Float, PrimInt};
use std::fmt::Display;

pub type Color<T> = Vec3<T>;

impl<T: Float> Color<T> {
    pub fn clamp(&self) -> Color<T> {
        let one = T::one();
        Color {
            x: if self.x > one { one } else { self.x },
            y: if self.y > one { one } else { self.y },
            z: if self.z > one { one } else { self.z },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp() {
        let color = Color {
            x: 1.5f64,
            y: 1.8,
            z: 114.,
        };
        assert_eq!(
            color.clamp(),
            Color {
                x: 1f64,
                y: 1.,
                z: 1.
            }
        );

        let color = Color {
            x: 0.2f64,
            y: 0.5,
            z: 0.99,
        };
        assert_eq!(
            color.clamp(),
            Color {
                x: 0.2f64,
                y: 0.5,
                z: 0.99
            }
        );

        let color = Color {
            x: 1.01f64,
            y: 256.99,
            z: 0.99,
        };
        assert_eq!(
            color.clamp(),
            Color {
                x: 1f64,
                y: 1.,
                z: 0.99
            }
        );
    }

    #[test]
    fn to_int() {
        let color = Color {
            x: 0.9,
            y: 0.8,
            z: 0.7,
        };
        assert_eq!(
            color.to_int::<u8>(),
            Color {
                x: 229,
                y: 204,
                z: 178
            }
        );

        let color = Color {
            x: 1.0,
            y: 0.2,
            z: 0.0,
        };
        assert_eq!(
            color.to_int::<u8>(),
            Color {
                x: 255,
                y: 51,
                z: 0
            }
        );
    }
}
