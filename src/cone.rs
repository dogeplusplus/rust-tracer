use crate::{
    intersections::Intersection, materials::Material, matrix::Matrix, ray::Ray, shape::Shape,
    vector, world::ShapeEnum, Tuple,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cone {
    transform: Matrix<f32, 4, 4>,
    pub material: Material,
    pub minimum: f32,
    pub maximum: f32,
    pub closed: bool,
}

impl Default for Cone {
    fn default() -> Self {
        let identity = Matrix::new([
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ]);
        Cone {
            transform: identity,
            material: Material::default(),
            minimum: -f32::INFINITY,
            maximum: f32::INFINITY,
            closed: false,
        }
    }
}

impl Shape for Cone {
    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let a = f32::powi(ray.direction.x, 2) - f32::powi(ray.direction.y, 2) + f32::powi(ray.direction.z, 2);
        let b = 2. * (
            ray.origin.x * ray.direction.x - ray.origin.y * ray.direction.y + ray.origin.z * ray.direction.z
        );
        let c = f32::powi(ray.origin.x, 2) - f32::powi(ray.origin.y, 2) + f32::powi(ray.origin.z, 2);
        let disc = f32::powi(b, 2) - 4. * a * c;
    
        if a == 0. {
            let mut xs = Vec::new();

            if b != 0. {
                let t = -c / (2. * b);
                xs.push(Intersection::new(t, ShapeEnum::Cone(*self)));
            }
            intersect_caps(*self, ray, &mut xs);
            return xs;
        }

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
                xs.push(Intersection::new(t0, ShapeEnum::Cone(*self)));
            }

            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection::new(t1, ShapeEnum::Cone(*self)));
            }

            intersect_caps(*self, ray, &mut xs);
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
        let mut y = f32::sqrt(f32::powi(point.x, 2) + f32::powi(point.z, 2));
        if point.y > 0. {
            y = -y;
        }
        vector(point.x, y, point.z)
    }   
}

fn check_cap(ray: Ray, t: f32, y: f32) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    f32::powi(x, 2) + f32::powi(z, 2) <= y
}

fn intersect_caps(cone: Cone, ray: Ray, xs: &mut Vec<Intersection>) {
    if !cone.closed || ray.direction.y == 0. {
        return;
    }

    let t = (cone.minimum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, cone.minimum) {
        xs.push(Intersection::new(t, ShapeEnum::Cone(cone)));
    }

    let t = (cone.maximum - ray.origin.y) / ray.direction.y;
    if check_cap(ray, t, cone.maximum) {
        xs.push(Intersection::new(t, ShapeEnum::Cone(cone)));
    }
}