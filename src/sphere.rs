use crate::intersections::Intersection;
use crate::matrix::Matrix;
use crate::ray::{transform, Ray};
use crate::{dot, normalize, point, Tuple};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub center: Tuple,
    pub radius: f32,
    pub transform: Matrix<f32, 4, 4>,
}

impl Sphere {
    pub fn new() -> Self {
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        Sphere {
            center: point(0., 0., 0.),
            radius: 1.,
            transform: identity,
        }
    }
}

pub fn intersect(sphere: Sphere, ray: Ray) -> Vec<Intersection> {
    let ray = transform(ray, sphere.transform.inverse().unwrap());
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

pub fn set_transform(sphere: &mut Sphere, transform: Matrix<f32, 4, 4>) {
    sphere.transform = transform;
}

pub fn normal_at(s: Sphere, world_point: Tuple) -> Tuple {
    let inverse = s.transform.inverse().unwrap();
    let object_point = inverse * world_point;
    let object_normal = object_point - point(0., 0., 0.);
    let mut world_normal = inverse.transpose() * object_normal;
    world_normal.w = 0.0;

    normalize(world_normal)
}
