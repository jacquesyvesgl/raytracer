use nalgebra::Vector3;
use rand::prelude::*;
use crate::material::*;
use crate::config::Config;
use crate::color::Color;
use crate::camera::Camera;
use crate::primitives::Sphere;


pub fn get_final_scene() -> Config {

    let ground_material = Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    let mut objects = Vec::<Sphere>::new();

    objects.push(Sphere{ 
        center: Vector3::new(0., -1000., 0.),
        radius: 1000.,
        material: ground_material,
    });

    let material1 = Material::Dielectric(Dielectric::new(1.5));
    objects.push(Sphere { center: Vector3::new(0., 1., 0.), radius: 1.0, material: material1 });
    let material2 = Material::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    objects.push(Sphere { center: Vector3::new(-4., 1., 0.), radius: 1.0, material: material2 });
    let material3 = Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    objects.push(Sphere { center: Vector3::new(4., 1., 0.), radius: 1.0, material: material3 });

    let mut rng = thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vector3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + rng.gen::<f32>());

            if (center - Vector3::new(4., 0.2, 0.)).norm() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Material::Lambertian(Lambertian::new(albedo));
                    objects.push(Sphere { center, radius: 0.2, material: sphere_material });
                } else if choose_mat < 0.95 {
                    let albedo = Color::random();
                    let fuzz = rng.gen::<f32>() * 0.5;
                    let sphere_material = Material::Metal(Metal::new(albedo, fuzz));
                    objects.push(Sphere { center, radius: 0.2, material: sphere_material });
                } else {
                    let sphere_material = Material::Dielectric(Dielectric::new(1.5));
                    objects.push(Sphere { center, radius: 0.2, material: sphere_material })
                }
            }
        }
    }

    let final_scene = Config {
        height: 200,
        width: 300,
        samples_per_pixel: 50,
        depth: 50,
        camera: Camera::new(
            Vector3::new(13., 2., 3.), 
            Vector3::new(0., 0., 0.), 
            Vector3::new(0., 1., 0.),
            20., 
            3./2.),
        objects,
    };

    final_scene
}

pub fn get_three_balls() -> Config {
    let ground_material = Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let material_center = Material::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let golden = Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    let glass = Material::Dielectric(Dielectric::new(1.5));
    let glass_inside = Material::Dielectric(Dielectric::new(1.5));

    let scene = Config {
        height: 90,
        width: 160,
        samples_per_pixel: 20,
        depth: 50,
        camera: Camera::new(
            Vector3::new(-2., 2., 1.), 
            Vector3::new(0., 0., -1.),
            Vector3::new(0., 1., 0.),
            20.,
            16./9.
            ),
        objects: vec![
            Sphere{
                center: Vector3::new(0., -100.5, -1.),
                radius: 100.,
                material: ground_material,
            },
            Sphere{
                center: Vector3::new(0., 0., -1.),
                radius: 0.5,
                material: material_center,
            },
            Sphere{
                center: Vector3::new(-1., 0., -1.),
                radius: 0.5,
                material: glass,
            },
            Sphere{
                center: Vector3::new(-1., 0., -1.),
                radius: -0.45,
                material: glass_inside,
            },
            Sphere{
                center: Vector3::new(1., 0., -1.),
                radius: 0.5,
                material: golden,
            },
        ]
    };

    scene
}
