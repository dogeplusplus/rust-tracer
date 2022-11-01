use crate::{
    group::Group,
    intersections::Intersection,
    materials::Material,
    matrix::Matrix,
    point,
    ray::{transform, Ray},
    vector,
};
use crate::{normalize, Tuple};

pub trait Shape<'a> {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection>
    where
        Self: Sized;
    fn get_transform(&self) -> Matrix<f32, 4, 4>;
    fn set_transform(&mut self, transform: Matrix<f32, 4, 4>);
    fn set_parent(&mut self, parent: &'a Group);
    fn local_normal_at(&self, point: Tuple) -> Tuple;
}

pub fn intersect<'a, T: Shape<'a>>(shape: &T, ray: Ray) -> Vec<Intersection> {
    let local_ray = transform(ray, shape.get_transform().inverse().unwrap());
    shape.local_intersect(local_ray)
}

pub fn normal_at<'a, T: Shape<'a>>(shape: &T, point: Tuple) -> Tuple {
    let inverse = shape.get_transform().inverse().unwrap();
    let local_point = inverse * point;
    let local_normal = shape.local_normal_at(local_point);
    let mut world_normal = inverse.transpose() * local_normal;
    world_normal.w = 0.;

    normalize(world_normal)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestShape<'a> {
    pub material: Material,
    pub saved_ray: Ray,
    pub transform: Matrix<f32, 4, 4>,
    pub parent: Option<&'a Group<'a>>,
}

impl TestShape<'_> {
    pub fn new() -> Self {
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        TestShape {
            material: Material::default(),
            saved_ray: Ray {
                origin: point(0., 0., 0.),
                direction: vector(0., 0., 0.),
            },
            transform: identity,
            parent: None,
        }
    }
}

impl<'a> Shape<'a> for TestShape<'a> {
    fn get_transform(&self) -> Matrix<f32, 4, 4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
        self.transform = transform
    }

    fn set_parent(&mut self, parent: &'a Group) {
        self.parent = Some(parent);
    }

    fn local_intersect(&self, _: Ray) -> Vec<Intersection> {
        Vec::new()
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        point
    }
}
