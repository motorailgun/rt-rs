#[allow(unused_imports)]
use num_traits::{Float, Num, PrimInt};
use rt_rs::prelude::*;
use std::fmt::Display;

fn out_image<T: Float, U: PrimInt + Display>(
    canvas: &Vec<Vec<Color<T>>>,
    width: usize,
    height: usize,
) {
    let level = U::max_value().to_u64().unwrap() + 1u64;
    println!("P3");
    println!("{width} {height}");
    println!("{level}");

    for line in canvas.iter() {
        for pixel in line.iter() {
            let pix_i = pixel.to_int::<U>();
            print!("{} {} {} ", pix_i.x, pix_i.y, pix_i.z);
        }
        println!();
    }
}

fn calc_color<T: Float + From<f64>>(incoming_ray: Ray<T>) -> Color<T> {
    let unit_direction = incoming_ray.direction.unit_vector();
    let t = (unit_direction.y + 1.0.into()) * 0.5.into();

    Color::<T> {
        x: 1.0.into(),
        y: 1.0.into(),
        z: 1.0.into(),
    } * (-t + 1.0.into())
        + Color::<T> {
            x: 0.5.into(),
            y: 0.7.into(),
            z: 1.0.into(),
        } * t
}

fn render<T: Float + From<f64>>(
    width: usize,
    height: usize,
    ray_generator: &dyn Fn(usize, usize) -> Ray<T>,
    color_generator: fn(Ray<T>) -> Color<T>,
) -> Vec<Vec<Color<T>>> {
    (0..height)
        .collect::<Vec<usize>>()
        .iter()
        .enumerate()
        .map(|(y, _)| {
            let y = height - 1 - y;

            (0..width)
                .collect::<Vec<usize>>()
                .iter()
                .enumerate()
                .map(|(x, _)| -> Color<T> {
                    let ray = ray_generator(x, y);
                    color_generator(ray)
                })
                .collect()
        })
        .collect()
}

fn main() {
    #[allow(non_camel_case_types)]
    type float_t = f64;

    let aspect_ratio: float_t = 16. / 9.;
    let image_width: usize = 400;
    let image_height = (400 as float_t / aspect_ratio) as usize;

    let viewport_height = 2.;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.;

    let origin = Point3::<float_t> {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    let horizontal = Vec3::<float_t> {
        x: viewport_width,
        y: 0.,
        z: 0.,
    };
    let vertical = Vec3::<float_t> {
        x: 0.,
        y: viewport_height,
        z: 0.,
    };
    let lower_left_corner = origin
        - horizontal / 2.
        - vertical / 2.
        - Vec3 {
            x: 0.,
            y: 0.,
            z: focal_length,
        };

    let ray_generator = |x: usize, y: usize| {
        let u = x as float_t / (image_width as float_t - 1.);
        let v = y as float_t / (image_height as float_t - 1.);

        Ray::<float_t> {
            start: origin,
            direction: lower_left_corner + horizontal * u + vertical * v - origin,
        }
    };

    let canvas: Vec<Vec<Color<float_t>>> =
        render(image_width, image_height, &ray_generator, calc_color);
    out_image::<float_t, u8>(&canvas, image_width, image_height);
}
