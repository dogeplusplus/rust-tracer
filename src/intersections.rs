use crate::{
    dot,
    ray::{position, Ray},
    reflect,
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

#[derive(Clone, Copy)]
pub struct Precomputation {
    pub t: f32,
    pub object: ShapeEnum,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub reflectv: Tuple,
    pub n1: f32,
    pub n2: f32,
}

pub fn prepare_computations(
    intersection: Intersection,
    ray: Ray,
    intersections: Vec<Intersection>,
) -> Precomputation {
    let pos = position(ray, intersection.t);
    let mut normal = match intersection.object {
        ShapeEnum::Plane(plane) => normal_at(plane, pos),
        ShapeEnum::Sphere(sphere) => normal_at(sphere, pos),
        ShapeEnum::Cube(cube) => normal_at(cube, pos),
        ShapeEnum::Cylinder(cylinder) => normal_at(cylinder, pos),
        ShapeEnum::Cone(cone) => normal_at(cone, pos),
    };
    let eye = -ray.direction;
    let mut inside = false;
    if dot(normal, eye) < 0. {
        normal = -normal;
        inside = true;
    }
    // Based on experiments, seems like this amount of perturbation is needed to avoid acne
    let over_point = pos + normal * 1e-4;
    let under_point = pos - normal * 1e-4;

    let reflection = reflect(ray.direction, normal);

    // Calculate refractive indices
    let mut n1 = 1.;
    let mut n2 = 1.;
    let mut containers = Vec::new();

    for inter in intersections {
        if inter == intersection {
            if containers.len() == 0 {
                n1 = 1.0;
            } else {
                n1 = match containers.last().unwrap() {
                    ShapeEnum::Sphere(sphere) => sphere.material.refractive_index,
                    ShapeEnum::Plane(plane) => plane.material.refractive_index,
                    ShapeEnum::Cube(cube) => cube.material.refractive_index,
                    ShapeEnum::Cylinder(cylinder) => cylinder.material.refractive_index,
                    ShapeEnum::Cone(cone) => cone.material.refractive_index,
                };
            }
        }

        if containers.contains(&inter.object) {
            containers.retain(|&x| x != inter.object);
        } else {
            containers.push(inter.object);
        }

        if inter == intersection {
            if containers.len() == 0 {
                n2 = 1.;
            } else {
                n2 = match containers.last().unwrap() {
                    ShapeEnum::Sphere(sphere) => sphere.material.refractive_index,
                    ShapeEnum::Plane(plane) => plane.material.refractive_index,
                    ShapeEnum::Cube(cube) => cube.material.refractive_index,
                    ShapeEnum::Cylinder(cylinder) => cylinder.material.refractive_index,
                    ShapeEnum::Cone(cone) => cone.material.refractive_index,
                };
            }
        }
    }

    Precomputation {
        t: intersection.t,
        object: intersection.object,
        point: pos,
        eyev: eye,
        normalv: normal,
        inside,
        over_point,
        under_point,
        reflectv: reflection,
        n1,
        n2,
    }
}

pub fn shlick(comps: Precomputation) -> f32 {
    let mut cos = dot(comps.eyev, comps.normalv);

    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = f32::powi(n, 2) * (1. - f32::powi(cos, 2));
        if sin2_t > 1. {
            return 1.;
        }
        let cos_t = f32::sqrt(1. - sin2_t);
        cos = cos_t;
    }

    let r0 = f32::powi((comps.n1 - comps.n2) / (comps.n1 + comps.n2), 2);
    r0 + (1. - r0) * f32::powi(1. - cos, 5)
}
