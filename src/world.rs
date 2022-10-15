use crate::{
    intersections::{hit, prepare_computations, Intersection, Precomputation, shlick},
    lights::{lighting, PointLight},
    magnitude,
    materials::Material,
    normalize,
    plane::Plane,
    point,
    ray::Ray,
    shape::intersect,
    sphere::Sphere,
    transforms::scaling,
    Color, Tuple, dot,
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

pub fn shade_hit(world: &World, comps: Precomputation, remaining: u16) -> Color {
    let material = match comps.object {
        ShapeEnum::Sphere(sphere) => sphere.material,
        ShapeEnum::Plane(plane) => plane.material,
    };

    let reflected = reflected_color(world, comps, remaining);
    let refracted = refracted_color(world, comps, remaining);
    let light = lighting(
        material,
        comps.object,
        world.light.unwrap(),
        comps.over_point,
        comps.eyev,
        comps.normalv,
        is_shadowed(world, comps.over_point),
    );

    if material.reflective > 0. && material.transparency > 0. {
        let reflectance = shlick(comps);
        light + reflected * reflectance + refracted * (1. - reflectance)
    } else {
        light + reflected + refracted
    }
}

pub fn color_at(world: &World, ray: Ray, remaining: u16) -> Color {
    let intersections = intersect_world(world, ray);
    let hits = hit(intersections.clone());

    if hits.is_none() {
        return Color::new(0., 0., 0.);
    }

    let comps = prepare_computations(hits.unwrap(), ray, intersections);
    shade_hit(world, comps, remaining)
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

pub fn reflected_color(w: &World, comps: Precomputation, remaining: u16) -> Color {
    if remaining < 1 {
        return Color::new(0., 0., 0.);
    }

    let material = match comps.object {
        ShapeEnum::Sphere(sphere) => sphere.material,
        ShapeEnum::Plane(plane) => plane.material,
    };

    if material.reflective == 0. {
        Color::new(0., 0., 0.);
    }

    let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
    let color = color_at(&w, reflect_ray, remaining - 1);

    color * material.reflective
}

pub fn refracted_color(w: &World, comps: Precomputation, remaining: u16) -> Color {
    let black = Color::new(0., 0., 0.);

    if remaining < 1 {
        return black;
    }

    // Find ratio of first refractive to second
    // cos theta is the same as the dot product of two vectors 
    let n_ratio = comps.n1 / comps.n2;
    let cos_i = dot(comps.eyev, comps.normalv);
    let sin2_t = f32::powi(n_ratio, 2) * (1. - f32::powi(cos_i, 2));
    let cos_t = f32::sqrt(1. - sin2_t);

    // Total internal reflection
    if sin2_t > 1. {
        return black;
    }

    // Direction of refracted ray
    let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
    let refract_ray = Ray::new(comps.under_point, direction);


    let material = match comps.object {
        ShapeEnum::Sphere(sphere) => sphere.material,
        ShapeEnum::Plane(plane) => plane.material,
    };

    let color = color_at(w, refract_ray, remaining - 1) * material.transparency;
    color
}