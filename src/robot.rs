use crate::float2::Float2;
use crate::line::Line;
use crate::utils::{direction_to_vector, intersection_distance};

pub enum Rotation {
    Left,
    Right,
    None,
}

pub enum Direction {
    Forward,
    Backward,
    None,
}

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
            direction: 90.0,
            lidar: vec![0.0; 360],
            position: Float2::new(1000.0, 1000.0),
            radius: 175.0,
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

    pub fn get_lidar(&self) -> Vec<f32> {
        self.lidar.clone()
    }

    // ToDo
    // - convert line to vector
    // - check vector intersection
    // https://www.gamedev.net/forums/topic/647810-intersection-point-of-two-vectors/5094127/
    // - check if intersectionpoint is within the line
    //     - if so: collision == true
    pub fn lidar_scan(&mut self, room: &Vec<Line>) {
        self.lidar
            .iter_mut()
            .enumerate()
            .for_each(|(num, distance)| {
                let ray = direction_to_vector(num as f32);
                let mut closest = f32::MAX;
                room.iter().for_each(|wall| {
                    let distance = intersection_distance(self.position, ray, *wall);
                    if distance < closest {
                        closest = distance;
                    }
                });
                *distance = closest;
            });
    }

    pub fn moving(&mut self, direction: &Direction) {
        let vector = direction_to_vector(self.direction);
        match direction {
            Direction::Forward => self.position += vector * 5.0,
            Direction::Backward => self.position -= vector * 5.0,
            Direction::None => {}
        }
    }

    pub fn rotate(&mut self, rotation: &Rotation) {
        match rotation {
            Rotation::Left => self.direction -= 3.0,
            Rotation::Right => self.direction += 3.0,
            Rotation::None => {}
        }
    }
}
