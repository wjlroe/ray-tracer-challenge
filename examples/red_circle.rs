extern crate ray_tracer_challenge;

use ray_tracer_challenge::camera::Camera;
use ray_tracer_challenge::lighting::PointLight;
use ray_tracer_challenge::materials::Material;
use ray_tracer_challenge::matrices::Matrix4;
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

    let mut side_color = Material::default();
    side_color.color = Tuple::color(1.0, 0.9, 0.9);
    side_color.specular = 0.0;

    {
        let mut floor = Sphere::new();
        floor.transform = Matrix4::scaling(10.0, 0.01, 10.0);
        floor.material = side_color;
        world.objects.push(floor);
    }

    {
        let mut left_wall = Sphere::new();
        left_wall.transform = Matrix4::translation(0.0, 0.0, 5.0)
            * Matrix4::rotation_y(-PI / 4.0)
            * Matrix4::rotation_x(PI / 2.0)
            * Matrix4::scaling(10.0, 0.01, 10.0);
        left_wall.material = side_color;
        world.objects.push(left_wall);
    }

    {
        let mut right_wall = Sphere::new();
        right_wall.transform = Matrix4::translation(0.0, 0.0, 5.0)
            * Matrix4::rotation_y(PI / 4.0)
            * Matrix4::rotation_x(PI / 2.0)
            * Matrix4::scaling(10.0, 0.01, 10.0);
        right_wall.material = side_color;
        world.objects.push(right_wall);
    }

    {
        let mut middle = Sphere::new();
        middle.transform = Matrix4::translation(-0.5, 1.0, 0.5);
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

    let mut camera = Camera::new(1000, 500, PI / 3.0);
    camera.transform = view_transform(
        Tuple::point(0.0, 1.5, -5.0),
        Tuple::point(0.0, 1.0, 0.0),
        Tuple::vector(0.0, 1.0, 0.0),
    );

    println!("Rendering world with {} pixels", camera.num_pixels());
    let mut image = camera.render(world);
    let draw_debug = false;
    if draw_debug {
        // 400,10
        let debug_point = (40, 2);
        let red = Tuple::color(1.0, 0.0, 0.0);
        for pre_x in 1..4 {
            for pre_y in 1..4 {
                image.write_pixel(
                    debug_point.0 - pre_x,
                    debug_point.1 - pre_y,
                    &red,
                );
                image.write_pixel(
                    debug_point.0 - pre_x,
                    debug_point.1 + pre_y,
                    &red,
                );
                image.write_pixel(
                    debug_point.0 + pre_x,
                    debug_point.1 - pre_y,
                    &red,
                );
                image.write_pixel(
                    debug_point.0 + pre_x,
                    debug_point.1 + pre_y,
                    &red,
                );
            }
        }
    }

    let ppm = image.to_ppm();
    let filename = "red_circle.ppm";
    let mut file = File::create(filename)?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
