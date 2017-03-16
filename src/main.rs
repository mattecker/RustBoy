mod gb;
mod other;

#[macro_use]
extern crate sdl2;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

fn main() {
    let 	file_name	= std::env::args().nth(1).unwrap(); // unsafe, need error handling
    let     file_buf    = load_rom(file_name);
    gb::get_info::ret_info(&file_buf);

	let mut emu	= other::emu::Emu::new(file_buf);

	other::display::render(&mut emu);
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file        = fs::File::open(path.as_ref()).unwrap();
    let mut file_buf    = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
