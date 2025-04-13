mod float2;
mod line;

use crate::float2::Float2;
use crate::line::Line;
use std::f64::consts::PI;

const RADIUS: f64 = 175.0;
const RADIANS: f64 = PI / 180.0;

// X goes to the right
// Y goes up
// 0 = x, 1 = y

struct Robot {
    direction: f64,
    lidar: Vec<f64>,
    position: Float2,
    radius: f64,
    sensor_collision: bool,
    sensor_wall: bool,
}

impl Robot {
    fn new() -> Self {
        Self {
            direction: 0.0,
            lidar: vec![0.0; 360],
            position: Float2::new(0.0, 0.0),
            radius: RADIUS,
            sensor_collision: false,
            sensor_wall: false,
        }
    }

    // ToDo
    // - convert line to vector
    // - check vector intersection
    // https://www.gamedev.net/forums/topic/647810-intersection-point-of-two-vectors/5094127/
    // - check if intersectionpoint is within the line
    //     - if so: collision == true
    fn lidar_scan(&self, room: Vec<Line>) {
        let direction = 45.0 * RADIANS;
        let ray_lidar = Float2::new(direction.cos(), direction.sin()) + self.position;
        let mut wall_ray = room[0].get_b() - room[0].get_a();
        let wall_ray_length = wall_ray.length();
        wall_ray = wall_ray.make_unit() + room[0].get_a();
    }

    fn set_pos(&mut self, pos: Float2) {
        self.position = pos;
    }
}

fn bounding_box(room: &Vec<Line>) {
    let (mut min_x, mut min_y) = (f64::MAX, f64::MAX);
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
}

fn main() {
    let room = vec![
        Line::new(Float2::new(0.0, 0.0), Float2::new(5000.0, 0.0)),
        Line::new(Float2::new(5000.0, 0.0), Float2::new(5000.0, 5000.0)),
        Line::new(Float2::new(0.0, 5000.0), Float2::new(5000.0, 5000.0)),
        Line::new(Float2::new(0.0, 0.0), Float2::new(0.0, 5000.0)),
    ];

    for wall in room.iter() {
        wall.print();
    }

    bounding_box(&room);

    let ilse = Robot::new();
}
