mod gb;

#[allow(unused_imports)]
use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]
fn main() {
    let 	file_name	= std::env::args().nth(1).unwrap(); // unsafe, need error handling
    let     file_buf    = load_rom(file_name);
    gb::get_info::ret_info(&file_buf);

    let mut cpu     = gb::cpu::Cpu::new();
    let mut memory  = gb::memory::Memory::new();
    memory.initialize();
    let mut ins     = file_buf[cpu.reg_pc as usize]; // keeps track of current instruction

    while cpu.cont {
        if ins == 0xCB {
            cpu.reg_pc  += 1;
            ins = file_buf[cpu.reg_pc as usize];
            gb::instructions_cb::exec_ins_cb(&mut cpu, &mut memory, &file_buf, ins);
        } else {
            gb::instructions::exec_ins(&mut cpu, &mut memory, &file_buf, ins);
        }

        ins = file_buf[cpu.reg_pc as usize];
    }

    println!("Register PC: 0b{:b}", cpu.reg_pc);
    println!("Bit 0: {}", get_bit_at(cpu.reg_pc, 0));
    println!("Bit 1: {}", get_bit_at(cpu.reg_pc, 1));
    println!("Bit 2: {}", get_bit_at(cpu.reg_pc, 2));
    println!("Bit 3: {}", get_bit_at(cpu.reg_pc, 3));

}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file        = fs::File::open(path.as_ref()).unwrap();
    let mut file_buf    = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}

fn get_bit_at(input: u16, n: u8) -> bool {
    if n < 16 {
        input & (1 << n) != 0
    } else {
        false
    }
}
