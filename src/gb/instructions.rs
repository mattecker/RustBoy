use gb::cpu::Cpu;
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]



// LD (BC),A - 8
pub fn ins02(cpu: &mut Cpu) {
    cpu.reg_c = cpu.reg_a;
    cpu.reg_b = 0b00000000u8;
}


// will hopefully look cleaner than individual functions when executing an instruction
pub fn exec_ins(cpu: &mut Cpu, file_buf: &Vec<u8>, ins: u8) {
    match ins {
        0x00    =>  {
            println!("Executing 0x00"); // debug

        }
        0x01    =>  {
            println!("Executing 0x01"); // debug

        }
        0x02    =>  {
            println!("Executing 0x02"); // debug
            cpu.reg_c = cpu.reg_a;
            cpu.reg_b = 0b00000000u8;
        }
        0x03    =>  {
            println!("Executing 0x03"); // debug

        }
        0x04    =>  {
            println!("Executing 0x04"); // debug

        }
        0x05    =>  {
            println!("Executing 0x05"); // debug

        }
        /*
        0x06    =>  ,
        0x07    =>  ,
        0x08    =>  ,
        0x09    =>  ,
        0x0A    =>  ,
        0x0B    =>  ,
        0x0C    =>  ,
        0x0C    =>  ,
        0x0D    =>  ,
        0x0E    =>  ,
        0x0F    =>  */
        0xC3    =>  { // JP a16
            println!("Executing 0xC3"); // debug
            cpu.reg_pc = de_endian(file_buf[(cpu.reg_pc+1) as usize], file_buf[(cpu.reg_pc+2) as usize]);
        }

        0xD3|0xDB|0xDD|0xE3|0xE4|0xEB|0xEC|0xED|0xF4|0xFC|0xFD => {
            println!{"Instruction {} is not supported by the GameBoy's CPU", ins};
        }
        _       =>  println!("Unrecognized/unimplemented instruction: {}", ins),
    }
}

fn de_endian(first: u8, second: u8) -> u16 {
    let bit_data: u16 = ((second * 128) + first) as u16;
    bit_data
}
