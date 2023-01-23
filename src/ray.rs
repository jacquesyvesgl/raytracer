use nalgebra::Vector3;

use crate::material::Material;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vector3<f32>{
        self.origin + self.direction.scale(t)
    }

    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
        Ray { origin, direction }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HitRecord<'material>{
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub t: f32,
    pub material: &'material Material,
    pub front_face: bool,
    pub incoming: Vector3<f32>,
}

