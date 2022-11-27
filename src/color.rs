use num_traits::{Num, Unsigned, PrimInt, NumCast};
use std::ops::{Add, Mul, Rem, Div};
use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug)]
pub struct Color<T: Num + fmt::Display> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: Num + fmt::Display> fmt::Display for Color<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}

impl<T: Num + Copy + fmt::Display> Add for Color<T> {
    type Output = Color<T>;
    fn add(self, other: Self) -> Self::Output {
        Color::<T> {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl<T: Num + Copy + fmt::Display> Mul<T> for Color<T> {
    type Output = Color<T>;
    fn mul(self, other: T) -> Self::Output {
        Color::<T> {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl<T: Num + Copy + fmt::Display> Mul<Color<T>> for Color<T> {
    type Output = Color<T>;
    fn mul(self, other: Color<T>) -> Self::Output {
        Color::<T> {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl<T: Num + Copy + fmt::Display> Div<T> for Color<T> {
    type Output = Color<T>;
    fn div(self, other: T) -> Self::Output {
        return Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other
        };
    }
}


impl<T: Num + Copy + fmt::Display> Rem<T> for Color<T> {
    type Output = Color<T>;
    fn rem(self, other: T) -> Self::Output {
        Color::<T> {
            r: self.r % other,
            g: self.g % other,
            b: self.b % other,
        }
    }
}

impl<T: Num + fmt::Display + NumCast + Copy + PartialOrd> Color<T> {
    fn clamp(&self) -> Color<T> {
        let one = T::one();
        Color {
            r: if self.r > one {one} else {self.r},
            g: if self.g > one {one} else {self.g},
            b: if self.b > one {one} else {self.b},
        }
    }

    pub fn to_int<G: PrimInt + Unsigned + Display>(&self) -> Option<Color<G>>{
        let clamped = self.clamp();

        if let Some(max_value) = T::from(G::max_value()) {
            let Color{r, g, b} = clamped * max_value;
            G::from(r)
                .zip(G::from(g))
                .zip(G::from(b))
                .map(|((r, g), b)| Color{r, g, b})
        } else {
            None
        }
    }
}
