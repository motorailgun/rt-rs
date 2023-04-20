pub mod color;
pub mod point3;
pub mod ray;
pub mod vec3;

pub mod prelude {
    pub type Vec3<T> = super::vec3::Vec3<T>;
    pub type Color<T> = super::color::Color<T>;
    pub type Point3<T> = Vec3<T>;
    pub type Ray<T> = super::ray::Ray<T>;
}
