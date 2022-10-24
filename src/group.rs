use crate::{matrix::Matrix, world::ShapeEnum, materials::Material, shape::Shape, Tuple, ray::Ray, intersections::Intersection};

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub shapes: Vec<ShapeEnum>,
    pub transform: Matrix<f32, 4, 4>,
    pub material: Material,
}

impl Default for Group {
    fn default() -> Self {
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        Group {
            shapes: Vec::new(),
            transform: identity,
            material: Material::default(),
        }
    }
}

impl Shape for Group {
    fn local_intersect(&self, _ray: Ray) -> Vec<Intersection> {
        Vec::new()
    }

    fn get_transform(&self) -> Matrix<f32, 4, 4> {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
        self.transform = transform;
    }
    fn local_normal_at(&self, _point: Tuple) -> Tuple {
        panic!("No local normal at for group.")
    }
}
