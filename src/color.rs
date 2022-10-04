use num_traits::PrimInt;

#[derive(Clone, Copy)]
pub struct Color<T: PrimInt> {
    pub r: T,
    pub g: T,
    pub b: T,
}
