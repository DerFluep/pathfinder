use crate::float2::Float2;
use crate::line::Line;
use crate::utils::{direction_to_vector, intersection_distance};
use crate::window::Viewport;

use std::time::Duration;

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
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            direction: 0.0,
            lidar: vec![0.0; 360],
            position: Float2::new(x, y),
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

    fn lidar_scan(&mut self, room: &Vec<Line>) {
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

    fn check_collision(&mut self, room: &Vec<Line>) {
        self.sensor_collision = false;
        let pos_x = self.position.get_x();
        let pos_y = self.position.get_y();
        room.iter().for_each(|wall| {
            let x1 = wall.get_a().get_x();
            let x2 = wall.get_b().get_x();
            let y1 = wall.get_a().get_y();
            let y2 = wall.get_b().get_y();

            if x1 == x2 {
                let distance = (pos_x - x1).abs();
                if distance <= self.radius {
                    self.sensor_collision = true;
                }
            }

            let slope = (y2 - y1) / (x2 - x1);
            let y_intercept = y1 - slope * x1;

            let a = 1.0 + slope.powi(2);
            let b = 2.0 * slope * (y_intercept - pos_y) - 2.0 * pos_x;
            let c = pos_x.powi(2) + (y_intercept - pos_y).powi(2) - self.radius.powi(2);

            let discriminant = b.powi(2) - 4.0 * a * c;

            if discriminant >= 0.0 {
                self.sensor_collision = true;
            }
        });
    }

    fn moving(&mut self, direction: &Direction) {
        let vector = direction_to_vector(self.direction);
        match direction {
            Direction::Forward => self.position += vector * 5.0,
            Direction::Backward => self.position -= vector * 5.0,
            Direction::None => {}
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        match rotation {
            Rotation::Left => self.direction -= 1.0,
            Rotation::Right => self.direction += 1.0,
            Rotation::None => {}
        }
    }

    pub fn run(&mut self, room: &Vec<Line>, viewport: &mut Viewport) {
        // rotate to nearest wall
        'rotate: loop {
            if viewport.get_input() {
                break 'rotate;
            };
            let mut min_dist = f32::MAX;
            let mut min_dist_dir = f32::MAX;
            self.lidar_scan(&room);
            self.lidar.iter().enumerate().for_each(|(num, dist)| {
                if *dist < min_dist {
                    min_dist = *dist;
                    min_dist_dir = num as f32;
                }
            });
            // 0.0 = robot forward direction
            if min_dist_dir == 0.0 {
                break 'rotate;
            }
            self.rotate(&Rotation::Left);
            viewport.draw(&room, &self);
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        'moving: loop {
            if viewport.get_input() {
                break 'moving;
            };
            self.lidar_scan(&room);
            self.check_collision(&room);
            if self.sensor_collision {
                break 'moving;
            }
            self.moving(&Direction::Forward);
            viewport.draw(&room, &self);
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }

        'endloop: loop {
            if viewport.get_input() {
                break 'endloop;
            }
            viewport.draw(&room, &self);
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
