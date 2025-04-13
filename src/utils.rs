use crate::float2::Float2;
use std::f32::consts::PI;

pub const RADIANS: f32 = PI / 180.0;

pub fn direction_to_vector(direction: f32) -> Float2 {
    let radians = direction * RADIANS;
    let vector = Float2::new(radians.cos(), radians.sin());
    vector.make_unit()
}
