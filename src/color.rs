use num_traits::PrimInt;

#[derive(Clone, Copy)]
pub struct Color<T: PrimInt> {
    r: T,
    g: T,
    b: T,
}