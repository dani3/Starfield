use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;

use std::{thread, time};

const SCREEN_WIDTH: isize  = 1400;
const SCREEN_HEIGHT: isize = 800;

mod star;

pub use self::star::Star;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Starfield", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    canvas.clear();
    canvas.present();

    let mut stars = Vec::new();
    for _i in 0 .. 500 {
        stars.push(Star::new(SCREEN_HEIGHT as usize, SCREEN_WIDTH as usize));
    }

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let cursor = sdl2::mouse::MouseState::new(&event_pump);

        canvas.set_draw_color(Color::RGB(14, 9, 34));
        canvas.present();
        canvas.clear();

        for star in stars.iter_mut() {
            let pos = star.update(cursor.x());
            canvas.set_draw_color(Color::RGB(pos.4, pos.4, pos.4));
            let _ = canvas.draw_line(Point::new(pos.0 as i32, pos.1 as i32), Point::new(pos.2 as i32, pos.3 as i32));
        }

        canvas.present();

        thread::sleep(time::Duration::from_millis(10));
    }
}
