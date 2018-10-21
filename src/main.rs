extern crate ray_tracer_challenge;

use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::tuples::Tuple;
use std::fs::File;
use std::io::prelude::*;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct World {
    gravity: Tuple,
    wind: Tuple,
}

impl World {
    fn tick(&self, p: Projectile) -> Projectile {
        let position = p.position + p.velocity;
        let velocity = p.velocity + self.gravity + self.wind;
        Projectile { position, velocity }
    }
}

fn main() -> std::io::Result<()> {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
    };
    let w = World {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let c_width = 900;
    let c_height = 550;
    let mut canvas = Canvas::new(c_width, c_height);
    let red = Tuple::color(1.0, 0.0, 0.0);
    let mut num_ticks = 0;
    while p.position.y > 0.0 {
        num_ticks += 1;
        println!(
            "Tick {:4}. Projectile is at ({}, {}, {:.3})...",
            num_ticks,
            p.position.x.round() as u32,
            c_height - p.position.y.round() as u32,
            p.position.z
        );
        canvas.write_pixel(
            p.position.x.round() as u32,
            c_height - p.position.y.round() as u32,
            &red,
        );
        let new_p = w.tick(p);
        p = new_p;
    }
    let ppm = canvas.to_ppm();
    let filename = "output.ppm";
    let mut file = File::create(filename)?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
