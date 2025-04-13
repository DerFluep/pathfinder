use crate::{float2::Float2, line::Line};
use std::f32::consts::PI;

pub const RADIANS: f32 = PI / 180.0;

pub fn direction_to_vector(direction: f32) -> Float2 {
    let radians = direction * RADIANS;
    let vector = Float2::new(radians.cos(), radians.sin());
    vector.make_unit()
}

pub fn intersection_distance(origin: Float2, vector: Float2, line: Line) -> f32 {
    let origin_x = origin.get_x();
    let origin_y = origin.get_y();
    let vector_x = vector.get_x();
    let vector_y = vector.get_y();
    let point1_x = line.get_a().get_x();
    let point1_y = line.get_a().get_y();
    let point2_x = line.get_b().get_x();
    let point2_y = line.get_b().get_y();

    // Line direction
    let dx = point2_x - point1_x;
    let dy = point2_y - point1_y;

    let denom = vector_x * dy - vector_y * dx;

    /* if denom == 0.0 {
        println!("Lines are parallel");
    } */

    let t = ((point1_x - origin_x) * dy - (point1_y - origin_y) * dx) / denom;
    let s = ((point1_x - origin_x) * vector_y - (point1_y - origin_y) * vector_x) / denom;

    let mut distance = f32::MAX;
    if t >= 0.0 && 0.0 <= s && s <= 1.0 {
        distance = t;
    }
    distance
}
