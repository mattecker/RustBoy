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

    while cpu.cont {
        gb::instructions::exec_ins(&mut cpu, &mut memory, &file_buf);
    }
}

fn load_rom<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file        = fs::File::open(path.as_ref()).unwrap();
    let mut file_buf    = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
