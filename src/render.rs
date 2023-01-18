use std::{fs, io::Write, sync::{mpsc, Arc}};
use rand::prelude::*;
use threadpool::ThreadPool;

use crate::parameters::*;
use crate::ray::{Ray, HitRecord};
use crate::material::Scatterable;
use crate::primitives::*;
use crate::config::Config;
use crate::color::*;

fn hit_world<'material>(
    world: &'material Vec<Sphere>,
    ray: &Ray, 
    t_min: f32, 
    t_max: f32,
    ) -> Option<HitRecord<'material>> {
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for sphere in world {
        if let Some(hit) = sphere.hit(ray, t_min, closest_so_far) {
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
            match scatter {
                Some((scattered, albedo)) => {
                    match scattered {
                        // Scatter and attenuate by the reflectance (= albedo)
                        Some(scattered_ray) => albedo * ray_color(&scattered_ray, scene, depth - 1),
                        None => BLACK,
                    }
                }
                None => BLACK
            }
        }
        None => {
            let unit_direction = ray.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.);
            let blue = Color::new(0.5, 0.7, 1.0);

            WHITE.scale(1. - t) + blue.scale(t)
        }
    }
}

pub fn blue_sky(
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
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();
    
    let scene = Arc::new(scene);
    let (tx, rx) = mpsc::channel();
    let n_workers = 8;
    let pool = ThreadPool::new(n_workers);

    let mut image = vec![vec![(0, 0, 0); scene.width as usize]; scene.height as usize];

    let scale = 1. / scene.samples_per_pixel as f32;

    for j in 0..scene.height {
        let tx_row = tx.clone();
        let scene = Arc::clone(&scene);
        pool.execute(move || {
            for i in 0..scene.width {
                let mut color = BLACK;
                for _ in 0..scene.samples_per_pixel {
                    let u = i as f32 / scene.width as f32;
                    let v = j as f32 / scene.height as f32;
                    let ray = scene.camera.get_ray(u, v);
                    color = color + ray_color(&ray, &scene, scene.depth);
                }

                let r = (scale * color.r).sqrt();
                let g = (scale * color.g).sqrt();
                let b = (scale * color.b).sqrt();

                tx_row.send(((i, j), Color::new(r, g, b))).unwrap()

            }
        })
    }

    drop(tx);

    for ((i, j), color) in rx {
        let r = clamp(color.r * 255., 0., 255.);
        let g = clamp(color.g * 255., 0., 255.); 
        let b = clamp(color.b * 255., 0., 255.); 
        image[j][i] = (r as u8, g as u8, b as u8);
    }

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

pub fn write_image(image: Vec<Color>, world: &Config, filename: &str) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    file.write_all("P3\n".as_bytes()).expect("write failed");
    file.write_all(format!("{} {}\n", world.width, world.height).as_bytes()).expect("write failed");
    file.write_all("255\n".as_bytes()).expect("write failed");

    let scale = 1. / world.samples_per_pixel as f32;

    for color in image {
        let mut r = color.r;
        let mut g = color.g;
        let mut b = color.b;

        // Gamma-correct for gamma=2.0
        r = (r * scale).sqrt();
        g = (g * scale).sqrt();
        b = (b * scale).sqrt();

        r = clamp(r * 255., 0., 255.);
        g = clamp(g * 255., 0., 255.); 
        b = clamp(b * 255., 0., 255.); 

        file.write_all(format!("{} {} {}\n",r as u8, g as u8, b as u8)
                       .as_bytes()).expect("write failed");
    }
}

