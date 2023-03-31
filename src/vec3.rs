use num_traits::{Float, Num};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Vec3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

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
            z: self.z - rhs.z,
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

    pub fn unit_vector(&self) -> Vec3<T> {
        *self / self.length_sq()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let x = Vec3::<f64> {
            x: 2.,
            y: 3.,
            z: 4.,
        };
        let y = Vec3::<f64> {
            x: 4.,
            y: 5.,
            z: 6.,
        };

        let res = x + y;

        assert!(res.x - 6. < 1e-8);
        assert!(res.y - 8. < 1e-8);
        assert!(res.z - 10. < 1e-8);
    }

    #[test]
    fn sub() {
        let x = Vec3::<f64> {
            x: 2.,
            y: 3.,
            z: 4.,
        };
        let y = Vec3::<f64> {
            x: 4.,
            y: 5.,
            z: 6.,
        };

        let res = x - y;

        assert!(res.x + 2. < 1e-8);
        assert!(res.y + 2. < 1e-8);
        assert!(res.z + 2. < 1e-8);
    }

    #[test]
    fn mul() {
        let x = Vec3::<f64> {
            x: 2.,
            y: 3.,
            z: 4.,
        };
        let y = 4f64;

        let res = x * y;

        assert!(res.x - 8. < 1e-8);
        assert!(res.y - 12. < 1e-8);
        assert!(res.z - 16. < 1e-8);
    }

    #[test]
    fn div() {
        let x = Vec3::<f64> {
            x: 2.,
            y: 3.,
            z: 4.,
        };
        let y = 4f64;

        let res = x / y;

        assert!(res.x - 0.5 < 1e-8);
        assert!(res.y - 0.75 < 1e-8);
        assert!(res.z - 1.0 < 1e-8);
    }

    #[test]
    fn neg() {
        let x = Vec3::<f64> {
            x: 2.,
            y: 3.,
            z: 4.,
        };
        let res = -x;

        assert!(res.x + 2. < 1e-8);
        assert!(res.y + 3. < 1e-8);
        assert!(res.z + 4. < 1e-8);
    }
}
