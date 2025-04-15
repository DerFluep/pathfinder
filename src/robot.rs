use crate::float2::Float2;
use crate::line::Line;
use crate::utils::{direction_to_vector, intersection_distance};

use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
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

pub struct RobotState {
    direction: f32,
    lidar: Vec<f32>,
    position: Float2,
    radius: f32,
}

pub struct Robot {
    state: Arc<Mutex<RobotState>>,
    sensor_collision: bool,
    sensor_wall: bool,
}

impl Robot {
    pub fn new(x: f32, y: f32) -> Self {
        let state = RobotState {
            direction: 0.0,
            lidar: vec![0.0; 360],
            position: Float2::new(x, y),
            radius: 175.0,
        };
        Self {
            state: Arc::new(Mutex::new(state)),
            sensor_collision: false,
            sensor_wall: false,
        }
    }

    pub fn get_state(&self) -> Arc<Mutex<RobotState>> {
        Arc::clone(&self.state)
    }

    fn lidar_scan(&mut self, room: &Arc<Vec<Line>>) {
        let mut state = self.state.lock().unwrap();
        let direction = state.direction;
        let position = state.position;
        state
            .lidar
            .iter_mut()
            .enumerate()
            .for_each(|(num, distance)| {
                let ray = direction_to_vector(num as f32 + direction);
                let mut closest = 10000.0;
                room.iter().for_each(|wall| {
                    let distance = intersection_distance(position, ray, *wall);
                    if distance < closest {
                        closest = distance;
                    }
                });
                *distance = closest;
            });
    }

    fn check_collision(&mut self, room: &Arc<Vec<Line>>) {
        let state = self.state.lock().unwrap();
        self.sensor_collision = false;
        let pos_x = state.position.get_x();
        let pos_y = state.position.get_y();
        room.iter().for_each(|wall| {
            let x1 = wall.get_a().get_x();
            let x2 = wall.get_b().get_x();
            let y1 = wall.get_a().get_y();
            let y2 = wall.get_b().get_y();

            // check if the line is vertical
            // vertical lines have no slope
            if x1 == x2 {
                let distance = (pos_x - x1).abs();
                if distance <= state.radius {
                    self.sensor_collision = true;
                }
                return;
            }

            let slope = (y2 - y1) / (x2 - x1);
            let y_intercept = y1 - slope * x1;

            let a = 1.0 + slope.powi(2);
            let b = 2.0 * slope * (y_intercept - pos_y) - 2.0 * pos_x;
            let c = pos_x.powi(2) + (y_intercept - pos_y).powi(2) - state.radius.powi(2);

            let discriminant = b.powi(2) - 4.0 * a * c;

            // discriminant == 0.0 -> one intersection point
            // discriminant > 0.0 -> two intersection points
            if discriminant >= 0.0 {
                self.sensor_collision = true;
            }
        });
    }

    fn moving(&mut self, direction: &Direction) {
        let mut state = self.state.lock().unwrap();
        let vector = direction_to_vector(state.direction);
        match direction {
            Direction::Forward => state.position += vector * 5.0,
            Direction::Backward => state.position -= vector * 5.0,
            Direction::None => {}
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        let mut state = self.state.lock().unwrap();
        match rotation {
            Rotation::Left => state.direction -= 1.0,
            Rotation::Right => state.direction += 1.0,
            Rotation::None => {}
        }
    }

    pub fn run(self, room: Arc<Vec<Line>>) -> JoinHandle<()> {
        let state = Arc::clone(&self.state);
        let sensor_collision = self.sensor_collision;
        thread::spawn(move || {
            let mut robot = self;
            // rotate to nearest wall
            'rotate: loop {
                let mut min_dist = f32::MAX;
                let mut min_dist_dir = f32::MAX;
                robot.lidar_scan(&room);
                let state = state.lock().unwrap();
                state.lidar.iter().enumerate().for_each(|(num, dist)| {
                    if *dist < min_dist {
                        min_dist = *dist;
                        min_dist_dir = num as f32;
                    }
                });
                // 0.0 = robot forward direction
                if min_dist_dir == 0.0 {
                    break 'rotate;
                }
                robot.rotate(&Rotation::Left);
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }

            'moving: loop {
                robot.lidar_scan(&room);
                robot.check_collision(&room);
                if sensor_collision {
                    break 'moving;
                }
                robot.moving(&Direction::Forward);
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }

            loop {
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        })
    }
}
