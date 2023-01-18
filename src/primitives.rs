use nalgebra::Vector3;
use crate::{ray::*, material::*};
use crate::aabb::AABB;


pub trait Primitive : Send + Sync{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material:  Material) -> Sphere {
        Sphere { center, radius, material }
    }

}

impl Primitive for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant >= 0. {
            let sqrtd = discriminant.sqrt();
            let root_a = (-half_b - sqrtd) / a;
            let root_b = (-half_b + sqrtd) / a;
            for root in [root_a, root_b].iter() {
                if *root < t_max && *root > t_min {
                    let p = ray.at(*root);
                    let normal = (p - self.center) / self.radius;
                    let front_face = ray.direction.dot(&normal) < 0.;

                    return Some(HitRecord { 
                        position: p,
                        normal: if front_face { normal } else { -normal }, 
                        t: *root, 
                        front_face,
                        material: &self.material,
                    });
                }
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        let bounding_box = AABB::new(
            self.center - Vector3::new(self.radius, self.radius, self.radius),
            self.center + Vector3::new(self.radius, self.radius, self.radius),
        );

        Some(bounding_box)
    }
}
