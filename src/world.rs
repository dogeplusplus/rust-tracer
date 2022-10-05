use crate::{
    intersections::{hit, prepare_computations, Intersection, Precomputation},
    lights::{lighting, PointLight},
    materials::Material,
    point,
    ray::Ray,
    shape::intersect,
    sphere::Sphere,
    transforms::scaling,
    Color, Tuple, magnitude, normalize, plane::Plane,
};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShapeEnum {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Clone)]
pub struct World {
    pub objects: Vec<ShapeEnum>,
    pub light: Option<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        let light = PointLight::new(point(-10., 10., -10.), Color::new(1., 1., 1.));
        let mut s1 = Sphere::default();
        let mut m = Material::default();
        m.color = Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.material = m;

        let mut s2 = Sphere::default();
        s2.transform = scaling(0.5, 0.5, 0.5);
        let objects = vec![ShapeEnum::Sphere(s1), ShapeEnum::Sphere(s2)];
        World {
            objects,
            light: Some(light),
        }
    }
}

impl World {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
            light: None,
        }
    }
}

pub fn contains(world: &World, object: ShapeEnum) -> bool {
    for obj in &world.objects {
        if *obj == object {
            return true;
        }
    }
    false
}

pub fn intersect_world(world: &World, ray: Ray) -> Vec<Intersection> {
    let mut intersections = Vec::new();
    for obj in &world.objects {
        let obj_intersects = match *obj {
            ShapeEnum::Plane(plane) => intersect(plane, ray),
            ShapeEnum::Sphere(sphere) => intersect(sphere, ray),
        };
        intersections.extend(obj_intersects);
    }
    intersections.sort_by(|&a, &b| (a.t).partial_cmp(&b.t).unwrap());
    intersections
}

pub fn shade_hit(world: &World, comps: Precomputation) -> Color {
    let material = match comps.object {
        ShapeEnum::Sphere(sphere) => sphere.material,
        ShapeEnum::Plane(plane) => plane.material,
    };

    lighting(
        material,
        comps.object,
        world.light.unwrap(),
        comps.point,
        comps.eyev,
        comps.normalv,
        is_shadowed(world, comps.over_point),
    )
}

pub fn color_at(world: &World, ray: Ray) -> Color {
    let intersections = intersect_world(world, ray);
    let hits = hit(intersections);

    if hits.is_none() {
        return Color::new(0., 0., 0.);
    }

    let comps = prepare_computations(hits.unwrap(), ray);
    shade_hit(world, comps)
}


pub fn is_shadowed(world: &World, point: Tuple) -> bool {
    if let Some(light) = world.light {
        let v = light.position - point;
        let distance = magnitude(v);
        let direction = normalize(v);

        let r = Ray::new(point, direction);
        let intersections = intersect_world(world, r);

        let h = hit(intersections);
        if let Some(intersect) = h {
            return intersect.t < distance;
        } else {
            return false;
        }
    } else {
        return false;
    }
}