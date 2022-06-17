pub mod vec3d;
use vec3d::Vec3d;

fn main() {
    println!("Hello, world!");
    let x = Vec3d::<f64>::new();
    println!("{}", x.dot(&x).x);
}
