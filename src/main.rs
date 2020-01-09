extern crate sdl2;

pub mod video;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() 
{
    const FB_WIDTH: u32 = 640;
    const FB_HEIGHT: u32 = 480;

    let scale = 1;

    let window_width = FB_WIDTH * scale;
    let window_height = FB_HEIGHT * scale;

    let sdl = sdl2::init().unwrap();
    let sdl_video = sdl.video().unwrap();
    let sdl_window = sdl_video.window("PX1 Emulator", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut sdl_canvas = sdl_window.into_canvas().build().unwrap();

    let mut sdl_event_pump = sdl.event_pump().unwrap();

    'running: loop {
        sdl_canvas.clear();
        for event in sdl_event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        sdl_canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
