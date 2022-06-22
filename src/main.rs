pub mod vec3d;
pub mod color;

use vec3d::Vec3d;
use color::Color;

fn ray_color(ray: Vec3d<f64>) -> Color<u8> {
    
}

fn main() {
    const ASPECT_RATIO: usize = (16. / 9.) as usize;
    const IMAGE_HEIGHT: usize = 200;
    const IMAGE_WIDTH: usize = IMAGE_HEIGHT * ASPECT_RATIO;

    let mut canvas: [[i32; IMAGE_HEIGHT]; IMAGE_WIDTH];


}
