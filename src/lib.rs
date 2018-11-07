pub mod camera;
pub mod canvas;
pub mod intersections;
pub mod lighting;
pub mod materials;
pub mod matrices;
pub mod rays;
pub mod spheres;
pub mod transforms;
pub mod tuples;
pub mod world;

const EPSILON: f32 = 0.001;

pub fn float_eq(left: f32, right: f32) -> bool {
    (left - right).abs() < EPSILON
}
