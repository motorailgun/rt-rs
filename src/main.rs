pub mod vec3d;
pub mod color;
pub mod ray;

use vec3d::Vec3d;
use color::Color;
use ray::Ray;

/* fn ray_color(ray: Ray<f32>) -> Color<f32> {
    let ray_dir = ray.direction; 
    let factor = (ray_dir.unitize().y + 1.) * 0.5;
    let base
} */

fn main() {
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMAGE_HEIGHT: usize = 200;
    const IMAGE_WIDTH: usize = ((IMAGE_HEIGHT as f32) * ASPECT_RATIO) as usize;

    let canvas: Vec<Vec<Color<f32>>> = (0..IMAGE_HEIGHT).into_iter().map(|y: usize| {
        (0..IMAGE_WIDTH).into_iter().map(|x: usize| {
            Color{
                r: x as f32 / IMAGE_WIDTH as f32,
                g: y as f32 / IMAGE_HEIGHT as f32,
                b: 0.4,
            }
        }).collect()
    }).collect();


    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, 255);

    canvas.iter().for_each(|line| {
        let mut pr_string: String = String::from("");
        line.iter().for_each(|pixel| {
            let pixel_int = match pixel.to_int::<u8>() {
                Some(px) => px,
                None => panic!("Error converting pixel to integer format")
            };

            pr_string = format!("{} {}", pr_string, pixel_int);
        });
        println!("{}", pr_string);
    })
}
