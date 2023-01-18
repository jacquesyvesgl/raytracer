use crate::{ray::Ray};
use nalgebra::Vector3;

#[cfg(test)]
use assert_approx_eq::assert_approx_eq;

#[derive(Debug)]
pub struct Camera {
    pub origin: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub focal_length: f32,
    pub lower_left_corner: Vector3<f32>,
    look_from: Vector3<f32>,
    look_at: Vector3<f32>,
    vup: Vector3<f32>, // Vertical up, define the rotation
    vfov: f32, // Vertical FOV
    aspect_ratio: f32,
}

#[derive(Debug)]
pub struct CameraParams {
    pub look_from: Vector3<f32>,
    pub look_at: Vector3<f32>,
    pub vup: Vector3<f32>, // Vertical up, define the rotation
    pub vfov: f32, // Vertical FOV
    pub aspect_ratio: f32,
}

impl Camera {
    pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, vup: Vector3<f32>, vfov: f32, aspect_ratio: f32) -> Camera {
        let theta = vfov.to_radians();
        let half_height = (theta / 2.).tan();
        let half_width = aspect_ratio * half_height;

        // Define an orthonormal basis for the camera
        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = look_from;
        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = u * 2. * half_width;
        let vertical = v * 2. * half_height;

        Camera { 
            origin,
            horizontal,
            vertical,
            focal_length: (look_from - look_at).norm(), 
            lower_left_corner,
            look_from,
            look_at,
            vup,
            vfov,
            aspect_ratio
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
            )
    }
}

#[test]
fn test_camera() {
    let camera = Camera::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        90.0,
        (800.0 / 600.0) as f32,
    );
    assert_eq!(camera.origin.x, 0.0);
    assert_eq!(camera.origin.y, 0.0);
    assert_eq!(camera.origin.z, 0.0);

    assert_approx_eq!(camera.lower_left_corner.x, -(1.0 + (1.0 / 3.0)));
    assert_approx_eq!(camera.lower_left_corner.y, -1.0);
    assert_approx_eq!(camera.lower_left_corner.z, -1.0);
}

#[test]
fn test_camera_get_ray() {
    let camera = Camera::new(
        Vector3::new(-4.0, 4.0, 1.0),
        Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, 1.0, 0.0),
        160.0,
        (800 / 600) as f32,
    );
    let ray = camera.get_ray(0.5, 0.5);
    assert_eq!(ray.origin.x, -4.0);
    assert_eq!(ray.origin.y, 4.0);
    assert_eq!(ray.origin.z, 1.0);

    assert_approx_eq!(ray.direction.x, (2.0 / 3.0));
    assert_approx_eq!(ray.direction.y, -(2.0 / 3.0));
    assert_approx_eq!(ray.direction.z, -(1.0 / 3.0));
}

