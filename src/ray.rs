use crate::Tuple;

#[derive(Debug,Clone,Copy)]
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