extern crate ray_tracer_challenge;

use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::matrices::Matrix4;
use ray_tracer_challenge::tuples::Tuple;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let clock_radius = 100; // radius of the clock in pixels
    let canvas_dim = clock_radius * 2 + 20 * 2;
    let center_point = canvas_dim / 2;
    let mut canvas = Canvas::new(canvas_dim, canvas_dim);
    let hand_color = Tuple::color(1.0, 1.0, 1.0);
    let recenter_on_origin =
        Matrix4::translation(center_point as f32, center_point as f32, 0.0);
    let scaling =
        Matrix4::scaling(clock_radius as f32, clock_radius as f32, 1.0);
    let transformations = recenter_on_origin * scaling;
    let hour12 = Tuple::point(0.0, 1.0, 0.0);
    let mut hands = Vec::new();
    for i in 0..12 {
        let rotation = Matrix4::rotation_z(i as f32 * PI / 6.0);
        let hand = transformations * rotation * hour12;
        hands.push(hand);
    }
    for hand in hands {
        println!(
            "Dot at: ({}, {})",
            hand.x.round() as u32,
            hand.y.round() as u32
        );
        canvas.write_pixel(
            hand.x.round() as u32,
            hand.y.round() as u32,
            &hand_color,
        );
    }
    let ppm = canvas.to_ppm();
    let filename = "clock.ppm";
    let mut file = File::create(filename)?;
    file.write_all(ppm.as_bytes())?;
    Ok(())
}
