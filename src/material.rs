use rand::Rng;
use nalgebra::Vector3;

use crate::{ray::*, ray::HitRecord, color::*, vector3::*};

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;

    fn emitted(&self) -> Color {
        BLACK
    }
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    Light(Light),
} 

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m)=> m.scatter(ray, hit_record),
            Material::Dielectric(d) => d.scatter(ray, hit_record),
            Material::Light(l) => l.scatter(ray, hit_record),
        }
    }

    fn emitted(&self) -> Color {
        match self {
            Material::Lambertian(l) => l.emitted(),
            Material::Metal(m) => m.emitted(),
            Material::Dielectric(d) => d.emitted(),
            Material::Light(l) => l.emitted(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Light {
    color: Color,
}

impl Light {
    pub fn new(color: Color) -> Light {
        return Light { color }
    }
}

impl Scatterable for Light {
    fn scatter(&self, _ray: &Ray, _hit_record: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self) -> Color {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + Vector3::random_unit_vector();
        if Vector3::near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;

        }
        let scattered = Ray::new(hit_record.position, scatter_direction);
        let attenuation = self.albedo;
        // println!("Scattered ray:");
        // println!("-- Normal: {:?}", &hit_record.normal);
        // println!("-- Incoming: {:?}", &hit_record.incoming);
        // println!("-- Origin: {:?}", &hit_record.position);
        // println!("-- Direction: {:?}\n", &scatter_direction);
        Some((scattered, attenuation))
    }
}

#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vector3::reflect(&ray.direction, &hit_record.normal);
        let scattered = Ray::new(
            hit_record.position, 
            reflected + Vector3::random_in_unit_sphere() * self.fuzz);
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit_record.normal) > 0. {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub index_of_refraction: f32
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric { index_of_refraction }
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powi(5)
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut rng = rand::thread_rng();
        let attenuation = Color::new(1., 1., 1.);
        let etai_over_etat = if hit_record.front_face { 
            1./self.index_of_refraction 
        } else { 
            self.index_of_refraction 
        };
        let unit_direction = ray.direction.normalize();
        let cos_theta = - unit_direction.dot(&hit_record.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let cannot_refract = etai_over_etat * sin_theta > 1.;
        if cannot_refract || reflectance(cos_theta, etai_over_etat) > rng.gen() {
            let reflected = Vector3::reflect(&unit_direction, &hit_record.normal);
            let scattered = Ray::new(hit_record.position, reflected);
            Some((scattered, attenuation))
        } else {
            let refracted = Vector3::refract(&unit_direction, &hit_record.normal, etai_over_etat);
            let scattered = Ray::new(hit_record.position, refracted);
            Some((scattered, attenuation))
        }
    }
}
