use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use tracer::camera::render;
use tracer::canvas::{canvas_to_ppm};
use tracer::materials::Material;
use tracer::sphere::{Sphere};
use tracer::lights::PointLight;
use tracer::transforms::{rotation_x, rotation_y, scaling, translation, view_transform, shearing};
use tracer::world::World;
use tracer::{point, Color, vector};
use tracer::camera::Camera;

fn main() -> Result<(), &'static str> {
    // Flatenned sphere
    let mut floor = Sphere::default();
    floor.transform = scaling(10., 0.01, 10.);
    floor.material = Material::default();
    floor.material.color = Color::new(1., 0.9, 0.9);
    floor.material.specular = 0.;

    // Left wall sphere
    let mut left_wall = Sphere::default();
    left_wall.transform = translation(0., 0., 5.)
        * rotation_y(-PI / 4.)
        * rotation_x(PI / 2.)
        * scaling(10., 0.01, 10.);
    left_wall.material = floor.material;

    // Right wall sphere
    let mut right_wall = Sphere::default();
    right_wall.transform = translation(0., 0., 5.)
        * rotation_y(PI / 4.)
        * rotation_x(PI / 2.)
        * scaling(10., 0.01, 10.);
    right_wall.material = floor.material;

    let mut middle = Sphere::default();
    middle.transform = translation(-0.5, 1., 0.5);
    middle.material = Material::default();
    middle.material.color = Color::new(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = Sphere::default();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material = Material::default();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut left = Sphere::default();
    left.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33) * shearing(0.5, 0.5, 0.5, 0.5, 0.5, 0.5);
    left.material = Material::default();
    left.material.color = Color::new(0., 0.5, 0.9);
    left.material.diffuse = 0.7;
    left.material.specular = 1.;

    let mut world = World::default();
    world.objects = vec![floor, left_wall, right_wall, middle, right, left];
    world.light = Some(PointLight::new(point(-10., 10., -10.), Color::new(1., 1., 1.)));
    let mut camera = Camera::new(250, 125, PI / 3.);
    camera.transform = view_transform(point(0., 1.5, -5.), point(0., 1., 0.), vector(0., 1., 0.));
    let canvas = render(camera, world)?;
    let ppm = canvas_to_ppm(&canvas);
    let mut f = File::create("output.svg").expect("Could not create file");
    for row in ppm {
        f.write_all(row.as_bytes()).expect("Could not write row.");
        f.write("\n".as_bytes()).expect("Could not write new line");
    }
    Ok(())
}