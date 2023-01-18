use crate::camera::Camera;
use crate::primitives::Sphere;

#[derive(Debug)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub camera: Camera,
    pub objects: Vec<Sphere>,
    pub depth: usize,
}

