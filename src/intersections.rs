use crate::{
    dot,
    ray::{position, Ray},
    shape::normal_at,
    world::ShapeEnum,
    Tuple,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection {
    pub t: f32,
    pub object: ShapeEnum,
}

impl Intersection {
    pub fn new(t: f32, object: ShapeEnum) -> Self {
        Intersection { t, object }
    }
}

pub fn hit(intersections: Vec<Intersection>) -> Option<Intersection> {
    let valid_intersections: Vec<Intersection> = intersections
        .into_iter()
        .filter(|x| x.t >= 0.)
        .collect::<Vec<Intersection>>();
    if valid_intersections.len() > 0 {
        let mut min_intersection = valid_intersections[0];
        for intersect in valid_intersections {
            if intersect.t < min_intersection.t {
                min_intersection = intersect;
            }
        }
        return Some(min_intersection);
    }
    None
}

pub struct Precomputation {
    pub t: f32,
    pub object: ShapeEnum,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

pub fn prepare_computations(intersection: Intersection, ray: Ray) -> Precomputation {
    let pos = position(ray, intersection.t);
    let mut normal = match intersection.object {
        ShapeEnum::Plane(plane) => normal_at(plane, pos),
        ShapeEnum::Sphere(sphere) => normal_at(sphere, pos),
    };
    let eye = -ray.direction;
    let mut inside = false;
    if dot(normal, eye) < 0. {
        normal = -normal;
        inside = true;
    }
    // Based on experiments, seems like this amount of perturbation is needed to avoid acne
    let over_point = pos + normal * 1e-4;

    Precomputation {
        t: intersection.t,
        object: intersection.object,
        point: pos,
        eyev: eye,
        normalv: normal,
        inside,
        over_point,
    }
}
