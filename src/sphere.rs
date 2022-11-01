use crate::group::Group;
use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::world::ShapeEnum;
use crate::{dot, normalize, point, Tuple};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere<'a> {
    pub center: Tuple,
    pub radius: f32,
    pub transform: Matrix<f32, 4, 4>,
    pub material: Material,
    pub parent: Option<&'a Group<'a>>,
}

impl Default for Sphere<'_> {
    fn default() -> Self {
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
            material: Material::default(),
            parent: None,
        }
    }
}

impl<'a> Shape<'a> for Sphere<'a> {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
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
            vec![
                Intersection::new(t1, ShapeEnum::Sphere(*self)),
                Intersection::new(t2, ShapeEnum::Sphere(*self)),
            ]
        }
    }

    fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
        self.transform = transform;
    }

    fn set_parent(&mut self, parent: &'a Group) {
        self.parent = Some(parent);
    }

    fn get_transform(&self) -> Matrix<f32, 4, 4> {
        self.transform
    }

    fn local_normal_at(&self, world_point: Tuple) -> Tuple {
        let mut world_normal = world_point;
        world_normal.w = 0.0;
        normalize(world_normal)
    }
}

pub fn glass_sphere() -> Sphere<'static> {
    let mut s = Sphere::default();
    s.material.transparency = 1.0;
    s.material.refractive_index = 1.5;
    s
}
