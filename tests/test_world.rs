mod tests {
    use tracer::{
        intersections::prepare_computations,
        intersections::Intersection,
        lights::PointLight,
        materials::Material,
        plane::Plane,
        point,
        ray::Ray,
        shape::Shape,
        sphere::Sphere,
        transforms::{scaling, translation},
        vector,
        world::{
            color_at, contains, intersect_world, is_shadowed, reflected_color, shade_hit,
            ShapeEnum, World, refracted_color,
        },
        Color, patterns::{Pattern, PatternType},
    };

    #[test]
    fn test_world() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert!(w.light.is_none());
    }

    #[test]
    fn test_default_world() {
        let light = PointLight::new(point(-10., 10., -10.), Color::new(1., 1., 1.));
        let mut s1 = Sphere::default();
        let mut m = Material::default();
        m.color = Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.material = m;

        let mut s2 = Sphere::default();
        s2.transform = scaling(0.5, 0.5, 0.5);

        let w = World::default();
        assert_eq!(w.light.unwrap(), light);
        assert!(contains(&w, ShapeEnum::Sphere(s1)));
        assert!(contains(&w, ShapeEnum::Sphere(s2)));
    }

    #[test]
    fn test_intersect_world_ray() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = intersect_world(&w, r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.);
    }

    #[test]
    fn test_shade_intersection() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let shape = w.objects[0];
        let i = Intersection::new(4., shape);
        let comps = prepare_computations(i, r, vec![i]);
        let c = shade_hit(&w, comps, 5);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855))
    }

    #[test]
    fn test_shade_intersection_inside() {
        let mut w = World::default();
        w.light = Some(PointLight::new(point(0., 0.25, 0.), Color::new(1., 1., 1.)));
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = prepare_computations(i, r, vec![i]);
        let c = shade_hit(&w, comps, 5);
        assert_eq!(c, Color::new(0.9049522, 0.9049522, 0.9049522));
    }

    #[test]
    fn test_intersect_world_ray_miss() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 1., 0.));
        let c = color_at(&w, r, 5);
        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_intersect_world_ray_hit() {
        let w = World::default();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let c = color_at(&w, r, 5);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_intersection_behind_ray() {
        let mut w = World::default();

        match w.objects[0] {
            ShapeEnum::Sphere(ref mut sphere) => sphere.material.ambient = 1.,
            _ => panic!("Shape is not a sphere"),
        }
        match w.objects[1] {
            ShapeEnum::Sphere(ref mut sphere) => sphere.material.ambient = 1.,
            _ => panic!("Shape is not a sphere"),
        }
        let r = Ray::new(point(0., 0., 0.75), vector(0., 0., -1.));
        let c = color_at(&w, r, 5);

        match w.objects[1] {
            ShapeEnum::Sphere(sphere) => {
                assert_eq!(c, sphere.material.color);
            }
            _ => panic!("Shape is not a sphere"),
        }
    }

    #[test]
    fn test_in_light_non_colinear() {
        let w = World::default();
        let p = point(0., 10., 0.);
        assert!(!is_shadowed(&w, p));
    }

    #[test]
    fn test_in_shadow_behind_object() {
        let w = World::default();
        let p = point(10., -10., 10.);
        assert!(is_shadowed(&w, p));
    }

    #[test]
    fn test_obj_behind_light() {
        let w = World::default();
        let p = point(-20., 20., -20.);
        assert!(!is_shadowed(&w, p));
    }

    #[test]
    fn test_point_between_light_and_obj() {
        let w = World::default();
        let p = point(-2., 2., -2.);
        assert!(!is_shadowed(&w, p));
    }

    #[test]
    fn test_shade_hit_sphere() {
        let mut w = World::new();
        w.light = Some(PointLight::new(point(0., 0., -10.), Color::new(1., 1., 1.)));
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.set_transform(translation(0., 0., 10.));
        w.objects = vec![ShapeEnum::Sphere(s1), ShapeEnum::Sphere(s2)];
        let r = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
        let i = Intersection::new(4., ShapeEnum::Sphere(s2));
        let comps = prepare_computations(i, r, vec![i]);
        let c = shade_hit(&w, comps, 5);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn test_hit_offset_point() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut shape = Sphere::default();
        shape.set_transform(translation(0., 0., 1.));
        let i = Intersection::new(5., ShapeEnum::Sphere(shape));
        let comps = prepare_computations(i, r, vec![i]);
        assert!(comps.over_point.z < -f32::EPSILON / 2.);
        assert!(comps.point.z > comps.over_point.z)
    }

    #[test]
    fn test_non_reflective_surface() {
        let w = World::default();
        let r = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let mut shape = match w.objects[1] {
            ShapeEnum::Sphere(sphere) => sphere,
            _ => panic!("Not a sphere"),
        };
        shape.material.ambient = 1.;
        let i = Intersection::new(1., w.objects[1]);
        let comps = prepare_computations(i, r, vec![i]);
        let color = reflected_color(&w, comps, 5);
        assert_eq!(color, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_reflective_surface() {
        let mut w = World::default();
        let mut material = Material::default();
        material.reflective = 0.5;
        let shape = ShapeEnum::Plane(Plane {
            transform: translation(0., -1., 0.),
            material,
        });
        w.objects.push(shape);

        let r = Ray::new(
            point(0., 0., -3.),
            vector(0., -f32::sqrt(2.) / 2., f32::sqrt(2.) / 2.),
        );
        let i = Intersection::new(f32::sqrt(2.), shape);
        let comps = prepare_computations(i, r, vec![i]);
        let color = reflected_color(&w, comps, 5);
        assert_eq!(color, Color::new(0.19034664, 0.23793328, 0.14275998));
    }

    #[test]
    fn test_shade_hit_reflective() {
        let mut w = World::default();
        let mut material = Material::default();
        material.reflective = 0.5;
        let shape = ShapeEnum::Plane(Plane {
            transform: translation(0., -1., 0.),
            material,
        });
        w.objects.push(shape);

        let r = Ray::new(
            point(0., 0., -3.),
            vector(0., -f32::sqrt(2.) / 2., f32::sqrt(2.) / 2.),
        );
        let i = Intersection::new(f32::sqrt(2.), shape);
        let comps = prepare_computations(i, r, vec![i]);
        let color = shade_hit(&w, comps, 5);
        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn test_color_reflective_surfaces() {
        let mut w = World::default();
        w.light = Some(PointLight::new(point(0., 0., 0.), Color::new(1., 1., 1.)));
        let mut lower = Plane::default();
        let mut material = Material::default();
        material.reflective = 1.;
        let transform = translation(0., -1., 0.);
        lower.set_transform(transform);
        lower.material = material;
        let lower = ShapeEnum::Plane(lower);
        w.objects.push(lower);

        let mut upper = Plane::default();
        let mut material = Material::default();
        material.reflective = 1.;
        let transform = translation(0., 1., 0.);
        upper.set_transform(transform);
        upper.material = material;
        let upper = ShapeEnum::Plane(upper);
        w.objects.push(upper);

        let r = Ray::new(point(0., 0., 0.), vector(0., 1., 0.));
        color_at(&w, r, 5);
    }

    #[test]
    fn test_reflected_color_max_depth() {
        let mut w = World::default();
        let mut shape = Plane::default();
        let mut material = Material::default();
        material.reflective = 0.5;
        let transform = translation(0., -1., 0.);
        shape.set_transform(transform);
        shape.material = material;
        let shape = ShapeEnum::Plane(shape);
        w.objects.push(shape);

        let r = Ray::new(point(0., 0., -3.), vector(0., -f32::sqrt(2.) / 2., f32::sqrt(2.) / 2.));
        let i = Intersection::new(f32::sqrt(2.), shape);
        let comps = prepare_computations(i, r, vec![i]);
        let color = reflected_color(&w, comps, 0);
        assert_eq!(color, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_refraction_opaque() {
        let w = World::default();
        let shape = w.objects.first().unwrap();
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = vec![
            Intersection::new(4., *shape),
            Intersection::new(6., *shape),
        ];
        let comps = prepare_computations(xs[0], r, xs);
        let c = refracted_color(&w, comps, 5);
        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_refraction_max_depth() {
        let w = World::default();
        let shape = w.objects[0];
        match shape {
            ShapeEnum::Sphere(mut sphere) => {
                sphere.material.transparency = 1.;
                sphere.material.refractive_index = 1.5;
            },
            _ => panic!("Not a sphere"),
        }

        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let xs = vec![
            Intersection::new(4., shape),
            Intersection::new(6., shape),
        ];
        let comps = prepare_computations(xs[0], r, xs);
        let c = refracted_color(&w, comps, 0);
        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_total_internal_reflection() {
        let mut w = World::default();
        if let ShapeEnum::Sphere(ref mut sphere) = w.objects[0] {
            sphere.material.transparency = 1.;
            sphere.material.refractive_index = 1.5;
        }
        let r = Ray::new(point(0., 0., f32::sqrt(2.) / 2.), vector(0., 1., 0.));
        let xs = vec![
            Intersection::new(-f32::sqrt(2.) / 2., w.objects[0]),
            Intersection::new(f32::sqrt(2.) / 2., w.objects[0]),
        ];
        
        let comps = prepare_computations(xs[1], r, xs);
        let c = refracted_color(&w, comps, 5);
        assert_eq!(c, Color::new(0., 0., 0.));
    }

    #[test]
    fn test_refracted_color_ray() {
        let mut w = World::default();

        if let ShapeEnum::Sphere(ref mut sphere) = w.objects[0] {
            sphere.material.ambient = 1.;
            sphere.material.pattern = Some(Pattern::new(PatternType::Test()));
        }

        if let ShapeEnum::Sphere(ref mut sphere) = w.objects[1] {
            sphere.material.transparency = 1.;
            sphere.material.refractive_index = 1.5;
        }

        let r = Ray::new(point(0., 0., 0.1), vector(0., 1., 0.));
        let xs = vec![
            Intersection::new(-0.9899, w.objects[0]),
            Intersection::new(-0.4899, w.objects[1]),
            Intersection::new(0.4899, w.objects[1]),
            Intersection::new(0.9899, w.objects[0]),
        ];

        let comps = prepare_computations(xs[2], r, xs);
        let c = refracted_color(&w, comps, 5);
        assert_eq!(c, Color::new(0., 0.99878335, 0.04724201));
    }

    #[test]
    fn test_shade_hit_transparent() {
        let mut w = World::default();
        let mut floor = Plane::default();
        floor.set_transform(translation(0., -1., 0.));
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(ShapeEnum::Plane(floor));

        let mut ball = Sphere::default();
        ball.material.color = Color::new(1., 0., 0.);
        ball.material.ambient = 0.5;
        ball.set_transform(translation(0., -3.5, -0.5));

        w.objects.push(ShapeEnum::Sphere(ball));

        let r = Ray::new(point(0., 0., -3.), vector(0., -f32::sqrt(2.) / 2., f32::sqrt(2.) / 2.));
        let xs = vec![
            Intersection::new(f32::sqrt(2.), ShapeEnum::Plane(floor)),
        ];
        let comps = prepare_computations(xs[0], r, xs);
        let color = shade_hit(&w, comps, 5);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }

    #[test]
    fn test_shade_hit_reflective_transparent() {
        let mut w = World::default();
        let r = Ray::new(point(0., 0., -3.), vector(0., -f32::sqrt(2.) / 2., f32::sqrt(2.) / 2.));
        let mut floor = Plane::default();
        floor.set_transform(translation(0., -1., 0.));
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(ShapeEnum::Plane(floor));

        let mut ball = Sphere::default();
        ball.material.color = Color::new(1., 0., 0.);
        ball.material.ambient = 0.5;
        ball.set_transform(translation(0., -3.5, -0.5));
        w.objects.push(ShapeEnum::Sphere(ball));

        let xs = vec![
            Intersection::new(f32::sqrt(2.), ShapeEnum::Plane(floor)),
        ];
        let comps = prepare_computations(xs[0], r, xs);
        let color = shade_hit(&w, comps, 5);
        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
