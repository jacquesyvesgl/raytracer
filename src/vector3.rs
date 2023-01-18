use nalgebra::Vector3;
use rand::prelude::*;

pub fn random_vector3(min: f32, max: f32) -> Vector3<f32> {
    let mut rng = thread_rng();
    let x = min + rng.gen::<f32>() * (max - min);
    let y = min + rng.gen::<f32>() * (max - min);
    let z = min + rng.gen::<f32>() * (max - min);

    Vector3::new(x, y, z)
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    loop {
        let p = random_vector3(-1., 1.);
        if p.norm_squared() >= 1. {
            continue;
        }
        return p
    }
}

pub fn random_unit_vector() -> Vector3<f32> {
    random_in_unit_sphere()
}

pub fn random_in_hemisphere(normal: &Vector3<f32>) -> Vector3<f32> {
    let in_unit_sphere = random_unit_vector();
    if in_unit_sphere.dot(normal) > 0. {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        in_unit_sphere.scale(-1.)
    }
}

pub fn near_zero(v: &Vector3<f32>) -> bool {
    let epsilon = 1e-8;
    v.x.abs() < epsilon && v.y.abs() < epsilon && v.z.abs() < epsilon
}

pub fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2. * v.dot(n) * n
}

pub fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = -uv.dot(n).min(1.);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1. - r_out_perp.norm_squared()).sqrt() * n;
    r_out_perp + r_out_parallel
}
