extern crate ray_tracer_challenge;

use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::rays::{hit, Ray, Sphere};
use ray_tracer_challenge::tuples::Tuple;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let color = Tuple::color(1.0, 0.0, 0.0);
    let sphere = Sphere::new();
    let eye = Tuple::point(0.0, 0.0, -5.0);
    let canvas_dim = 100;
    let wall_size = 7.0;
    let wall_z = 10.0;
    let pixel_size = wall_size / canvas_dim as f32;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_dim, canvas_dim);

    for y in 0..canvas_dim {
        let world_y = half - pixel_size * y as f32;
        for x in 0..canvas_dim {
            let world_x = -half + pixel_size * x as f32;
            let position = Tuple::point(world_x, world_y, wall_z);
            let r = Ray::new(eye, (position - eye).normalize());
            let xs = r.intersect(sphere);
            if let Some(_) = hit(xs) {
                canvas.write_pixel(x, y, &color);
            }
        }
    }

    let ppm = canvas.to_ppm();
    let filename = "red_circle.ppm";
    let mut file = File::create(filename)?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
