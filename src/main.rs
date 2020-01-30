extern crate sdl2;

pub mod video;

use sdl2::pixels::PixelFormatEnum;
//use sdl2::rect::Rect;
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

    let mut cg32 = video::CGFB::new();

    &cg32.upload_font();
    let mut cur_page: usize = 0;
    &cg32.fb_set_page_res(0, FB_WIDTH as u16, FB_HEIGHT as u16);
    &cg32.fb_set_page_res(1, FB_WIDTH as u16, FB_HEIGHT as u16);
    &cg32.fb_set_page_res(2, FB_WIDTH as u16, FB_HEIGHT as u16);
    let mut fb: Box<[u32]>;
    let sdl_texture_creator = sdl_canvas.texture_creator();
    let mut fb_texture = sdl_texture_creator.create_texture_streaming(PixelFormatEnum::ARGB32, FB_WIDTH, FB_HEIGHT).unwrap();

    let hello_text = "Praxis PX-1".as_bytes();

    for i in 0..20 {
        if i < hello_text.len() {
            let letter: u16 = hello_text[i] as u16;
            &cg32.fb_print_char(cur_page, i as u8, 0, letter);
        }
    }

    cur_page = 0;
    &cg32.fb_change_page_type(1, video::PageType::Graphics);
    &cg32.fb_fill_50_gradient(1);
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
        fb = cg32.fb_present(cur_page);
        fb_texture.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            let mut size: usize = (FB_HEIGHT * FB_WIDTH) as usize;
            if size < buffer.len() {
                size = buffer.len();
            }
            for i in 0..size {
                let offset: usize = (i * 4) as usize;
                if offset < fb.len() {
                    buffer[offset + 0] = ((fb[i as usize] >>  0) & 0xFF) as u8;
                    //println!("fb_texture: {}", ((fb[i as usize] >>  0) & 0xFF));
                    buffer[offset + 1] = ((fb[i as usize] >>  8) & 0xFF) as u8;
                    buffer[offset + 2] = ((fb[i as usize] >> 16) & 0xFF) as u8;
                    buffer[offset + 3] = ((fb[i as usize] >> 24) & 0xFF) as u8;
                }

            } 
        }).unwrap();

        sdl_canvas.copy(&fb_texture, None, None).unwrap();

        // The rest of the game loop goes here...
        sdl_canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
