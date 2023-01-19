use nalgebra::Vector3;
use crate::{ray::*, material::*};


pub trait Primitive : Send + Sync{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

fn set_face_normal(ray: &Ray, outward_normal: Vector3<f32>) -> (Vector3<f32>, bool) {
    let front_face = ray.direction.dot(&outward_normal) < 0.;
    if front_face {
        (outward_normal, front_face)
    } else {
        (-outward_normal, front_face)
    }
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
                    let outward_normal = (p - self.center) / self.radius;
                    let (normal, front_face) = set_face_normal(ray, outward_normal);

                    return Some(HitRecord { 
                        position: p,
                        normal,
                        front_face,
                        t: *root, 
                        material: &self.material,
                    });
                }
            }
        }
        None
    }
}

pub struct RectangleXY {
    pub x0: f32,
    pub x1: f32,
    pub y0: f32,
    pub y1: f32,
    pub k: f32,
    pub material: Material,
}

impl RectangleXY {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Material) -> RectangleXY {
        RectangleXY { x0, x1, y0, y1, k, material }
    }
}

impl Primitive for RectangleXY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None
        }
        
        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None
        }
        
        let position = ray.at(t);
        let outward_normal = Vector3::new(0., 0., 1.);
        let (normal, front_face) = set_face_normal(ray, outward_normal);

        Some(HitRecord {
            position,
            normal,
            front_face,
            t,
            material: &self.material,
        })
    }
}

pub struct RectangleXZ {
    pub x0: f32,
    pub x1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Material,
}

impl RectangleXZ {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Material) -> RectangleXZ {
        RectangleXZ { x0, x1, z0, z1, k, material }
    }
}

impl Primitive for RectangleXZ {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None
        }
        
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None
        }
        
        let position = ray.at(t);
        let outward_normal = Vector3::new(0., 0., 1.);
        let (normal, front_face) = set_face_normal(ray, outward_normal);

        Some(HitRecord {
            position,
            normal,
            front_face,
            t,
            material: &self.material,
        })
    }
}

pub struct RectangleYZ {
    pub y0: f32,
    pub y1: f32,
    pub z0: f32,
    pub z1: f32,
    pub k: f32,
    pub material: Material,
}

impl RectangleYZ {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Material) -> RectangleYZ {
        RectangleYZ { y0, y1, z0, z1, k, material }
    }
}

impl Primitive for RectangleYZ {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None
        }
        
        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None
        }
        
        let position = ray.at(t);
        let outward_normal = Vector3::new(0., 0., 1.);
        let (normal, front_face) = set_face_normal(ray, outward_normal);

        Some(HitRecord {
            position,
            normal,
            front_face,
            t,
            material: &self.material,
        })
    }
}
