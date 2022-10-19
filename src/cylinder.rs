use crate::{
    intersections::Intersection, materials::Material, matrix::Matrix, ray::Ray, shape::Shape,
    vector, world::ShapeEnum, Tuple,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cylinder {
    transform: Matrix<f32, 4, 4>,
    pub material: Material,
    pub minimum: f32,
    pub maximum: f32,
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
            minimum: -f32::INFINITY,
            maximum: f32::INFINITY,
        }
    }
}

impl Shape for Cylinder {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let a = f32::powi(ray.direction.x, 2) + f32::powi(ray.direction.z, 2);
        if a == 0. {
            return Vec::new();
        }

        let b = 2. * ray.origin.x * ray.direction.x + 2. * ray.origin.z * ray.direction.z;
        let c = f32::powi(ray.origin.x, 2) + f32::powi(ray.origin.z, 2) - 1.;
        let disc = f32::powi(b, 2) - 4. * a * c;

        if disc < 0. {
            Vec::new()
        } else {
            let mut t0 = (-b - f32::sqrt(disc)) / (2. * a);
            let mut t1 = (-b + f32::sqrt(disc)) / (2. * a);
            
            if t0 > t1 {
                (t0, t1) = (t1, t0);
            }

            let mut xs = Vec::new();
            let y0 = ray.origin.y + t0 * ray.direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(Intersection::new(t0, ShapeEnum::Cylinder(*self)));
            }

            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection::new(t1, ShapeEnum::Cylinder(*self)));
            }
            xs
        }
    }

    fn get_transform(&self) -> Matrix<f32, 4, 4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
        self.transform = transform;
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        vector(point.x, 0., point.z)
    }
}
