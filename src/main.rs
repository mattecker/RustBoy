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
    let 	file_name	= std::env::args().nth(1).unwrap();
    let     file_buf    = load_rom(file_name);

    gb::get_info::ret_info(&file_buf);

    let mut cpu     = gb::cpu::Cpu::new();
    let mut memory  = gb::memory::Memory::new();
    println!("Program Counter: {}", cpu.reg_pc);

    // debug code
    cpu.reg_a = 0b00000010u8;
    println!("reg_a = {}", cpu.reg_a);
    println!("reg_c = {}", cpu.reg_c);
    let ins = 0x02u8;
    gb::instructions::exec_ins(&mut cpu, &mut memory, &file_buf, ins);
    println!("reg_c = {}", cpu.reg_c);

    /* endian swap test (it works)
    cpu.reg_pc = 0xABCDu16;
    println!("Program Counter: {}", cpu.reg_pc);
    cpu.reg_pc = u16::from_be(cpu.reg_pc);
    println!("Program Counter swapped endian: {}", cpu.reg_pc);*/
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file        = fs::File::open(path.as_ref()).unwrap();
    let mut file_buf    = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
