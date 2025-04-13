extern crate sdl3;

use crate::float2::Float2;
use crate::line::Line;
use crate::robot::Robot;
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
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        room.iter().for_each(|wall| {
            canvas
                .draw_line(
                    FPoint::new(wall.get_a().get_x() / 10.0, wall.get_a().get_y() / 10.0),
                    FPoint::new(wall.get_b().get_x() / 10.0, wall.get_b().get_y() / 10.0),
                )
                .unwrap();
        });

        canvas
            .draw_rect(FRect::new(
                (robot.get_position().get_x() - robot.get_radius()) / 10.0,
                (robot.get_position().get_y() - robot.get_radius()) / 10.0,
                robot.get_radius() * 2.0 / 10.0,
                robot.get_radius() * 2.0 / 10.0,
            ))
            .unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
