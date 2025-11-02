mod float2;
mod line;
mod robot;
mod utils;
mod window;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use window::Viewport;

use crate::float2::Float2;
use crate::line::Line;
use crate::robot::Robot;

// X goes to the right
// Y goes down

// TODO: update to use new Arc<Mutex<>> room

/* fn bounding_box(room: &Vec<Line>) {
    let (mut min_x, mut min_y) = (f32::MAX, f32::MAX);
    let (mut max_x, mut max_y) = (0.0, 0.0);
    let mut arr_x = Vec::new();
    let mut arr_y = Vec::new();

    room.iter().for_each(|wall| {
        arr_x.push(wall.get_a().get_x());
        arr_x.push(wall.get_b().get_x());
        arr_y.push(wall.get_a().get_y());
        arr_y.push(wall.get_b().get_y());
    });

    arr_x.iter().for_each(|x| {
        if x < &min_x {
            min_x = *x;
        } else if x > &max_x {
            max_x = *x;
        }
    });

    arr_y.iter().for_each(|y| {
        if y < &min_y {
            min_y = *y;
        } else if y > &max_y {
            max_y = *y;
        }
    });

    println!("min x: {} y: {}", min_x, min_y);
    println!("max x: {} y: {}", max_x, max_y);
} */

fn main() {
    let room = Arc::new(vec![
        Line::new(Float2::new(0.0, 0.0), Float2::new(5000.0, 0.0)),
        Line::new(Float2::new(5000.0, 0.0), Float2::new(5000.0, 5000.0)),
        Line::new(Float2::new(0.0, 5000.0), Float2::new(5000.0, 5000.0)),
        Line::new(Float2::new(0.0, 0.0), Float2::new(0.0, 5000.0)),
    ]);
    let room2 = Arc::new(vec![
        Line::new(Float2::new(2500.0, 5000.0), Float2::new(2500.0, 2500.0)),
        Line::new(Float2::new(3000.0, 2500.0), Float2::new(0.0, 2500.0)),
    ]);

    let quit = Arc::new(AtomicBool::new(false));

    let ilse = Robot::new(3000.0, 4700.0);
    let ilse_state = ilse.get_state();
    let robot_thread = ilse.run(Arc::clone(&room2), Arc::clone(&quit));

    let mut viewport = Viewport::new();
    viewport.draw(Arc::clone(&room2), &ilse_state, Arc::clone(&quit));

    robot_thread.join().unwrap();
}
