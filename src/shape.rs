use crate::{Tuple, normalize};
use crate::{intersections::Intersection, ray::{Ray, transform}, matrix::Matrix};

pub trait Shape {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> where Self: Sized;
    fn get_transform(&self) -> Matrix<f32, 4, 4>;
    fn set_transform(&mut self, transform: Matrix<f32, 4, 4>);
    fn local_normal_at(&self, point: Tuple) -> Tuple;
}

pub fn intersect<T: Shape>(shape: T, ray: Ray) -> Vec<Intersection> {
    let local_ray = transform(ray, shape.get_transform().inverse().unwrap());
    shape.local_intersect(local_ray)
}

pub fn normal_at<T: Shape>(shape: T, point: Tuple) -> Tuple {
    let inverse = shape.get_transform().inverse().unwrap();
    let local_point = inverse * point;
    let local_normal = shape.local_normal_at(local_point);
    let mut world_normal = inverse.transpose() * local_normal;
    world_normal.w = 0.;

    normalize(world_normal)
}