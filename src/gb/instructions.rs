use gb::cpu::Cpu;
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]

// will hopefully look cleaner than individual functions when executing an instruction
pub fn exec_ins(cpu: &mut Cpu, file_buf: &Vec<u8>, ins: u8) {
    println!("Executing instruction 0x{:x}", ins);

    match ins {
        0x00    =>  { // NOP 1 4

        }
        0x01    =>  { // LD (BC),d16 3 12
            cpu.reg_b = file_buf[(cpu.reg_pc+2) as usize];
            cpu.reg_c = file_buf[(cpu.reg_pc+1) as usize];
        }
        0x02    =>  { // LD (BC),A 1 8
            cpu.reg_c = cpu.reg_a;
            cpu.reg_b = 0b00000000u8;
        }/*
        0x03    =>  { // INC BC 1 8

        }
        0x04    =>  { // INC B 1 4

        }
        0x05    =>  {

        }
        0x06    =>  { //

        }
        0x07    =>  { //

        }
        0x08    =>  { //

        }
        0x09    =>  { //

        }
        0x0A    =>  { //

        }
        0x0B    =>  { //

        }
        0x0C    =>  { //

        }
        0x0D    =>  { //

        }
        0x0E    =>  { //

        }
        0x0F    =>  { //

        }*/
        0xC3    =>  { // JP a16 3 16
            cpu.reg_pc = de_endian(file_buf[(cpu.reg_pc+1) as usize], file_buf[(cpu.reg_pc+2) as usize]);
        }
        0xD3|0xDB|0xDD|0xE3|0xE4|0xEB|0xEC|0xED|0xF4|0xFC|0xFD => { // unsupported instructions
            println!{"Instruction {} is not supported by the GameBoy's CPU", ins};
        }
        _       =>  println!("Unrecognized/unimplemented instruction: {}", ins),
    }
}

fn de_endian(first: u8, second: u8) -> u16 {
    let bit_data: u16 = ((second * 128) + first) as u16;
    bit_data
}
