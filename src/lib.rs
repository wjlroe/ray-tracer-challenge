extern crate num;

pub mod canvas;
pub mod matrices;
pub mod rays;
pub mod tuples;
pub mod world;

const EPSILON: f32 = 0.00001;

pub fn float_eq(left: f32, right: f32) -> bool {
    (left - right).abs() < EPSILON
}
