extern crate sdl3;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::float2::Float2;
use crate::line::Line;
use crate::robot::RobotState;
use crate::utils::direction_to_vector;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::{Canvas, FPoint};
use sdl3::video::Window;
use sdl3::EventPump;

const SCALE: f32 = 10.0;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn draw_circle(render: &mut Canvas<Window>, position: Float2, radius: f32) {
    let diameter = radius * 2.0 / SCALE;

    let pos_x = position.get_x() / SCALE;
    let pos_y = position.get_y() * -1.0 / SCALE + HEIGHT as f32;
    let mut x = radius / SCALE - 1.0;
    let mut y = 0.0;
    let mut tx = 1.0;
    let mut ty = 1.0;
    let mut error = tx - diameter;

    render.set_draw_color(Color::RGB(255, 0, 0));
    while x >= y {
        render
            .draw_point(FPoint::new(pos_x + x, pos_y - y))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x + x, pos_y + y))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x - x, pos_y - y))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x - x, pos_y + y))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x + y, pos_y - x))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x + y, pos_y + x))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x - y, pos_y - x))
            .unwrap();
        render
            .draw_point(FPoint::new(pos_x - y, pos_y + x))
            .unwrap();

        if error <= 0.0 {
            y += 1.0;
            error += ty;
            ty += 2.0;
        }

        if error > 0.0 {
            x -= 1.0;
            tx += 2.0;
            error += tx - diameter;
        }
    }
}

pub struct Viewport {
    canvas: Canvas<Window>,
    event_pump: EventPump,
    show_lidar: bool,
}

impl Viewport {
    pub fn new() -> Self {
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Pathfinder", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas();
        let event_pump = sdl_context.event_pump().unwrap();
        Self {
            canvas,
            event_pump,
            show_lidar: false,
        }
    }

    pub fn get_input(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => self.show_lidar = !self.show_lidar,
                _ => {}
            }
        }
        false
    }

    pub fn draw(
        &mut self,
        room: Arc<Vec<Line>>,
        robot: &Arc<Mutex<RobotState>>,
        quit: Arc<AtomicBool>,
    ) {
        let mut last_updated = Instant::now();
        let update_interval = Duration::from_millis(16);

        'running: loop {
            let now = Instant::now();
            let elapsed = now.duration_since(last_updated);

            if elapsed >= update_interval {
                last_updated = now;
                if self.get_input() {
                    quit.store(true, Ordering::Relaxed);
                    break 'running;
                }

                self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                self.canvas.clear();

                // Draw walls
                self.canvas.set_draw_color(Color::RGB(255, 0, 0));
                room.iter().for_each(|wall| {
                    self.canvas
                        .draw_line(
                            FPoint::new(
                                wall.get_a().get_x() / SCALE,
                                wall.get_a().get_y() * -1.0 / SCALE + HEIGHT as f32,
                            ),
                            FPoint::new(
                                wall.get_b().get_x() / SCALE,
                                wall.get_b().get_y() * -1.0 / SCALE + HEIGHT as f32,
                            ),
                        )
                        .unwrap();
                });

                // Draw robot
                let robot_state = robot.lock().unwrap();
                draw_circle(&mut self.canvas, robot_state.position, robot_state.radius);
                let vector = direction_to_vector(robot_state.direction);
                let line_end = vector * robot_state.radius + robot_state.position;
                self.canvas
                    .draw_line(
                        FPoint::new(
                            robot_state.position.get_x() / SCALE,
                            robot_state.position.get_y() * -1.0 / SCALE + HEIGHT as f32,
                        ),
                        FPoint::new(
                            line_end.get_x() / SCALE,
                            line_end.get_y() * -1.0 / SCALE + HEIGHT as f32,
                        ),
                    )
                    .unwrap();

                // Draw Lidar
                if self.show_lidar {
                    self.canvas.set_draw_color(Color::RGB(0, 255, 0));
                    robot_state
                        .lidar
                        .iter()
                        .enumerate()
                        .for_each(|(num, distance)| {
                            let vector = direction_to_vector(num as f32 + robot_state.direction);
                            let colision_point = (vector * *distance + robot_state.position);
                            self.canvas
                                .draw_line(
                                    FPoint::new(
                                        robot_state.position.get_x() / SCALE,
                                        robot_state.position.get_y() * -1.0 / SCALE + HEIGHT as f32,
                                    ),
                                    FPoint::new(
                                        colision_point.get_x() / SCALE,
                                        colision_point.get_y() * -1.0 / SCALE + HEIGHT as f32,
                                    ),
                                )
                                .unwrap()
                        });
                }
                self.canvas.present();
            } else {
                let sleep_duration = update_interval - elapsed;
                thread::sleep(sleep_duration);
            }
        }
    }
}
