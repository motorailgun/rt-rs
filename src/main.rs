pub mod vec3d;
pub mod color;
pub mod ray;
pub mod primitives;
pub mod material;
pub mod utils;

use std::fmt::Display;

use vec3d::Vec3d;
use color::Color;
use ray::Ray;
use utils::*;

use crate::{primitives::{Sphere, Primitive, HitRecord}, material::sphere_material};

fn ray_color<T: FloatU + Display>(ray: Ray<T>, world: &Vec<Box<dyn Primitive<T>>>, count: usize) -> Color<T> {
    if count == 0 {
        let zero = T::from(0f64).expect("no known conversion from f64 to T");
        return Color::<T>{r: zero, g: zero, b: zero}
    }

    let min: T = 1e-6.tt();
    let hit_list: Vec<Option<HitRecord<T>>> = world.iter()
                                              .map(|prim| prim.intersect(&ray, min, T::infinity()))
                                              .collect();
    let to_what = hit_list.into_iter()
                                        .filter_map(|record| record)
                                        .fold(None::<HitRecord<T>>, |post, record| {
                                            match post {
                                                Some(post_rec) => {
                                                    if post_rec.t > record.t {
                                                        Some(record)
                                                    } else {
                                                        Some(post_rec)
                                                    }
                                                },
                                                None => Some(record),
                                            }
                                        });

    match to_what {
        Some(record) => {
            record.prim.color()
        },
        None => {
            let ray_dir = ray.direction; 
            let factor: T = (ray_dir.unitize().y + 1.0.tt::<T>()) * 0.5.tt();
            let base: Color<T> = Color{r: 0.5.tt(), g: 0.7.tt(), b: 1.0.tt()};
            let sauce: Color<T> = Color{r: 1.0.tt(), g: 1.0.tt(), b: 1.0.tt()};

            sauce * (1.0.tt::<T>() - factor) + base * factor
        },
    }
}

fn main() {
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize;
    const RAY_COUNT: usize = 100;
    const SAMPLE_COUNT: usize = 100;

    let origin = Vec3d{x: 0., y: 0., z: 0.};

    let viewport_height: f32 = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let z_center = -1.0;

    let horizontal = Vec3d{x: viewport_width, y: 0., z: 0.};
    let vertical = Vec3d{x: 0., y: viewport_height, z: 0.};
    let upper_left_corner = origin - horizontal / 2. + vertical / 2. + Vec3d{x: 0., y: 0., z: z_center};

    let world: Vec<Box<dyn Primitive<f32>>> = vec![
        Box::new(Sphere{
            center: Vec3d{x: 0., y: 0., z: -1.},
            radius: 0.5,
            material: sphere_material::Lambertian{},
            color: Color{r: 0.8, g: 0.0, b: 0.0},
        }),
        Box::new(Sphere{
            center: Vec3d{x: 0., y: -100.5, z: -1.},
            radius: 100.,
            material: sphere_material::Lambertian{},
            color: Color{r: 0.0, g: 0.8, b: 0.0},
        })
    ];

    let canvas: Vec<Vec<Color<f32>>> = (0..IMAGE_HEIGHT).into_iter().map(|y: usize| {
        (0..IMAGE_WIDTH).into_iter().map(|x: usize| {
            let x_factor = x as f32 / (IMAGE_WIDTH - 1) as f32;
            let y_factor = y as f32 / (IMAGE_HEIGHT - 1) as f32;

            let ray = Ray{
                start: origin,
                direction: upper_left_corner + horizontal * x_factor - vertical * y_factor,
            };
            ray_color(ray, &world, RAY_COUNT)
        }).collect()
    }).collect();


    println!("P3\n{} {}\n{}", IMAGE_WIDTH, IMAGE_HEIGHT, 255);

    let print_result = || {
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
    };

    print_result();
}
