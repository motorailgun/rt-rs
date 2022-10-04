pub mod vec3d;
pub mod color;
pub mod ray;

use vec3d::Vec3d;
use color::Color;
use ray::Ray;

//fn ray_color(ray: Ray<f32>) -> Color<u8> {
//
//}

fn main() {
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMAGE_HEIGHT: usize = 200;
    const IMAGE_WIDTH: usize = ((IMAGE_HEIGHT as f32) * ASPECT_RATIO) as usize;

    let canvas: Vec<Vec<Color<u8>>> = (0..(IMAGE_HEIGHT - 1)).into_iter().map(|y: usize| {
        (0..(IMAGE_WIDTH - 1)).into_iter().map(|x: usize| {
            Color{
                r: (x % 256) as u8,
                g: (y % 256) as u8,
                b: 0u8,
            }
        }).collect()
    }).collect();


    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, 255);

    canvas.iter().for_each(|line| {
        let mut pr_string: String = String::from("");
        line.iter().for_each(|pixel| {
            pr_string = format!("{} {}", pr_string, pixel);
        });
        println!("{}", pr_string);
    })
}
