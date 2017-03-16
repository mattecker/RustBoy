mod gb;

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

    let mut cpu     = gb::cpu::Cpu::new();
    let mut memory  = gb::memory::Memory::new();
    memory.initialize();
	
    let mut ins     = file_buf[cpu.reg_pc as usize]; // keeps track of current instruction

	let mut ins_count	= 0;

    while cpu.cont {
		cpu.print();

        if ins == 0xCB {
            ins = file_buf[(cpu.reg_pc + 1) as usize];
            gb::instructions_cb::exec_ins_cb(&mut cpu, &mut memory, &file_buf, ins);
        } else {
            gb::instructions::exec_ins(&mut cpu, &mut memory, &file_buf, ins);
        }

		ins_count	+= 1;
		if ins_count == 200 {
			cpu.cont	= false;
		}

        ins = file_buf[cpu.reg_pc as usize];

		if cpu.interrupt_count != 0 {
			cpu.interrupt_handler();
		}
    }
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file        = fs::File::open(path.as_ref()).unwrap();
    let mut file_buf    = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
