pub mod camera;
pub mod canvas;
pub mod intersections;
pub mod lighting;
pub mod materials;
pub mod matrices;
pub mod patterns;
pub mod rays;
pub mod shapes;
pub mod transforms;
pub mod tuples;
pub mod world;

pub const EPSILON: f32 = 0.00001;
pub const REFLECTION_RECURSION_LIMIT: i32 = 5;

pub fn float_eq(left: f32, right: f32) -> bool {
    (left - right).abs() < EPSILON
}
