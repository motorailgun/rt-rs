pub mod vec3d;
pub mod color;
pub mod ray;
pub mod primitives;
pub mod material;
pub mod utils;

use vec3d::Vec3d;
use color::Color;
use ray::Ray;

use crate::{primitives::Sphere, material::sphere_material};

fn ray_color(ray: Ray<f32>) -> Color<f32> {
    let ray_dir = ray.direction; 
    let factor = (ray_dir.unitize().y + 1.) * 0.5;
    let base = Color{r: 0.5, g: 0.7, b: 1.0};
    let sauce = Color{r: 1., g: 1., b: 1.};

    sauce * (1. - factor) + base * factor
}

fn main() {
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMAGE_HEIGHT: usize = 200;
    const IMAGE_WIDTH: usize = ((IMAGE_HEIGHT as f32) * ASPECT_RATIO) as usize;

    let origin = Vec3d{x: 0., y: 0., z: 0.};

    let (lu_x, ru_x) = (-1.0, 1.0);
    let (lu_y, rd_y) = (ru_x / ASPECT_RATIO, -ru_x / ASPECT_RATIO);
    let z_center = 1.0;

    let world = vec![
        Sphere{
            center: Vec3d{x: 0., y: 0., z: 1.},
            radius: 0.5,
            material: sphere_material::Lambertian{
                color: Color{r: 0.8, g: 0.8, b: 0.8}
            }
        },
        Sphere{
            center: Vec3d{x: 0., y: -100.5, z: 1.},
            radius: 100.,
            material: sphere_material::Lambertian{
                color: Color{r: 0.8, g: 0.8, b: 0.8}
            }
        }
    ];

    let canvas: Vec<Vec<Color<f32>>> = (0..IMAGE_HEIGHT).into_iter().map(|y: usize| {
        (0..IMAGE_WIDTH).into_iter().map(|x: usize| {
            let x_factor = x as f32 / IMAGE_WIDTH as f32;
            let y_factor = y as f32 / IMAGE_HEIGHT as f32;

            let ray = Ray{
                start: origin,
                direction: Vec3d {
                    x: lu_x + (ru_x - lu_x) * x_factor,
                    y: lu_y + (rd_y - lu_y) * y_factor,
                    z: z_center,
                }
            };
            ray_color(ray)
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
