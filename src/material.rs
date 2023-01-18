use rand::Rng;

use crate::{ray::*, ray::HitRecord, color::Color, vector3::*};

pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)>;
}

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    
} 

impl Scatterable for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray, hit_record),
            Material::Metal(m)=> m.scatter(ray, hit_record),
            Material::Dielectric(d) => d.scatter(ray, hit_record),
        }
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
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();
        if near_zero(&scatter_direction) {
            scatter_direction = hit_record.normal;
        }
        let scattered = Ray::new(hit_record.position, scatter_direction);
        let attenuation = self.albedo;
        Some((Some(scattered), attenuation))
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
        let reflected = reflect(&ray.direction, &hit_record.normal);
        let scattered = Ray::new(
            hit_record.position, 
            reflected + random_in_unit_sphere() * self.fuzz);
        let attenuation = self.albedo;
        if scattered.direction.dot(&hit_record.normal) > 0. {
            Some((Some(scattered), attenuation))
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Color)> {
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
            let reflected = reflect(&unit_direction, &hit_record.normal);
            let scattered = Ray::new(hit_record.position, reflected);
            Some((Some(scattered), attenuation))
        } else {
            let refracted = refract(&unit_direction, &hit_record.normal, etai_over_etat);
            let scattered = Ray::new(hit_record.position, refracted);
            Some((Some(scattered), attenuation))
        }
    }
}
