use std::{fs, io::Write, sync::{mpsc, Arc}, time::Instant};
use rand::prelude::*;
use threadpool::ThreadPool;
use rayon::prelude::*;

use crate::parameters::*;
use crate::ray::{Ray, HitRecord};
use crate::material::Scatterable;
use crate::primitives::*;
use crate::config::Config;
use crate::color::*;

fn hit_world<'material>(
    world: &'material Vec<Box<dyn Primitive>>,
    ray: &Ray, 
    t_min: f32, 
    t_max: f32,
    ) -> Option<HitRecord<'material>> {
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for object in world {
        if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
            closest_so_far = hit.t;
            hit_record = Some(hit);
        }
    }
    hit_record
}

pub fn ray_color(
    ray: &Ray,
    scene: &Config,
    depth: usize,
    ) -> Color {

    if depth <= 0 {
        return BLACK;
    }

    let hit = hit_world(&scene.objects, ray, EPSILON, INF);
    match hit {
        Some(hit_record) => {
            let scatter = hit_record.material.scatter(ray, &hit_record);
            let emitted = hit_record.material.emitted();
            match scatter {
                Some((scattered_ray, attenuation)) => {
                    // Scatter and attenuate by the reflectance (= albedo)
                    emitted + attenuation * ray_color(&scattered_ray, scene, depth - 1)
                }
                None => emitted
            }
        }
        None => {
            // let unit_direction = ray.direction.normalize();
            // let t = 0.5 * (unit_direction.y + 1.);
            // let blue = Color::new(0.5, 0.7, 1.0);

            // WHITE.scale(1. - t) + blue.scale(t)
            println!("hit the void");
            BLACK
            // WHITE.scale(0.1)
        }
    }
}

fn blue_sky(
    ray: &Ray,
    _scene: &Config,
    _depth: usize,
    ) -> Color {
    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.);
    let blue = Color::new(0.5, 0.7, 1.0);

    WHITE.scale(1. - t) + blue.scale(t)
}

pub fn render(scene: Config, filename: &str) {
    let scene = Arc::new(scene);
    let (tx, rx) = mpsc::channel();
    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);

    let mut image = vec![vec![(0, 0, 0); scene.width as usize]; scene.height as usize];

    let scale = 1. / scene.samples_per_pixel as f32;

    let start = Instant::now();
    println!("Starting rendering...");

    for i in 0..scene.height {
        let tx_row = tx.clone();
        let scene = Arc::clone(&scene);
        pool.execute(move || {
            for j in 0..scene.width {
                let mut rng = thread_rng();
                let mut color = BLACK;
                for _ in 0..scene.samples_per_pixel {
                    let u = (j as f32 + rng.gen::<f32>()) / scene.width as f32;
                    let v = (i as f32 + rng.gen::<f32>())/ scene.height as f32;
                    let ray = scene.camera.get_ray(u, v);
                    color = color + ray_color(&ray, &scene, scene.depth);
                }

                let r = (scale * color.r).sqrt();
                let g = (scale * color.g).sqrt();
                let b = (scale * color.b).sqrt();

                // Don't ask why, just admire the result.
                tx_row.send(((scene.height - 1 - i, j), Color::new(r, g, b))).unwrap()

            }
        })
    }

    drop(tx);

    let mut n_pixels_computed = 0;
    let total = scene.width * scene.height;
    let twenty_percent = total / 5;
    for ((i, j), color) in rx {
        n_pixels_computed += 1;
        let r = clamp(color.r * 255., 0., 255.);
        let g = clamp(color.g * 255., 0., 255.); 
        let b = clamp(color.b * 255., 0., 255.); 
        image[i][j] = (r as u8, g as u8, b as u8);
        if n_pixels_computed % twenty_percent == 0 {
            println!("Rendered {} / {} pixels (~ {}%), time elapsed: {:?}",
                     n_pixels_computed,
                     total,
                     (n_pixels_computed as f32 / total as f32) * 100.,
                     start.elapsed())
        }
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();
    
    file.write_all("P3\n".as_bytes()).expect("write failed");
    file.write_all(format!("{} {}\n", scene.width, scene.height).as_bytes()).expect("write failed");
    file.write_all("255\n".as_bytes()).expect("write failed");

    for row in image {
        for (r, g, b) in row {
            file.write_all(format!("{} {} {}\n",r as u8, g as u8, b as u8)
                           .as_bytes()).expect("write failed");
        }
    }

}

fn clamp(number: f32, min: f32, max: f32) -> f32 {
    let mut res = number;
    if number < min {
        res = min
    } else if number > max {
        res = max
    }
    res
}

