use crate::{
    dot,
    ray::{position, Ray},
    sphere::{normal_at, Sphere},
    Tuple,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection {
    pub t: f32,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: f32, object: Sphere) -> Self {
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
    pub object: Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

pub fn prepare_computations(intersection: Intersection, ray: Ray) -> Precomputation {
    let pos = position(ray, intersection.t);
    let mut normal = normal_at(intersection.object, pos);
    let eye = -ray.direction;
    let mut inside = false;
    if dot(normal, eye) < 0. {
        normal = -normal;
        inside = true;
    }
    let over_point = pos + normal * f32::EPSILON;

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
