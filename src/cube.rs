use crate::{
    intersections::Intersection, matrix::Matrix, normalize, ray::Ray, shape::Shape,
    Tuple,
    world::ShapeEnum, materials::Material,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cube {
    pub transform: Matrix<f32, 4, 4>,
    pub material: Material,
}

impl Default for Cube {
    fn default() -> Self {
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        Cube {
            transform: identity,
            material: Material::default(),
        }
    }
}

fn check_axis(origin: f32, direction: f32) -> [f32; 2] {
    let tmin_numerator = -1. - origin;
    let tmax_numerator = 1. - origin;

    let mut tmin = tmin_numerator * f32::INFINITY;
    let mut tmax = tmax_numerator * f32::INFINITY;

    if direction.abs() >= f32::EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    }

    if tmin > tmax {
        (tmax, tmin) = (tmin, tmax);
    }

    [tmin, tmax]
}

impl Shape for Cube {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let [xtmin, xtmax] = check_axis(ray.origin.x, ray.direction.x);
        let [ytmin, ytmax] = check_axis(ray.origin.y, ray.direction.y);
        let [ztmin, ztmax] = check_axis(ray.origin.z, ray.direction.z);


        let tmins = vec![xtmin, ytmin, ztmin];
        let tmaxs = vec![xtmax, ytmax, ztmax];

        let tmin = tmins.iter().fold(-f32::INFINITY, |a, &b| a.max(b));
        let tmax = tmaxs.iter().fold(f32::INFINITY, |a, &b| a.min(b));

        if tmin > tmax {
            Vec::new()
        } else {
            vec![
                Intersection::new(tmin, ShapeEnum::Cube(*self)),
                Intersection::new(tmax, ShapeEnum::Cube(*self)),
            ]
        }
    }

    fn set_transform(&mut self, transform: Matrix<f32, 4, 4>) {
        self.transform = transform;
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
