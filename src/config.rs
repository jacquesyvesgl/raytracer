use crate::camera::Camera;
use crate::primitives::Primitive;

pub struct Config {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub camera: Camera,
    pub objects: Vec<Box<dyn Primitive>>,
    pub depth: usize,
}

