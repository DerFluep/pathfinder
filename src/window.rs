extern crate sdl3;

use crate::line::Line;
use crate::robot::{Direction, Robot, Rotation};
use crate::utils::direction_to_vector;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::{Canvas, FPoint, FRect};
use sdl3::video::Window;
use sdl3::EventPump;

const SCALE: f32 = 10.0;

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
            .window("Pathfinder", 600, 600)
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

    pub fn draw(&mut self, room: &Vec<Line>, robot: &Robot) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Draw walls
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        room.iter().for_each(|wall| {
            self.canvas
                .draw_line(
                    FPoint::new(wall.get_a().get_x() / 10.0, wall.get_a().get_y() / SCALE),
                    FPoint::new(wall.get_b().get_x() / 10.0, wall.get_b().get_y() / SCALE),
                )
                .unwrap();
        });

        // Draw robot
        self.canvas
            .draw_rect(FRect::new(
                (robot.get_position().get_x() - robot.get_radius()) / SCALE,
                (robot.get_position().get_y() - robot.get_radius()) / SCALE,
                robot.get_radius() * 2.0 / SCALE,
                robot.get_radius() * 2.0 / SCALE,
            ))
            .unwrap();
        let vector = direction_to_vector(robot.get_direction());
        let line_end = vector * robot.get_radius() + robot.get_position();
        self.canvas
            .draw_line(
                FPoint::new(
                    robot.get_position().get_x() / SCALE,
                    robot.get_position().get_y() / SCALE,
                ),
                FPoint::new(line_end.get_x() / SCALE, line_end.get_y() / SCALE),
            )
            .unwrap();

        // Draw Lidar
        if self.show_lidar {
            self.canvas.set_draw_color(Color::RGB(0, 255, 0));
            robot
                .get_lidar()
                .iter()
                .enumerate()
                .for_each(|(num, distance)| {
                    let vector = direction_to_vector(num as f32 + robot.get_direction());
                    let colision_point = (vector * *distance + robot.get_position()) / SCALE;
                    self.canvas
                        .draw_line(
                            FPoint::new(
                                robot.get_position().get_x() / SCALE,
                                robot.get_position().get_y() / SCALE,
                            ),
                            FPoint::new(colision_point.get_x(), colision_point.get_y()),
                        )
                        .unwrap()
                });
        }
        self.canvas.present();
    }
}
