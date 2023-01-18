use crate::aabb::*;
use crate::ray::{Ray};
use crate::primitives::{Primitive, Sphere};
use rand::prelude::*;

enum Hittable {
    Primitive(Box<dyn Primitive>),
    BHV(BHV),
}

impl Hittable {
    fn bounding_box(&self) -> Option<AABB> {
        match self {
            Hittable::Primitive(p) => p.bounding_box(),
            Hittable::BHV(b) => b.bounding_box(),
        }
    }
}

pub struct BHV {
    bounding_box: Option<AABB>,
    left: Box<Hittable>,
    right: Box<Hittable>,
}

impl BHV {
    fn bounding_box(&self) -> Option<AABB> {
        self.bounding_box
    }

    fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        if let Some(b) = self.bounding_box {
            return false
        }

        match (&*self.left, &*self.right) {
            (Hittable::Primitive(p), Hittable::BHV(bhv)) | (Hittable::BHV(bhv), Hittable::Primitive(p)) => {
                let primitive_hit = match p.hit(ray, t_min, t_max) { None => false, Some(_) => true };
                bhv.hit(ray, t_min, t_max) || primitive_hit
            }
            (Hittable::BHV(left), Hittable::BHV(right)) => {
                left.hit(ray, t_min, t_max) || right.hit(ray, t_min, t_max)
            }
            (Hittable::Primitive(left), Hittable::Primitive(right)) => {
                let left = match left.hit(ray, t_min, t_max) { None => false, Some(_) => true };
                let right = match right.hit(ray, t_min, t_max) { None => false, Some(_) => true };
                left || right
            }
        }
    }

    pub fn new(start: usize, end: usize, world: Vec<Sphere>) -> BHV { // MODIFIER Vec<f32> !!!
        let mut rng = thread_rng();
        
        let objects = world.clone();

        let axis = rng.gen_range(0..=2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
            _ => panic!("Error at comparator chossing"),
        };

        let object_span = end - start;
         
        let left: Box<Hittable>;
        let right: Box<Hittable>;

        match object_span {
            1 => {
                left = Box::new(Hittable::Primitive(Box::new(objects[start])));
                right = Box::new(Hittable::Primitive(Box::new(objects[start])));
            }
            _ => panic!("Error at BHV division"),
        }

        let box_left = left.bounding_box();
        let box_right = right.bounding_box();

        let bounding_box = surrounding_box(box_left, box_right);

        BHV { bounding_box, left, right }
    }
}
