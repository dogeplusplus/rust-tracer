use std::f32::consts::PI;
use std::fs::File;
use std::io::Write;
use tracer::camera::render;
use tracer::camera::Camera;
use tracer::canvas::canvas_to_ppm;
use tracer::lights::PointLight;
use tracer::materials::Material;
use tracer::patterns::GradientPattern;
use tracer::patterns::{CheckerPattern, Pattern, PatternType, RingPattern, StripePattern};
use tracer::plane::Plane;
use tracer::sphere::Sphere;
use tracer::transforms::{rotation_x, scaling, shearing, translation, view_transform};
use tracer::world::{ShapeEnum, World};
use tracer::{point, vector, Color};

fn main() -> Result<(), &'static str> {
    let mut middle = Sphere::default();
    middle.transform = translation(-0.5, 1., 0.5);
    middle.material = Material::default();
    middle.material.color = Color::new(0.1, 1., 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let stripe = PatternType::Stripe(StripePattern::new(
        Color::new(1.0, 0.5, 0.0),
        Color::new(0., 0.5, 0.5),
    ));
    let mut stripe_pattern = Pattern::new(stripe);
    stripe_pattern.set_transform(scaling(0.3, 0.3, 0.3));
    middle.material.pattern = Some(Pattern::new(stripe));

    let mut right = Sphere::default();
    right.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    right.material = Material::default();
    right.material.color = Color::new(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let ring = PatternType::Ring(RingPattern::new(
        Color::new(0.0, 0.9, 0.9),
        Color::new(0.5, 0.5, 0.),
    ));
    let mut ring_pattern = Pattern::new(ring);
    ring_pattern.set_transform(rotation_x(PI / 2.) * scaling(0.2, 0.2, 0.2));
    right.material.pattern = Some(ring_pattern);

    let mut left = Sphere::default();
    left.transform = translation(-1.5, 0.33, -0.75)
        * scaling(0.5, 0.5, 0.5)
        * shearing(0.5, 0.5, 0.5, 0.5, 0.5, 0.5);
    left.material = Material::default();
    left.material.color = Color::new(0., 0.5, 0.9);
    left.material.diffuse = 0.7;
    left.material.specular = 1.;

    let gradient = PatternType::Gradient(GradientPattern::new(
        Color::new(1.0, 0.2, 0.),
        Color::new(0., 0.2, 1.0),
    ));
    let mut gradient_pattern = Pattern::new(gradient);
    gradient_pattern.set_transform(
        shearing(0.5, 0.2, 1., 0., 0.2, 0.2)
    );
    left.material.pattern = Some(gradient_pattern);

    let mut floor = Plane::default();
    floor.transform = translation(0., 0., -0.75);

    let checker = PatternType::Checker(CheckerPattern::new(
        Color::new(1., 1., 1.),
        Color::new(0.0, 0.0, 0.0),
    ));
    floor.material.pattern = Some(Pattern::new(checker));

    let mut world = World::default();
    world.objects = vec![
        ShapeEnum::Sphere(middle),
        ShapeEnum::Sphere(right),
        ShapeEnum::Sphere(left),
        ShapeEnum::Plane(floor),
    ];
    world.light = Some(PointLight::new(
        point(-10., 10., -10.),
        Color::new(1., 1., 1.),
    ));
    let mut camera = Camera::new(500, 250, PI / 3.);
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
