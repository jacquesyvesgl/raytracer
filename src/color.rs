use std::ops::{Add, Sub, Mul};
use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self{ r, g, b }
    }

    pub fn scale(&self, s: f32) -> Self {
        Self { r: s * &self.r, g: s * &self.g, b: s * self.b}
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self{ r: rng.gen(), g: rng.gen(), b: rng.gen() }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

pub const BLACK: Color = Color{r:0., g:0., b:0.};
pub const WHITE: Color = Color{r:1., g:1., b:1.};
pub const RED: Color = Color{r:1., g:0., b:0.};
pub const GREEN: Color = Color{r:0., g:1., b:0.};
pub const BLUE: Color = Color{r:0., g:0., b:1.};
