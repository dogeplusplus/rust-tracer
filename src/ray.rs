use crate::{matrix::Matrix, Tuple};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

pub fn position(ray: Ray, t: f32) -> Tuple {
    ray.origin + ray.direction * t
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Ray { origin, direction }
    }
}

pub fn transform(ray: Ray, matrix: Matrix<f32, 4, 4>) -> Ray {
    Ray::new(matrix * ray.origin, matrix * ray.direction)
}
