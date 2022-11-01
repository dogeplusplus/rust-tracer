use crate::{
    intersections::Intersection,
    materials::Material,
    matrix::Matrix,
    ray::Ray,
    shape::{intersect, Shape},
    world::ShapeEnum,
    Tuple,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Group<'a> {
    pub shapes: Vec<&'a ShapeEnum<'a>>,
    pub transform: Matrix<f32, 4, 4>,
    pub material: Material,
}

impl Default for Group<'_> {
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

impl<'a> Group<'a> {
    pub fn add_child(&mut self, shape: &'a ShapeEnum<'a>) {
        match shape {
            ShapeEnum::Sphere(mut sphere) => sphere.set_parent(self),
            ShapeEnum::Plane(mut plane) => plane.set_parent(self),
            ShapeEnum::Cube(mut cube) => cube.set_parent(self),
            ShapeEnum::Cylinder(mut cylinder) => cylinder.set_parent(self),
            ShapeEnum::Cone(mut cone) => cone.set_parent(self),
            ShapeEnum::Test(mut test) => test.set_parent(self),
            _ => todo!("Figure out how to deal with groups"),
        }
        self.shapes.push(shape);
    }
}

impl<'a> Shape<'a> for Group<'a> {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut intersections = Vec::new();
        for shape in &self.shapes {
            let xs = match shape {
                ShapeEnum::Sphere(sphere) => intersect(sphere, ray),
                ShapeEnum::Plane(plane) => intersect(plane, ray),
                ShapeEnum::Cube(cube) => intersect(cube, ray),
                ShapeEnum::Cylinder(cylinder) => intersect(cylinder, ray),
                ShapeEnum::Cone(cone) => intersect(cone, ray),
                ShapeEnum::Test(test) => intersect(test, ray),
                ShapeEnum::Group(group) => intersect(&**group, ray),
            };
            intersections.extend(xs);
        }
        intersections.sort_by(|a, b| (a.t).partial_cmp(&b.t).unwrap());
        intersections
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

    fn set_parent(&mut self, _parent: &'a Group) {
        todo!("blank")
    }
}
