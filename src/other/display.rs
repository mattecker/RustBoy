extern crate sdl2;

use other::emu::Emu;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn render(emu: &mut Emu) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let mut texture = renderer.create_texture_streaming(
        PixelFormatEnum::RGB24, 256, 256).unwrap();
    // Create a red-green gradient
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..256 {
            for x in 0..256 {
                let offset = y*pitch + x*3;
                buffer[offset + 0] = x as u8;
                buffer[offset + 1] = y as u8;
                buffer[offset + 2] = 0;
            }
        }
    }).unwrap();

    renderer.clear();
    renderer.copy(&texture, None, Some(Rect::new(100, 100, 256, 256))).unwrap();
    renderer.copy_ex(&texture, None,
        Some(Rect::new(450, 100, 256, 256)), 30.0, None, false, false).unwrap();
    renderer.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        emu.exec();
    }
}
