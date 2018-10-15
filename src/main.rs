extern crate ray_tracer_challenge;

use ray_tracer_challenge::tuples::Tuple;

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

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 5.0, 0.0).normalize(),
    };
    let w = World {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut num_ticks = 0;
    while p.position.y > 0.0 {
        num_ticks += 1;
        println!(
            "Tick {:4}. Projectile is at ({:.3}, {:.3}, {:.3})...",
            num_ticks, p.position.x, p.position.y, p.position.z
        );
        let new_p = w.tick(p);
        p = new_p;
    }
}
