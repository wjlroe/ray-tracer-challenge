extern crate ray_tracer_challenge;

use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::lighting::PointLight;
use ray_tracer_challenge::materials::Material;
use ray_tracer_challenge::matrices::Matrix4;
use ray_tracer_challenge::shapes::{Plane, Sphere};
use ray_tracer_challenge::transforms::view_transform;
use ray_tracer_challenge::tuples::Tuple;
use ray_tracer_challenge::world::World;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let light = PointLight::new(
        Tuple::point(-1.0, 2.5, -1.0),
        Tuple::color(1.0, 1.0, 1.0),
    );

    let mut world = World::new();
    world.light_source = Some(light);

    let mut side_color = Material::default();
    side_color.color = Tuple::color(1.0, 0.9, 0.9);
    side_color.specular = 0.0;

    {
        let mut floor = Plane::new();
        floor.material = side_color;
        world.objects.push(floor);
    }

    // hexagon angles
    // 120 degrees around y-axis
    // 2 PI / 3.0 radians
    // if 90 degrees is PI/2.0
    for hex in 0..6 {
        let mut wall = Plane::new();
        wall.transform = Matrix4::translation(0.0, 0.0, 4.0)
            * Matrix4::rotation_y(hex as f32 * (PI / 3.0))
            * Matrix4::rotation_x(PI / 2.0);
        wall.material = side_color;
        world.objects.push(wall);
    }

    {
        let mut middle = Sphere::new();
        middle.transform = Matrix4::translation(-0.5, -0.1, 0.5);
        middle.material = Material::default();
        middle.material.color = Tuple::color(0.1, 1.0, 0.5);
        middle.material.diffuse = 0.7;
        middle.material.specular = 0.3;
        world.objects.push(middle);
    }

    {
        let mut right = Sphere::new();
        right.transform = Matrix4::translation(1.5, 0.5, -0.5)
            * Matrix4::scaling(0.5, 0.5, 0.5);
        right.material = Material::default();
        right.material.color = Tuple::color(0.5, 1.0, 0.1);
        right.material.diffuse = 0.7;
        right.material.specular = 0.3;
        world.objects.push(right);
    }

    {
        let mut left = Sphere::new();
        left.transform = Matrix4::translation(-1.5, 0.33, -0.75)
            * Matrix4::scaling(0.33, 0.33, 0.33);
        left.material = Material::default();
        left.material.color = Tuple::color(1.0, 0.8, 0.1);
        left.material.diffuse = 0.7;
        left.material.specular = 0.3;
        world.objects.push(left);
    }

    let aspect = 2560.0 / 1440.0;
    let vsize = 100;
    let hsize = (aspect * vsize as f32).round() as u32;
    let mut camera = Camera::new(hsize, vsize, PI / 3.0);
    camera.transform = view_transform(
        Tuple::point(0.0, 10.5, 0.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 0.0, 1.0),
    );

    println!("Rendering world with {} pixels", camera.num_pixels());
    let image = camera.render(world);

    let ppm = image.to_ppm();
    let filename = "plane_example.ppm";
    let mut file = File::create(filename)?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
