extern crate sdl2;

use other::emu::Emu;
use other::sprite::Sprite;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

/*
0	=> 0
1	=> 255
2	=> 184
3	=> 104
*/

pub fn render(emu: &mut Emu) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

	let scale: u32	= 5;

    let window = video_subsystem.window("RustBoy", 160*scale, 144*scale)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer	= window.renderer().build().unwrap();

    // let mut texture	= renderer.create_texture_streaming(PixelFormatEnum::RGB24, 256, 256).unwrap();
    // // Create a red-green gradient
    // texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
    //     for y in 0..256 {
    //         for x in 0..256 {
    //             let offset = y*pitch + x*3;
    //             buffer[offset + 0] = x as u8;
    //             buffer[offset + 1] = y as u8;
    //             buffer[offset + 2] = 0;
    //         }
    //     }
    // }).unwrap();

	let mut texture2	= renderer.create_texture_streaming(PixelFormatEnum::RGB24, 8*scale, 8*scale).unwrap();

	texture2.with_lock(None, |buffer: &mut [u8], pitch: usize| {
		for y in 0..8*scale {
			for x in 0..8*scale {
				let offset	= (y*pitch as u32 + x*3) as usize;
                buffer[offset + 0] = 104 as u8;
                buffer[offset + 1] = 104 as u8;
                buffer[offset + 2] = 104 as u8;
			}
		}
	}).unwrap();

    renderer.clear();
    // renderer.copy(&texture, None, Some(Rect::new(100, 100, 256, 256))).unwrap();
    // renderer.copy_ex(&texture, None, Some(Rect::new(450, 100, 256, 256)), 30.0, None, false, false).unwrap();

	renderer.copy(&texture2, None, Some(Rect::new(0, 0, 8*scale, 8*scale))).unwrap();

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
        let spr	= emu.exec();
		print_sprite(&spr, &mut renderer, scale);
    }
}

fn print_sprite(spr: &Sprite, renderer: &mut sdl2::render::Renderer, scale: u32) {
	let mut texture	= renderer.create_texture_streaming(PixelFormatEnum::RGB24, 8*scale, 8*scale).unwrap();

	texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
		for y in 0..8*scale {
			for x in 0..8*scale {
				let offset	= (y*pitch as u32 + x*3) as usize;
                buffer[offset + 0] = spr.pixels[(y / scale) as usize][(x / scale) as usize];
                buffer[offset + 1] = spr.pixels[(y / scale) as usize][(x / scale) as usize];
                buffer[offset + 2] = spr.pixels[(y / scale) as usize][(x / scale) as usize];
			}
		}
	}).unwrap();

	renderer.copy(&texture, None, Some(Rect::new(1000, 1000, 8*scale, 8*scale))).unwrap();
	renderer.present();
}
