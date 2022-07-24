use crate::{
    intersections::{hit, prepare_computations, Intersection, Precomputation},
    lights::{lighting, PointLight},
    materials::Material,
    point,
    ray::Ray,
    sphere::{intersect, Sphere},
    transforms::scaling,
    Color,
};

#[derive(Clone)]
pub struct World {
    pub objects: Vec<Sphere>,
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
        let objects = vec![s1, s2];
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

pub fn contains(world: &World, object: Sphere) -> bool {
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
        let obj_intersects = intersect(*obj, ray);
        intersections.extend(obj_intersects);
    }
    intersections.sort_by(|&a, &b| (a.t).partial_cmp(&b.t).unwrap());
    intersections
}

pub fn shade_hit(world: &World, comps: Precomputation) -> Color {
    lighting(
        comps.object.material,
        world.light.unwrap(),
        comps.point,
        comps.eyev,
        comps.normalv,
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
