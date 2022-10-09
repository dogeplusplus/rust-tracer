use crate::intersections::Intersection;
use crate::materials::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::world::ShapeEnum;
use crate::Tuple;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Plane {
    pub transform: Matrix<f32, 4, 4>,
    pub material: Material,
}

impl Default for Plane {
    fn default() -> Self {
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);

        let mater = Material::default();
        Plane {
            transform: identity,
            material: mater,
        }
    }
}

impl Shape for Plane {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        if ray.direction.y.abs() < f32::EPSILON {
            Vec::new()
        } else {
            let t = -ray.origin.y / ray.direction.y;
            vec![Intersection::new(t, ShapeEnum::Plane(*self))]
        }
    }

    fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
        self.transform = transform;
    }

    fn get_transform(&self) -> Matrix<f32, 4, 4> {
        self.transform
    }

    fn local_normal_at(&self, _: Tuple) -> Tuple {
        Tuple::new(0., 1., 0., 0.)
    }
}
