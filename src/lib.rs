use std::ops::{Add, Sub, Neg, Mul, Div};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn is_vector(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 0.0
    }
}

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.w + other.w == 2.0 {
            panic!("Cannot add point to a point")
        } else {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
                w: self.w + other.w,
            }
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.w == 0. && other.w == 1. {
            panic!("Cannot subtract point from vector")
        }
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y, z: -self.z, w: -self.w }
    }
}

pub fn point(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f32, y: f32, z: f32) -> Tuple {
    Tuple::new(x, y, z, 0.0)
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

pub fn magnitude(t: Tuple) -> f32 {
    (t.x * t.x + t.y * t.y + t.z * t.z + t.w * t.w).sqrt()
}

pub fn normalize(t: Tuple) -> Tuple {
    t / magnitude(t)
}

pub fn dot(a: Tuple, b: Tuple) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

pub fn cross(a: Tuple, b: Tuple) -> Tuple {
    vector(a.y*b.z - a.z*b.y, a.z*b.x - a.x*b.z, a.x*b.y - a.y*b.x)
}

pub struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

pub struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    Projectile{position, velocity}
}