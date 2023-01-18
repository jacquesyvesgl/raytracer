use std::{fs, io::Write};
use rand::prelude::*;

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

pub fn render(scene: &Config) -> Vec<Color> {
    let mut image = Vec::with_capacity(scene.width * scene.height);
    let mut rng = thread_rng();

    for j in (0..scene.height).rev() {
        for i in 0..scene.width {
            let mut color = BLACK;
            for _ in 0..scene.samples_per_pixel {
                let u = i as f32 / scene.width as f32 + rng.gen::<f32>();
                let v = j as f32 / scene.height as f32 + rng.gen::<f32>();
                let ray = scene.camera.get_ray(u, v);
                // color = color + ray_color(&ray, &scene, scene.depth);
                color = color + blue_sky(&ray, &scene, scene.depth);
            }

            image.push(color)
        }
    }

    image
}

pub fn render_and_write(scene: &Config, filename: &str) {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)
        .unwrap();

    file.write_all("P3\n".as_bytes()).expect("write failed");
    file.write_all(format!("{} {}\n", scene.width, scene.height).as_bytes()).expect("write failed");
    file.write_all("255\n".as_bytes()).expect("write failed");

    let scale = 1. / scene.samples_per_pixel as f32;

    for j in (0..scene.height).rev() {
        for i in 0..scene.width {
            let mut color = BLACK;
            for _ in 0..scene.samples_per_pixel {
                let u = i as f32 / scene.width as f32;
                let v = j as f32 / scene.height as f32;
                let ray = scene.camera.get_ray(u, v);
                color = color + ray_color(&ray, &scene, scene.depth);
            }

            let mut r = (scale * color.r).sqrt();
            let mut g = (scale * color.g).sqrt();
            let mut b = (scale * color.b).sqrt();

            r = clamp(r * 255., 0., 255.);
            g = clamp(g * 255., 0., 255.); 
            b = clamp(b * 255., 0., 255.); 

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

