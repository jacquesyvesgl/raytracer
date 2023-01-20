use nalgebra::Vector3;
use rand::prelude::*;
use crate::material::*;
use crate::config::Config;
use crate::color::Color;
use crate::camera::Camera;
use crate::primitives::*;


pub fn final_scene() -> Config {

    let ground_material = Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    let mut objects = Vec::<Box<dyn Primitive>>::new();

    objects.push(Box::new(Sphere{ 
        center: Vector3::new(0., -1000., 0.),
        radius: 1000.,
        material: ground_material,
    }));

    let material1 = Material::Dielectric(Dielectric::new(1.5));
    objects.push(Box::new(Sphere { center: Vector3::new(0., 1., 0.), radius: 1.0, material: material1 }));
    let material2 = Material::Lambertian(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    objects.push(Box::new(Sphere { center: Vector3::new(-4., 1., 0.), radius: 1.0, material: material2 }));
    let material3 = Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    objects.push(Box::new(Sphere { center: Vector3::new(4., 1., 0.), radius: 1.0, material: material3 }));

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
                    objects.push(Box::new(Sphere { center, radius: 0.2, material: sphere_material }));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random();
                    let fuzz = rng.gen::<f32>() * 0.5;
                    let sphere_material = Material::Metal(Metal::new(albedo, fuzz));
                    objects.push(Box::new(Sphere { center, radius: 0.2, material: sphere_material }));
                } else {
                    let sphere_material = Material::Dielectric(Dielectric::new(1.5));
                    objects.push(Box::new(Sphere { center, radius: 0.2, material: sphere_material }))
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

pub fn three_balls() -> Config {
    let ground_material = Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let material_center = Material::Lambertian(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let glass_center = Material::Dielectric(Dielectric::new(1.5));
    let golden = Material::Metal(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));
    let glass = Material::Dielectric(Dielectric::new(1.5));
    let glass_inside = Material::Dielectric(Dielectric::new(1.5));

    let scene = Config {
        height: 360,
        width: 640,
        samples_per_pixel: 10,
        depth: 50,
        camera: Camera::new(
            Vector3::new(-2., 2., 1.), 
            Vector3::new(0., 0., -1.),
            Vector3::new(0., 1., 0.),
            20.,
            16./9.
            ),
        objects: vec![
            Box::new(Sphere{
                center: Vector3::new(0., -100.5, -1.),
                radius: 100.,
                material: ground_material,
            }),
            Box::new(Sphere{
                center: Vector3::new(0., 0., -1.),
                radius: 0.45,
                material: material_center,
            }),
            Box::new(Sphere{
                center: Vector3::new(0., 0., -1.),
                radius: 0.5,
                material: glass_center,
            }),
            Box::new(Sphere{
                center: Vector3::new(-1., 0., -1.),
                radius: 0.5,
                material: glass,
            }),
            Box::new(Sphere{
                center: Vector3::new(-1., 0., -1.),
                radius: -0.45,
                material: glass_inside,
            }),
            Box::new(Sphere{
                center: Vector3::new(1., 0., -1.),
                radius: 0.5,
                material: golden,
            }),
        ]
    };

    scene
}

pub fn simple_light() -> Config {
    let ground_material = Material::Lambertian(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let sphere = Material::Lambertian(Lambertian::new(Color::new(0.8, 0.8, 0.8)));
    let light = Material::Light(Light::new(Color::new(4., 4., 4.)));

    let scene = Config {
        height: 360,
        width: 640,
        samples_per_pixel: 100,
        depth: 50,
        camera: Camera::new(
            Vector3::new(26., 3., 6.), 
            Vector3::new(0., 2., 0.),
            Vector3::new(0., 1., 0.),
            20.,
            16./9.),
        objects: vec![
            Box::new(Sphere{
                center: Vector3::new(0., -1000., 0.),
                radius: 1000.,
                material: ground_material,
            }),
            Box::new(Sphere{
                center: Vector3::new(0., 2., 0.),
                radius: 2.,
                material: sphere,
            }),
            Box::new(RectangleXY::new(3., 5., 1., 3., -2., light)),
        ]

    };

    scene
}

pub fn cornell_box() -> Config {
    let red = Material::Lambertian(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let green = Material::Lambertian(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let floor = Material::Lambertian(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let back = Material::Lambertian(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let front = Material::Lambertian(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ceiling = Material::Lambertian(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let light = Material::Light(Light::new(Color::new(15., 15., 15.)));

    let scene = Config {
        height: 400,
        width: 400,
        samples_per_pixel: 10,
        depth: 50,
        camera: Camera::new(
            Vector3::new(278., 278., -800.), 
            Vector3::new(278., 278., 0.),
            Vector3::new(0., 1., 0.),
            40.,
            1.),
        objects: vec![
            Box::new(RectangleYZ::new(0., 555., -1000., 555., 555., green)),
            Box::new(RectangleYZ::new(0., 555., -1000., 555., 0., red)),
            Box::new(RectangleXZ::new(213., 343., 227., 332., 554., light)),
            Box::new(RectangleXZ::new(0., 555., -1000., 555., 0., floor)),
            Box::new(RectangleXZ::new(0., 555., -1000., 555., 555., ceiling)),
            Box::new(RectangleXY::new(0., 555., 0., 555., 555., back)),
            Box::new(RectangleXY::new(0., 555., 0., 555., -1000., front)),
        ]
    };

    scene
}
