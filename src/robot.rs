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
            direction: 0.0,
            lidar: vec![0.0; 360],
            position: Float2::new(2000.0, 1000.0),
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

    pub fn get_direction(&self) -> f32 {
        self.direction
    }

    pub fn get_lidar(&self) -> Vec<f32> {
        self.lidar.clone()
    }

    pub fn lidar_scan(&mut self, room: &Vec<Line>) {
        self.lidar
            .iter_mut()
            .enumerate()
            .for_each(|(num, distance)| {
                let ray = direction_to_vector(num as f32 + self.direction);
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
            Rotation::Left => self.direction -= 1.0,
            Rotation::Right => self.direction += 1.0,
            Rotation::None => {}
        }
    }

    pub fn run(&mut self, room: &Vec<Line>) {
        // rotate to nearest wall
        let mut min_dist = f32::MAX;
        let mut min_dist_dir = f32::MAX;
        self.lidar_scan(room);
        self.lidar.iter().enumerate().for_each(|(num, dist)| {
            if *dist < min_dist {
                min_dist = *dist;
                min_dist_dir = num as f32;
            }
        });

        if min_dist_dir != 0.0 {
            self.rotate(&Rotation::Left);
        } else {
            // move until colision
            self.moving(&Direction::Forward);
        }
    }
}
