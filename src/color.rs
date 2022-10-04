use num_traits::PrimInt;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Color<T: PrimInt + fmt::Display> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T: PrimInt + fmt::Display> fmt::Display for Color<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
