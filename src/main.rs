pub mod vec3d;
pub mod color;
pub mod ray;

use vec3d::Vec3d;
use color::Color;
use ray::Ray;

fn ray_color(ray: Ray<f32>) -> Color<u8> {

}

fn main() {
    const ASPECT_RATIO: usize = (16. / 9.) as usize;
    const IMAGE_HEIGHT: usize = 200;
    const IMAGE_WIDTH: usize = IMAGE_HEIGHT * ASPECT_RATIO;

    let mut canvas: [[i32; IMAGE_HEIGHT]; IMAGE_WIDTH];


}
