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
                        incoming: ray.direction.clone(),
                    });
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct RectangleXY {
    x0: f32,
    x1: f32,
    y0: f32, 
    y1: f32,
    k: f32,
    material: Material,
}

impl RectangleXY {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material: Material) -> RectangleXY {
        RectangleXY { x0, x1, y0, y1, k, material}
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

        let outward_normal = Vector3::new(0., 0., 1.);
        let (normal, front_face) = set_face_normal(ray, outward_normal);

        return Some(HitRecord {
            position: ray.at(t),
            normal,
            front_face,
            t,
            material: &self.material,
            incoming: ray.direction.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RectangleXZ {
    x0: f32,
    x1: f32,
    z0: f32, 
    z1: f32,
    k: f32,
    material: Material,
}

impl RectangleXZ {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material: Material) -> RectangleXZ {
        RectangleXZ { x0, x1, z0, z1, k, material}
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

        let outward_normal = Vector3::new(0., 1., 0.);
        let (normal, front_face) = set_face_normal(ray, outward_normal);

        return Some(HitRecord {
            position: ray.at(t),
            normal,
            front_face,
            t,
            material: &self.material,
            incoming: ray.direction.clone(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct RectangleYZ {
    y0: f32,
    y1: f32,
    z0: f32, 
    z1: f32,
    k: f32,
    material: Material,
}

impl RectangleYZ {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material: Material) -> RectangleYZ {
        RectangleYZ { y0, y1, z0, z1, k, material}
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
        if z < self.z0 || z > self.z1 || y < self.y0 || y > self.y1 {
            return None
        }

        let outward_normal = Vector3::new(1., 0., 0.);
        let (normal, front_face) = set_face_normal(ray, outward_normal);

        return Some(HitRecord {
            position: ray.at(t),
            normal,
            front_face,
            t,
            material: &self.material,
            incoming: ray.direction.clone(),
        })
    }
}

pub struct RectangularCuboid { // "Box" is a reserved keyword lol
    pub vertice0: Vector3<f32>,
    pub vertice1: Vector3<f32>,
    sides: Vec<Box<dyn Primitive>>,
}

impl RectangularCuboid {
    pub fn new(p0: Vector3<f32>, p1: Vector3<f32>, material: Material) -> RectangularCuboid {
        let mut sides = Vec::<Box<dyn Primitive>>::new();

        sides.push(Box::new(RectangleXY::new(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone())));
        sides.push(Box::new(RectangleXY::new(p0.x, p1.x, p0.y, p1.y, p0.z, material.clone())));

        sides.push(Box::new(RectangleXZ::new(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone())));
        sides.push(Box::new(RectangleXZ::new(p0.x, p1.x, p0.z, p1.z, p0.y, material.clone())));

        sides.push(Box::new(RectangleYZ::new(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone())));
        sides.push(Box::new(RectangleYZ::new(p0.y, p1.y, p0.z, p1.z, p0.x, material.clone())));

        RectangularCuboid {
            vertice0: p0,
            vertice1: p1,
            sides
        }
    }
}

impl Primitive for RectangularCuboid {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // It is the same function as hit_world
        // except the cheeky &
        let mut closest_so_far = t_max;
        let mut hit_record = None;
        for side in &self.sides {
            if let Some(hit) = side.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit)
            }
        }
        hit_record
    }
}

pub struct Translate {
    hittable: Box<dyn Primitive>,
    offset: Vector3<f32>,
}

impl Translate {
    pub fn new(hittable: Box<dyn Primitive>, offset: Vector3<f32>) -> Translate {
        Translate { hittable, offset }
    }
}

impl Primitive for Translate {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_ray = Ray::new(ray.origin - self.offset, ray.direction);
        match self.hittable.hit(&moved_ray, t_min, t_max) {
            None => None,
            Some(hit) => {
                let (normal, front_face) = set_face_normal(&moved_ray, hit.normal);
                Some(HitRecord {
                    position: hit.position + self.offset,
                    normal,
                    front_face,
                    t: hit.t,
                    material: hit.material,
                    incoming: hit.incoming,
                })
            }
        }
    }
}

pub struct RotateY {
    sin_theta: f32,
    cos_theta: f32,
    hittable: Box<dyn Primitive>,
}

impl RotateY {
    pub fn new(angle: f32, hittable: Box<dyn Primitive>) -> RotateY {
        RotateY { sin_theta: angle.sin(), cos_theta: angle.cos(), hittable}
    }
}

impl Primitive for RotateY {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin = Vector3::new(
            self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z,
            ray.origin.y,
            self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z);
        let direction = Vector3::new(
            self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z,
            ray.direction.y,
            self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z);
        let rotated_ray = Ray::new(origin, direction);

        match self.hittable.hit(&rotated_ray, t_min, t_max) {
            None => None,
            Some(hit) => {
                let position = Vector3::new(
                    self.cos_theta * hit.position.x + self.sin_theta * hit.position.z,
                    hit.position.y,
                    - self.sin_theta * hit.position.x + self.cos_theta * hit.position.z);
                let outward_normal = Vector3::new(
                    self.cos_theta * hit.normal.x + self.sin_theta * hit.normal.z,
                    hit.normal.y, 
                    - self.sin_theta * hit.normal.x + self.cos_theta * hit.normal.z);
                let (normal, front_face) = set_face_normal(&rotated_ray, outward_normal);
                Some(HitRecord {
                    position,
                    normal,
                    front_face,
                    t: hit.t,
                    material: hit.material,
                    incoming: hit.incoming,
                })
            }
        }
    }
}
