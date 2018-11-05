extern crate ray_tracer_challenge;

use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::materials::Material;
use ray_tracer_challenge::rays::PointLight;
use ray_tracer_challenge::spheres::Sphere;
use ray_tracer_challenge::transforms::view_transform;
use ray_tracer_challenge::tuples::Tuple;
use ray_tracer_challenge::world::World;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let light = PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Tuple::color(1.0, 1.0, 1.0),
    );

    let mut world = World::new();
    world.light_source = Some(light);

    {
        let mut sphere = Sphere::new();
        let mut material = Material::default();
        material.color = Tuple::color(1.0, 0.2, 1.0);
        sphere.material = material;
        world.objects.push(sphere);
    }

    let mut camera = Camera::new(100, 50, PI / 3.0);
    camera.transform = view_transform(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    );

    let image = camera.render(world);

    let ppm = image.to_ppm();
    let filename = "red_circle.ppm";
    let mut file = File::create(filename)?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
