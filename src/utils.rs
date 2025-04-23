use crate::{float2::Float2, line::Line};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::{Duration, Instant},
};

pub fn direction_to_vector(direction: f32) -> Float2 {
    let radians = direction.to_radians();
    Float2::new(radians.cos(), radians.sin())
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

    // max lidar distance 20m
    let mut distance = 20000.0;
    if t >= 0.0 && 0.0 <= s && s <= 1.0 {
        distance = t;
    }
    distance
}

pub fn run_with_interval<F>(interval: Duration, quit: &AtomicBool, mut f: F)
where
    F: FnMut(Duration) -> bool,
{
    let mut last_updated = Instant::now();

    loop {
        let now = Instant::now();
        let elapsed = now.duration_since(last_updated);

        if elapsed >= interval {
            last_updated = now;

            if quit.load(Ordering::Relaxed) {
                break;
            }

            // If closure returns true, break the loop
            if f(elapsed) {
                break;
            }
        } else {
            let sleep_duration = interval - elapsed;
            thread::sleep(sleep_duration);
        }
    }
}
