use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

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


    let mut	rom_name		= String::new();
    let mut	gameboy_color	= String::new();
    let mut	super_gameboy	= String::new();
    let mut	cartridge_type	= String::new();
    let mut	rom_size		= String::new();
    let mut	ram_size		= String::new();
    let mut	destination		= String::new();

    print!("ROM Name: ");
    for x in 308..323 {
    	rom_name.push(file_buf[x] as char);
    }
    println!("{}", rom_name);

    print!("ROM Size: ");
    
    match file_buf[328] {
    	0  => rom_size = "256Kbit".to_string(),
    	1  => rom_size = "512Kbit".to_string(),
    	2  => rom_size = "1Mbit".to_string(),
    	3  => rom_size = "2Mbit".to_string(),
    	4  => rom_size = "4Mbit".to_string(),
    	5  => rom_size = "8Mbit".to_string(),
    	6  => rom_size = "16Mbit".to_string(),
    	82 => rom_size = "9Mbit".to_string(),
    	83 => rom_size = "10Mbit".to_string(),
    	84 => rom_size = "12Mbit".to_string(),
    	_  => rom_size = "Unrecognized value, try blowing on the cartridge".to_string(),
    }

    println!("{}", rom_size);

    print!("RAM Size: ");

    match file_buf[329] {
    	0 => ram_size = "None".to_string(),
    	1 => ram_size = "16kBit".to_string(),
    	2 => ram_size = "64kBit".to_string(),
    	3 => ram_size = "256kBit".to_string(),
    	4 => ram_size = "1MBit".to_string(),
    	_ => ram_size = "Unrecognized value, try blowing on the cartridge".to_string(),
    }

    println!("{}", ram_size);


	// println!("{}", file_name);
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file        = fs::File::open(path.as_ref()).unwrap();
    let mut file_buf    = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}