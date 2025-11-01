use crate::float2::Float2;
use crate::line::Line;
use crate::utils::{direction_to_vector, intersection_distance, run_with_interval};

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub enum Direction {
    Forward,
    Backward,
    None,
}

pub enum Rotation {
    Left,
    Right,
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
            speed: 400.0,
            rotation_speed: 60.0,
            sensor_collision: false,
            sensor_wall: 0.0,
            interval: Duration::from_millis(10),
        }
    }

    pub fn get_state(&self) -> Arc<Mutex<RobotState>> {
        Arc::clone(&self.state)
    }

    fn scan_lidar(&mut self, room: &Arc<Vec<Line>>) {
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

    fn rotate(&mut self, rotation: f32, elapsed: &Duration) {
        let mut state = self.state.lock().unwrap();
        state.direction +=
            elapsed.as_secs_f32() * rotation.clamp(-self.rotation_speed, self.rotation_speed);
        drop(state);
    }

    fn goto_nearest_wall(&mut self, room: &Arc<Vec<Line>>, quit: Arc<AtomicBool>) {
        // rotate to nearest wall
        run_with_interval(self.interval, &quit, |elapsed| {
            let mut min_dist = f32::MAX;
            let mut min_dist_dir = usize::MAX;
            self.scan_lidar(room);

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
                self.rotate(self.rotation_speed, &elapsed);
            } else {
                self.rotate(-self.rotation_speed, &elapsed);
            }

            false // continue looping
        });

        run_with_interval(self.interval, &quit, |elapsed| {
            self.scan_lidar(room);

            let mut min_dist = f32::MAX;
            let state = self.state.lock().unwrap();
            state.lidar.iter().for_each(|x| {
                if *x < min_dist {
                    min_dist = *x;
                }
            });

            if min_dist <= state.radius + 10.0 {
                return true;
            }
            drop(state);

            self.moving(&Direction::Forward, &elapsed);
            false
        });
    }

    pub fn run(self, room: Arc<Vec<Line>>, quit: Arc<AtomicBool>) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut robot = self;

            robot.goto_nearest_wall(&room, Arc::clone(&quit));

            // rotate 90deg to wall
            run_with_interval(robot.interval, &quit, |elapsed| {
                robot.scan_lidar(&room);

                let mut min_dir = 0;
                let mut min_val = 10000.0;
                let state = robot.state.lock().unwrap();
                state.lidar.iter().enumerate().for_each(|(num, x)| {
                    if *x < min_val {
                        min_dir = num;
                        min_val = *x;
                    }
                });
                drop(state);

                if min_dir == 270 {
                    return true;
                }

                robot.rotate(robot.rotation_speed, &elapsed);
                false
            });

            // follow wall
            let mut last_error = 0.0;
            let mut integral = 0.0;
            run_with_interval(robot.interval, &quit, |elapsed| {
                robot.scan_lidar(&room);
                robot.check_collision(&room);
                robot.check_wall(&room);

                let error = robot.sensor_wall - 21.5; // 21.5 ~ 10mm to the wall
                let p = error;
                integral += error;
                let i = integral;
                let d = error - last_error;

                // TODO: tweak p i and d values
                let correction = p * 0.5 + i * 0.001 + d * 20.0;

                robot.rotate(-correction, &elapsed);
                robot.moving(&Direction::Forward, &elapsed);

                last_error = error;

                false
            });
        })
    }
}
