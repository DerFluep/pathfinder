use crate::float2::Float2;
use crate::line::Line;
use crate::utils::{direction_to_vector, intersection_distance, run_with_interval};

use std::sync::atomic::AtomicBool;
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
    pub direction: f32,
    pub lidar: Vec<f32>,
    pub position: Float2,
    pub radius: f32,
}

// speed = mm/s
// rotation_speed = degree/s
pub struct Robot {
    state: Arc<Mutex<RobotState>>,
    speed: f32,
    rotation_speed: f32,
    sensor_collision: bool,
    sensor_wall: f32,
    interval: Duration,
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
            speed: 200.0,
            rotation_speed: 30.0,
            sensor_collision: false,
            sensor_wall: 0.0,
            interval: Duration::from_millis(10),
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
        let radius = state.radius;
        drop(state);

        for wall in room.iter() {
            let x1 = wall.get_a().get_x();
            let x2 = wall.get_b().get_x();
            let y1 = wall.get_a().get_y();
            let y2 = wall.get_b().get_y();

            // check if the line is vertical
            // vertical lines have no slope
            if x1 == x2 {
                let distance = (pos_x - x1).abs();
                if distance <= radius {
                    self.sensor_collision = true;
                    break;
                }
            }

            let slope = (y2 - y1) / (x2 - x1);
            let y_intercept = y1 - slope * x1;

            let a = 1.0 + slope.powi(2);
            let b = 2.0 * slope * (y_intercept - pos_y) - 2.0 * pos_x;
            let c = pos_x.powi(2) + (y_intercept - pos_y).powi(2) - radius.powi(2);

            let discriminant = b.powi(2) - 4.0 * a * c;

            // discriminant == 0.0 -> one intersection point
            // discriminant > 0.0 -> two intersection points
            if discriminant >= 0.0 {
                self.sensor_collision = true;
                break;
            }
        }
    }

    fn check_wall(&mut self, room: &Arc<Vec<Line>>) {
        self.sensor_wall = 0.0;
        let mut min_dist = 50.0;
        for wall in room.iter() {
            let state = self.state.lock().unwrap();
            let vector = direction_to_vector(state.direction + 290.0); // shoot the ray at an 20deg angle
            let origin = state.position
                + direction_to_vector(state.direction + 270.0) * (state.radius - 10.0);
            let distance = intersection_distance(origin, vector, *wall);
            if distance < min_dist {
                min_dist = distance
            };
            self.sensor_wall = min_dist;
        }
    }

    fn moving(&mut self, direction: &Direction, elapsed: &Duration) {
        let mut state = self.state.lock().unwrap();
        let vector = direction_to_vector(state.direction);
        match direction {
            Direction::Forward => state.position += vector * elapsed.as_secs_f32() * self.speed,
            Direction::Backward => state.position -= vector * elapsed.as_secs_f32() * self.speed,
            Direction::None => {}
        }
        drop(state);
    }

    fn rotate(&mut self, rotation: Rotation, elapsed: &Duration) {
        let mut state = self.state.lock().unwrap();
        match rotation {
            Rotation::Left => state.direction += elapsed.as_secs_f32() * self.rotation_speed,
            Rotation::Right => state.direction -= elapsed.as_secs_f32() * self.rotation_speed,
            Rotation::None => {}
        }
        drop(state);
    }

    fn goto_nearest_wall(&mut self, room: &Arc<Vec<Line>>, quit: Arc<AtomicBool>) {
        // rotate to nearest wall
        run_with_interval(self.interval, &quit, |elapsed| {
            let mut min_dist = f32::MAX;
            let mut min_dist_dir = usize::MAX;
            self.lidar_scan(&room);

            let state = self.state.lock().unwrap();
            state.lidar.iter().enumerate().for_each(|(num, dist)| {
                if *dist < min_dist {
                    min_dist = *dist;
                    min_dist_dir = num;
                }
            });
            drop(state);

            // 0.0 = robot forward direction
            if min_dist_dir == 0 {
                return true; // stop looping
            }
            if min_dist_dir <= 180 {
                self.rotate(Rotation::Left, &elapsed);
            } else {
                self.rotate(Rotation::Right, &elapsed);
            }

            false // continue looping
        });

        // TODO convert the counter so time units instead of counting time steps
        // TODO convert backward movement from time to distance
        let mut is_backwards = false;
        let mut back_counter = 0;
        let mut direction = Direction::Forward;
        run_with_interval(self.interval, &quit, |elapsed| {
            self.lidar_scan(&room);
            self.check_collision(&room);
            if self.sensor_collision {
                direction = Direction::Backward;
                is_backwards = true;
            }
            // move a little backwards so get some clearance to the wall
            if is_backwards {
                back_counter += 1;
            }
            if back_counter == 10 {
                return true;
            }

            self.moving(&direction, &elapsed);
            false
        });
    }

    pub fn run(self, room: Arc<Vec<Line>>, quit: Arc<AtomicBool>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut robot = self;

            robot.goto_nearest_wall(&room, Arc::clone(&quit));

            // rotate 90deg to wall
            let mut min_dist = 1000.0;
            run_with_interval(robot.interval, &quit, |elapsed| {
                robot.lidar_scan(&room);
                robot.check_wall(&room);
                if robot.sensor_wall < min_dist {
                    min_dist = robot.sensor_wall;
                }
                if robot.sensor_wall > min_dist + 1.0 {
                    // + <> is the offset to rotation relative to the wall
                    return true;
                }
                robot.rotate(Rotation::Left, &elapsed);
                false
            });

            // TODO smooth the rotation
            // follow wall
            run_with_interval(robot.interval, &quit, |elapsed| {
                let last_dist = robot.sensor_wall;
                let rotation;
                robot.lidar_scan(&room);
                robot.check_wall(&room);
                if robot.sensor_wall < last_dist {
                    rotation = Rotation::Left;
                } else {
                    rotation = Rotation::Right;
                }

                robot.check_collision(&room);
                if robot.sensor_collision {
                    return true;
                }

                robot.rotate(rotation, &elapsed);
                robot.moving(&Direction::Forward, &elapsed);
                false
            });
        })
    }
}
