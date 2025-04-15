use crate::float2::Float2;
use crate::line::Line;
use crate::utils::{direction_to_vector, intersection_distance};

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

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

// speed = m/s
// rotation_speed = degree/s
pub struct Robot {
    state: Arc<Mutex<RobotState>>,
    speed: f32,
    rotation_speed: f32,
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
            speed: 200.0,
            rotation_speed: 30.0,
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

    fn rotate(&mut self, rotation: &Rotation, elapsed: &Duration) {
        let mut state = self.state.lock().unwrap();
        match rotation {
            Rotation::Left => state.direction -= elapsed.as_secs_f32() * self.rotation_speed,
            Rotation::Right => state.direction += elapsed.as_secs_f32() * self.rotation_speed,
            Rotation::None => {}
        }
        drop(state);
    }

    pub fn run(self, room: Arc<Vec<Line>>, quit: Arc<AtomicBool>) -> JoinHandle<()> {
        let state = Arc::clone(&self.state);
        thread::spawn(move || {
            let mut robot = self;

            let mut last_updated = Instant::now();
            let update_interval = Duration::from_millis(10);

            // rotate to nearest wall
            'rotate: loop {
                let now = Instant::now();
                let elapsed = now.duration_since(last_updated);

                if elapsed >= update_interval {
                    last_updated = now;

                    if quit.load(Ordering::Relaxed) {
                        break 'rotate;
                    }

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
                    drop(state);

                    // 0.0 = robot forward direction
                    if min_dist_dir == 0.0 {
                        break 'rotate;
                    }
                    robot.rotate(&Rotation::Left, &elapsed);
                } else {
                    let sleep_duration = update_interval - elapsed;
                    thread::sleep(sleep_duration);
                }
            }

            'moving: loop {
                let now = Instant::now();
                let elapsed = now.duration_since(last_updated);

                if elapsed >= update_interval {
                    last_updated = now;

                    if quit.load(Ordering::Relaxed) {
                        break 'moving;
                    }

                    robot.lidar_scan(&room);
                    robot.check_collision(&room);
                    if robot.sensor_collision {
                        break 'moving;
                    }
                    robot.moving(&Direction::Forward, &elapsed);
                } else {
                    let sleep_duration = update_interval - elapsed;
                    thread::sleep(sleep_duration);
                }
            }

            loop {
                if quit.load(Ordering::Relaxed) {
                    break;
                }
                thread::sleep(Duration::from_millis(100));
            }
        })
    }
}
