use nalgebra::Vector3;
use crate::ray::Ray;
use crate::primitives::*;
// use crate::bhv::BHV;

pub struct AABB {
    pub minimum: Vector3<f32>,
    pub maximum: Vector3<f32>,
}

impl AABB {
    pub fn new(minimum: Vector3<f32>, maximum: Vector3<f32>) -> AABB {
        AABB { minimum, maximum }
    }

    pub fn hit(&self, ray: &Ray, t_min: &mut f32, t_max: &mut f32) -> bool {
        for i in 0..3 {
            let inv_d = 1. / ray.direction[i];
            let mut t0 = (self.minimum[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.maximum[i] - ray.origin[i]) * inv_d;

            if inv_d < 0. {
                std::mem::swap(&mut t0, &mut t1);
            }
            *t_min = if t0 > *t_min { t0 } else { *t_min };
            *t_max = if t1 < *t_max { t1 } else { *t_max };

            if t_max < t_min {
                return false
            }
        }
        return true;
    }
}

pub fn surrounding_box(box0: Option<AABB>, box1: Option<AABB>) -> Option<AABB> {
    match (box0, box1) {
        (Some(b), None) | (None, Some(b)) => { // Would have written Some(box) if box wasn't a
                                               // reserved Rust keyword...
            Some(AABB::new(b.minimum, b.maximum))
        }
        (Some(box0), Some(box1)) => {

            let small = Vector3::new(
                box0.minimum.x.min(box1.minimum.x),
                box0.minimum.y.min(box1.minimum.y),
                box0.minimum.z.min(box1.minimum.z));

            let big = Vector3::new(
                box0.maximum.x.min(box1.maximum.x),
                box0.maximum.y.min(box1.maximum.y),
                box0.maximum.z.min(box1.maximum.z));

            Some(AABB::new(small, big))
        }
        (None, None) => None
    }
}

pub fn bounding_box(world: &Vec<Sphere>) -> Option<AABB> {
    if world.is_empty() { return None }

    let mut first_box = true;
    let mut output_box = None;

    for sphere in world {
        match sphere.bounding_box() {
            Some(bounding_box) => {
                output_box = if first_box { 
                    Some(bounding_box)
                } else {
                    first_box = false;
                    surrounding_box(output_box, Some(bounding_box))
                } 
            }
            None => (),
        }
    }

    output_box
}

// pub fn box_compare(a: &BHV, b: &BHV, axis: usize) -> bool {
//     true
// }


// pub fn box_x_compare(a: &BHV, b: &BHV) -> bool {
//     box_compare(a, b, 0)
// }

// pub fn box_y_compare(a: &BHV, b: &BHV) -> bool {
//     box_compare(a, b, 1)
// }

// pub fn box_z_compare(a: &BHV, b: &BHV) -> bool {
//     box_compare(a, b, 2)
// }
