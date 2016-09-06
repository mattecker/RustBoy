mod gb;

#[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;
//use gb::get_info;

#[allow(dead_code)]
#[derive(Default)]
struct CPU {
    reg_a:  u8,	// accumulator
    reg_b:  u8,
    reg_c:  u8,
    reg_d:  u8,
    reg_e:  u8,
    reg_f:  u8,	// flags
    reg_h:  u8,
    reg_l:  u8,

    reg_sp: u16,// stack pointer
    reg_pc: u16	// program counter
}

#[allow(dead_code)]
impl CPU {
    fn new() -> CPU {
        CPU::default()
    }
}

#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    let 	file_name	= std::env::args().nth(1).unwrap();
    let     file_buf    = load_rom(file_name);

    gb::get_info::ret_info(&file_buf);


	// println!("{}", file_name);
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file        = fs::File::open(path.as_ref()).unwrap();
    let mut file_buf    = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}