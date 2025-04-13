extern crate sdl3;

use crate::line::Line;
use crate::robot::{Direction, Robot, Rotation};
use crate::utils::direction_to_vector;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::{FPoint, FRect};
use std::time::Duration;

pub fn create_window(room: &Vec<Line>, robot: &mut Robot) {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Pathfinder", 600, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut show_lidar = false;

    let mut rotation = Rotation::None;
    let mut direction = Direction::Forward;
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => rotation = Rotation::Left,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => direction = Direction::Forward,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => direction = Direction::Backward,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => show_lidar = !show_lidar,
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => rotation = Rotation::None,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => rotation = Rotation::Right,
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => rotation = Rotation::None,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw walls
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        room.iter().for_each(|wall| {
            canvas
                .draw_line(
                    FPoint::new(wall.get_a().get_x() / 10.0, wall.get_a().get_y() / 10.0),
                    FPoint::new(wall.get_b().get_x() / 10.0, wall.get_b().get_y() / 10.0),
                )
                .unwrap();
        });

        // Draw robot
        canvas
            .draw_rect(FRect::new(
                (robot.get_position().get_x() - robot.get_radius()) / 10.0,
                (robot.get_position().get_y() - robot.get_radius()) / 10.0,
                robot.get_radius() * 2.0 / 10.0,
                robot.get_radius() * 2.0 / 10.0,
            ))
            .unwrap();
        let vector = direction_to_vector(robot.get_direction());
        let line_end = vector * robot.get_radius() + robot.get_position();
        canvas
            .draw_line(
                FPoint::new(
                    robot.get_position().get_x() / 10.0,
                    robot.get_position().get_y() / 10.0,
                ),
                FPoint::new(line_end.get_x() / 10.0, line_end.get_y() / 10.0),
            )
            .unwrap();

        // Draw Lidar
        if show_lidar {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            robot.lidar_scan(&room);
            robot
                .get_lidar()
                .iter()
                .enumerate()
                .for_each(|(num, distance)| {
                    let vector = direction_to_vector(num as f32 + robot.get_direction());
                    let colision_point = (vector * *distance + robot.get_position()) / 10.0;
                    canvas
                        .draw_line(
                            FPoint::new(
                                robot.get_position().get_x() / 10.0,
                                robot.get_position().get_y() / 10.0,
                            ),
                            FPoint::new(colision_point.get_x(), colision_point.get_y()),
                        )
                        .unwrap()
                });
        }

        // robot.rotate(&rotation);
        // robot.moving(&direction);
        robot.run(&room);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
