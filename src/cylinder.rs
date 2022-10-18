use crate::{
    materials::Material,
    matrix::Matrix,
    ray::Ray,
    intersections::Intersection,
    shape::Shape,
    world::ShapeEnum, vector, Tuple,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cylinder {
    transform: Matrix<f32, 4, 4>,
    pub material: Material,
}

impl Default for Cylinder {
    fn default() -> Self {
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        Cylinder {
            transform: identity,
            material: Material::default(),
        }
    }
}

impl Shape for Cylinder {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let a = f32::powi(ray.direction.x, 2) + f32::powi(ray.direction.z, 2);
        if a == 0. {
            return Vec::new()
        }

        let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
        let c = f32::powi(ray.origin.x, 2) + f32::powi(ray.origin.z, 2) - 1.;
        let disc = f32::powi(b, 2) - 4. * a * c;

        if disc < 0. {
            Vec::new()
        } else {
            vec![
                Intersection::new(1., ShapeEnum::Cylinder(*self)),
            ]
        }
        
    }

    fn get_transform(&self) -> Matrix<f32, 4, 4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
        self.transform = transform;
    }

    fn local_normal_at(&self, _point: Tuple) -> Tuple {
        vector(0., 0., 0.)
    }
}