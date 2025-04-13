use crate::float2::Float2;
use crate::line::Line;
use std::f32::consts::PI;

const RADIUS: f32 = 175.0;
const RADIANS: f32 = PI / 180.0;

pub struct Robot {
    direction: f32,
    lidar: Vec<f32>,
    position: Float2,
    radius: f32,
    sensor_collision: bool,
    sensor_wall: bool,
}

impl Robot {
    pub fn new() -> Self {
        Self {
            direction: 0.0,
            lidar: vec![0.0; 360],
            position: Float2::new(1000.0, 1000.0),
            radius: RADIUS,
            sensor_collision: false,
            sensor_wall: false,
        }
    }

    pub fn get_position(&self) -> Float2 {
        self.position
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
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

    pub fn set_pos(&mut self, pos: Float2) {
        self.position = pos;
    }
}
