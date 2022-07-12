use crate::intersections::Intersection;
use crate::{Tuple,dot,point};
use crate::ray::Ray;


#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Sphere {
    pub center: Tuple,
    pub radius: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere { center: point(0., 0., 0.), radius: 1. }
    }
}

pub fn intersect(sphere: Sphere, ray: Ray) -> Vec<Intersection> {
    let sphere_to_ray = ray.origin - point(0., 0., 0.);
    let a = dot(ray.direction, ray.direction);
    let b = 2. * dot(ray.direction, sphere_to_ray);
    let c = dot(sphere_to_ray, sphere_to_ray) - 1.;

    let discriminant = b * b - 4. * a * c;

    if discriminant < 0. {
        Vec::new()
    } else {
        let t1 = (-b - f32::sqrt(discriminant)) / (2. * a);
        let t2 = (-b + f32::sqrt(discriminant)) / (2. * a);
        vec![Intersection::new(t1, sphere), Intersection::new(t2, sphere)]
    }
}