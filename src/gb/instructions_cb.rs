use gb::cpu::Cpu;
use gb::memory::Memory;
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]

pub fn exec_ins_cb(cpu: &mut Cpu, memory: &mut Memory, file_buf: &Vec<u8>, ins: u8) {
    println!("Executing instruction 0xCB{:02X}", ins);

    match ins {
        /*
        0x00    => { //

        }
        0x01    => { //

        }
        0x02    => { //

        }
        0x03    => { //

        }
        0x04    => { //

        }
        0x05    => { //

        }
        0x06    => { //

        }
        0x07    => { //

        }
        0x08    => { //

        }
        0x09    => { //

        }
        0x0A    => { //

        }
        0x0B    => { //

        }
        0x0C    => { //

        }
        0x0D    => { //

        }
        0x0E    => { //

        }
        0x0F    => { //

        }
        0x10    => { //

        }
        0x11    => { //

        }
        0x12    => { //

        }
        0x13    => { //

        }
        0x14    => { //

        }
        0x15    => { //

        }
        0x16    => { //

        }
        0x17    => { //

        }
        0x18    => { //

        }
        0x19    => { //

        }
        0x1A    => { //

        }
        0x1B    => { //

        }
        0x1C    => { //

        }
        0x1D    => { //

        }
        0x1E    => { //

        }
        0x1F    => { //

        }
        0x20    => { //

        }
        0x21    => { //

        }
        0x22    => { //

        }
        0x23    => { //

        }
        0x24    => { //

        }
        0x25    => { //

        }
        0x26    => { //

        }
        0x27    => { //

        }
        0x28    => { //

        }
        0x29    => { //

        }
        0x2A    => { //

        }
        0x2B    => { //

        }
        0x2C    => { //

        }
        0x2D    => { //

        }
        0x2E    => { //

        }
        0x2F    => { //

        }
        0x30    => { //

        }
        0x31    => { //

        }
        0x32    => { //

        }
        0x33    => { //

        }
        0x34    => { //

        }
        0x35    => { //

        }
        0x36    => { //

        }
        0x37    => { //

        }
        0x38    => { //

        }
        0x39    => { //

        }
        0x3A    => { //

        }
        0x3B    => { //

        }
        0x3C    => { //

        }
        0x3D    => { //

        }
        0x3E    => { //

        }
        0x3F    => { //

        }
		*/
		//	this stuff probably doesn't pass the register correctly
        0x40    => { //BIT b,B 2 8
			let reg: u8	= cpu.reg_b;
			bit_br(cpu, file_buf, reg);
		}
        0x41    => { //BIT b,C 2 8
			let reg: u8	= cpu.reg_c;
			bit_br(cpu, file_buf, reg);
		}
        0x42    => { //BIT b,D 2 8
			let reg: u8	= cpu.reg_d;
			bit_br(cpu, file_buf, reg);
		}
        0x43    => { //BIT b,E 2 8
			let reg: u8	= cpu.reg_e;
			bit_br(cpu, file_buf, reg);
		}
        0x44    => { //BIT b,H 2 8
			let reg: u8	= cpu.reg_h;
			bit_br(cpu, file_buf, reg);
		}
        0x45    => { //BIT b,L 2 8
			let reg: u8	= cpu.reg_l;
			bit_br(cpu, file_buf, reg);
		}
        0x46    => { //BIT b,(HL) 2 16

		}
        0x47    => { //BIT b,A 2 8
			let reg: u8	= cpu.reg_a;
			bit_br(cpu, file_buf, reg);
		}
		/*
        0x48    => { //

        }
        0x49    => { //

        }
        0x4A    => { //

        }
        0x4B    => { //

        }
        0x4C    => { //

        }
        0x4D    => { //

        }
        0x4E    => { //

        }
        0x4F    => { //

        }
        0x50    => { //

        }
        0x51    => { //

        }
        0x52    => { //

        }
        0x53    => { //

        }
        0x54    => { //

        }
        0x55    => { //

        }
        0x56    => { //

        }
        0x57    => { //

        }
        0x58    => { //

        }
        0x59    => { //

        }
        0x5A    => { //

        }
        0x5B    => { //

        }
        0x5C    => { //

        }
        0x5D    => { //

        }
        0x5E    => { //

        }
        0x5F    => { //

        }
        0x60    => { //

        }
        0x61    => { //

        }
        0x62    => { //

        }
        0x63    => { //

        }
        0x64    => { //

        }
        0x65    => { //

        }
        0x66    => { //

        }
        0x67    => { //

        }
        0x68    => { //

        }
        0x69    => { //

        }
        0x6A    => { //

        }
        0x6B    => { //

        }
        0x6C    => { //

        }
        0x6D    => { //

        }
        0x6E    => { //

        }
        0x6F    => { //

        }
        0x70    => { //

        }
        0x71    => { //

        }
        0x72    => { //

        }
        0x73    => { //

        }
        0x74    => { //

        }
        0x75    => { //

        }
        0x76    => { //

        }
        0x77    => { //

        }
        0x78    => { //

        }
        0x79    => { //

        }
        0x7A    => { //

        }
        0x7B    => { //

        }
        0x7C    => { //

        }
        0x7D    => { //

        }
        0x7E    => { //

        }
        0x7F    => { //

        }*/
        0x80    => /*RES 0,B 2 8*/ reset_bit_nr(&mut cpu.reg_b, 0, &mut cpu.reg_pc),
        0x81    => /*RES 0,C 2 8*/ reset_bit_nr(&mut cpu.reg_c, 0, &mut cpu.reg_pc),
        0x82    => /*RES 0,D 2 8*/ reset_bit_nr(&mut cpu.reg_d, 0, &mut cpu.reg_pc),
        0x83    => /*RES 0,E 2 8*/ reset_bit_nr(&mut cpu.reg_e, 0, &mut cpu.reg_pc),
        0x84    => /*RES 0,H 2 8*/ reset_bit_nr(&mut cpu.reg_h, 0, &mut cpu.reg_pc),
        0x85    => /*RES 0,L 2 8*/ reset_bit_nr(&mut cpu.reg_l, 0, &mut cpu.reg_pc),
        //0x86    => /*RES 0,(HL) 2 16*/
        0x87    => /*RES 0,A 2 8*/ reset_bit_nr(&mut cpu.reg_a, 0, &mut cpu.reg_pc),
        0x88    => /*RES 1,B 2 8*/ reset_bit_nr(&mut cpu.reg_b, 1, &mut cpu.reg_pc),
        0x89    => /*RES 1,C 2 8*/ reset_bit_nr(&mut cpu.reg_c, 1, &mut cpu.reg_pc),
        0x8A    => /*RES 1,D 2 8*/ reset_bit_nr(&mut cpu.reg_d, 1, &mut cpu.reg_pc),
        0x8B    => /*RES 1,E 2 8*/ reset_bit_nr(&mut cpu.reg_e, 1, &mut cpu.reg_pc),
        0x8C    => /*RES 1,H 2 8*/ reset_bit_nr(&mut cpu.reg_h, 1, &mut cpu.reg_pc),
        0x8D    => /*RES 1,L 2 8*/ reset_bit_nr(&mut cpu.reg_l, 1, &mut cpu.reg_pc),
        //0x8E    => /*
        0x8F    => /*RES 1,A 2 8*/ reset_bit_nr(&mut cpu.reg_a, 1, &mut cpu.reg_pc),
        0x90    => /*RES 2,B 2 8*/ reset_bit_nr(&mut cpu.reg_b, 2, &mut cpu.reg_pc),
        0x91    => /*RES 2,C 2 8*/ reset_bit_nr(&mut cpu.reg_c, 2, &mut cpu.reg_pc),
        0x92    => /*RES 2,D 2 8*/ reset_bit_nr(&mut cpu.reg_d, 2, &mut cpu.reg_pc),
        0x93    => /*RES 2,E 2 8*/ reset_bit_nr(&mut cpu.reg_e, 2, &mut cpu.reg_pc),
        0x94    => /*RES 2,H 2 8*/ reset_bit_nr(&mut cpu.reg_h, 2, &mut cpu.reg_pc),
        0x95    => /*RES 2,L 2 8*/ reset_bit_nr(&mut cpu.reg_l, 2, &mut cpu.reg_pc),
        //0x96    => /*
        0x97    => /*RES 2,A 2 8*/ reset_bit_nr(&mut cpu.reg_a, 2, &mut cpu.reg_pc),
        0x98    => /*RES 3,B 2 8*/ reset_bit_nr(&mut cpu.reg_b, 3, &mut cpu.reg_pc),
        0x99    => /*RES 3,C 2 8*/ reset_bit_nr(&mut cpu.reg_c, 3, &mut cpu.reg_pc),
        0x9A    => /*RES 3,D 2 8*/ reset_bit_nr(&mut cpu.reg_d, 3, &mut cpu.reg_pc),
        0x9B    => /*RES 3,E 2 8*/ reset_bit_nr(&mut cpu.reg_e, 3, &mut cpu.reg_pc),
        0x9C    => /*RES 3,H 2 8*/ reset_bit_nr(&mut cpu.reg_h, 3, &mut cpu.reg_pc),
        0x9D    => /*RES 3,L 2 8*/ reset_bit_nr(&mut cpu.reg_l, 3, &mut cpu.reg_pc),
        //0x9E    => /*
        0x9F    => /*RES 3,A 2 8*/ reset_bit_nr(&mut cpu.reg_a, 3, &mut cpu.reg_pc),
        0xA0    => /*RES 4,B 2 8*/ reset_bit_nr(&mut cpu.reg_b, 4, &mut cpu.reg_pc),
        0xA1    => /*RES 4,C 2 8*/ reset_bit_nr(&mut cpu.reg_c, 4, &mut cpu.reg_pc),
        0xA2    => /*RES 4,D 2 8*/ reset_bit_nr(&mut cpu.reg_d, 4, &mut cpu.reg_pc),
        0xA3    => /*RES 4,E 2 8*/ reset_bit_nr(&mut cpu.reg_e, 4, &mut cpu.reg_pc),
        0xA4    => /*RES 4,H 2 8*/ reset_bit_nr(&mut cpu.reg_h, 4, &mut cpu.reg_pc),
        0xA5    => /*RES 4,L 2 8*/ reset_bit_nr(&mut cpu.reg_l, 4, &mut cpu.reg_pc),
        //0xA6    => /*
        0xA7    => /*RES 4,A 2 8*/ reset_bit_nr(&mut cpu.reg_a, 4, &mut cpu.reg_pc),
        0xA8    => /*RES 5,B 2 8*/ reset_bit_nr(&mut cpu.reg_b, 5, &mut cpu.reg_pc),
        0xA9    => /*RES 5,C 2 8*/ reset_bit_nr(&mut cpu.reg_c, 5, &mut cpu.reg_pc),
        0xAA    => /*RES 5,D 2 8*/ reset_bit_nr(&mut cpu.reg_d, 5, &mut cpu.reg_pc),
        0xAB    => /*RES 5,E 2 8*/ reset_bit_nr(&mut cpu.reg_e, 5, &mut cpu.reg_pc),
        0xAC    => /*RES 5,H 2 8*/ reset_bit_nr(&mut cpu.reg_h, 5, &mut cpu.reg_pc),
        0xAD    => /*RES 5,L 2 8*/ reset_bit_nr(&mut cpu.reg_l, 5, &mut cpu.reg_pc),
        //0xAE    => /*
        0xAF    => /*RES 5,A 2 8*/ reset_bit_nr(&mut cpu.reg_a, 5, &mut cpu.reg_pc),
        0xB0    => /*RES 6,B 2 8*/ reset_bit_nr(&mut cpu.reg_b, 6, &mut cpu.reg_pc),
        0xB1    => /*RES 6,C 2 8*/ reset_bit_nr(&mut cpu.reg_c, 6, &mut cpu.reg_pc),
        0xB2    => /*RES 6,D 2 8*/ reset_bit_nr(&mut cpu.reg_d, 6, &mut cpu.reg_pc),
        0xB3    => /*RES 6,E 2 8*/ reset_bit_nr(&mut cpu.reg_e, 6, &mut cpu.reg_pc),
        0xB4    => /*RES 6,H 2 8*/ reset_bit_nr(&mut cpu.reg_h, 6, &mut cpu.reg_pc),
        0xB5    => /*RES 6,L 2 8*/ reset_bit_nr(&mut cpu.reg_l, 6, &mut cpu.reg_pc),
        //0xB6    => /*
        0xB7    => /*RES 6,A 2 8*/ reset_bit_nr(&mut cpu.reg_a, 6, &mut cpu.reg_pc),
        0xB8    => /*RES 7,B 2 8*/ reset_bit_nr(&mut cpu.reg_b, 7, &mut cpu.reg_pc),
        0xB9    => /*RES 7,C 2 8*/ reset_bit_nr(&mut cpu.reg_c, 7, &mut cpu.reg_pc),
        0xBA    => /*RES 7,D 2 8*/ reset_bit_nr(&mut cpu.reg_d, 7, &mut cpu.reg_pc),
        0xBB    => /*RES 7,E 2 8*/ reset_bit_nr(&mut cpu.reg_e, 7, &mut cpu.reg_pc),
        0xBC    => /*RES 7,H 2 8*/ reset_bit_nr(&mut cpu.reg_h, 7, &mut cpu.reg_pc),
        0xBD    => /*RES 7,L 2 8*/ reset_bit_nr(&mut cpu.reg_l, 7, &mut cpu.reg_pc),
        //0xBE    => /*
        0xBF    => /*RES 7,A 2 8*/ reset_bit_nr(&mut cpu.reg_a, 7, &mut cpu.reg_pc),
        0xC0    => /*SET 0,B 2 8*/ set_bit_nr(&mut cpu.reg_b, 0, &mut cpu.reg_pc),
        0xC1    => /*SET 0,C 2 8*/ set_bit_nr(&mut cpu.reg_c, 0, &mut cpu.reg_pc),
        0xC2    => /*SET 0,D 2 8*/ set_bit_nr(&mut cpu.reg_d, 0, &mut cpu.reg_pc),
        0xC3    => /*SET 0,E 2 8*/ set_bit_nr(&mut cpu.reg_e, 0, &mut cpu.reg_pc),
        0xC4    => /*SET 0,H 2 8*/ set_bit_nr(&mut cpu.reg_h, 0, &mut cpu.reg_pc),
        0xC5    => /*SET 0,L 2 8*/ set_bit_nr(&mut cpu.reg_l, 0, &mut cpu.reg_pc),
        //0xC6    => /*SET 0,(HL) 2 16*/
        0xC7    => /*SET 0,A 2 8*/ set_bit_nr(&mut cpu.reg_a, 0, &mut cpu.reg_pc),
        0xC8    => /*SET 1,B 2 8*/ set_bit_nr(&mut cpu.reg_b, 1, &mut cpu.reg_pc),
        0xC9    => /*SET 1,C 2 8*/ set_bit_nr(&mut cpu.reg_c, 1, &mut cpu.reg_pc),
        0xCA    => /*SET 1,D 2 8*/ set_bit_nr(&mut cpu.reg_d, 1, &mut cpu.reg_pc),
        0xCB    => /*SET 1,E 2 8*/ set_bit_nr(&mut cpu.reg_e, 1, &mut cpu.reg_pc),
        0xCC    => /*SET 1,H 2 8*/ set_bit_nr(&mut cpu.reg_h, 1, &mut cpu.reg_pc),
        0xCD    => /*SET 1,L 2 8*/ set_bit_nr(&mut cpu.reg_l, 1, &mut cpu.reg_pc),
        //0xCE    => /*
        0xCF    => /*SET 1,A 2 8*/ set_bit_nr(&mut cpu.reg_a, 1, &mut cpu.reg_pc),
        0xD0    => /*SET 2,B 2 8*/ set_bit_nr(&mut cpu.reg_b, 2, &mut cpu.reg_pc),
        0xD1    => /*SET 2,C 2 8*/ set_bit_nr(&mut cpu.reg_c, 2, &mut cpu.reg_pc),
        0xD2    => /*SET 2,D 2 8*/ set_bit_nr(&mut cpu.reg_d, 2, &mut cpu.reg_pc),
        0xD3    => /*SET 2,E 2 8*/ set_bit_nr(&mut cpu.reg_e, 2, &mut cpu.reg_pc),
        0xD4    => /*SET 2,H 2 8*/ set_bit_nr(&mut cpu.reg_h, 2, &mut cpu.reg_pc),
        0xD5    => /*SET 2,L 2 8*/ set_bit_nr(&mut cpu.reg_l, 2, &mut cpu.reg_pc),
        //0xD6    => /*
        0xD7    => /*SET 2,A 2 8*/ set_bit_nr(&mut cpu.reg_a, 2, &mut cpu.reg_pc),
        0xD8    => /*SET 3,B 2 8*/ set_bit_nr(&mut cpu.reg_b, 3, &mut cpu.reg_pc),
        0xD9    => /*SET 3,C 2 8*/ set_bit_nr(&mut cpu.reg_c, 3, &mut cpu.reg_pc),
        0xDA    => /*SET 3,D 2 8*/ set_bit_nr(&mut cpu.reg_d, 3, &mut cpu.reg_pc),
        0xDB    => /*SET 3,E 2 8*/ set_bit_nr(&mut cpu.reg_e, 3, &mut cpu.reg_pc),
        0xDC    => /*SET 3,H 2 8*/ set_bit_nr(&mut cpu.reg_h, 3, &mut cpu.reg_pc),
        0xDD    => /*SET 3,L 2 8*/ set_bit_nr(&mut cpu.reg_l, 3, &mut cpu.reg_pc),
        //0xDE    => /*
        0xDF    => /*SET 3,A 2 8*/ set_bit_nr(&mut cpu.reg_a, 3, &mut cpu.reg_pc),
        0xE0    => /*SET 4,B 2 8*/ set_bit_nr(&mut cpu.reg_b, 4, &mut cpu.reg_pc),
        0xE1    => /*SET 4,C 2 8*/ set_bit_nr(&mut cpu.reg_c, 4, &mut cpu.reg_pc),
        0xE2    => /*SET 4,D 2 8*/ set_bit_nr(&mut cpu.reg_d, 4, &mut cpu.reg_pc),
        0xE3    => /*SET 4,E 2 8*/ set_bit_nr(&mut cpu.reg_e, 4, &mut cpu.reg_pc),
        0xE4    => /*SET 4,H 2 8*/ set_bit_nr(&mut cpu.reg_h, 4, &mut cpu.reg_pc),
        0xE5    => /*SET 4,L 2 8*/ set_bit_nr(&mut cpu.reg_l, 4, &mut cpu.reg_pc),
        //0xE6    => /*
        0xE7    => /*SET 4,A 2 8*/ set_bit_nr(&mut cpu.reg_a, 4, &mut cpu.reg_pc),
        0xE8    => /*SET 5,B 2 8*/ set_bit_nr(&mut cpu.reg_b, 5, &mut cpu.reg_pc),
        0xE9    => /*SET 5,C 2 8*/ set_bit_nr(&mut cpu.reg_c, 5, &mut cpu.reg_pc),
        0xEA    => /*SET 5,D 2 8*/ set_bit_nr(&mut cpu.reg_d, 5, &mut cpu.reg_pc),
        0xEB    => /*SET 5,E 2 8*/ set_bit_nr(&mut cpu.reg_e, 5, &mut cpu.reg_pc),
        0xEC    => /*SET 5,H 2 8*/ set_bit_nr(&mut cpu.reg_h, 5, &mut cpu.reg_pc),
        0xED    => /*SET 5,L 2 8*/ set_bit_nr(&mut cpu.reg_l, 5, &mut cpu.reg_pc),
        //0xEE    => /*
        0xEF    => /*SET 5,A 2 8*/ set_bit_nr(&mut cpu.reg_a, 5, &mut cpu.reg_pc),
        0xF0    => /*SET 6,B 2 8*/ set_bit_nr(&mut cpu.reg_b, 6, &mut cpu.reg_pc),
        0xF1    => /*SET 6,C 2 8*/ set_bit_nr(&mut cpu.reg_c, 6, &mut cpu.reg_pc),
        0xF2    => /*SET 6,D 2 8*/ set_bit_nr(&mut cpu.reg_d, 6, &mut cpu.reg_pc),
        0xF3    => /*SET 6,E 2 8*/ set_bit_nr(&mut cpu.reg_e, 6, &mut cpu.reg_pc),
        0xF4    => /*SET 6,H 2 8*/ set_bit_nr(&mut cpu.reg_h, 6, &mut cpu.reg_pc),
        0xF5    => /*SET 6,L 2 8*/ set_bit_nr(&mut cpu.reg_l, 6, &mut cpu.reg_pc),
        //0xF6    => /*
        0xF7    => /*SET 6,A 2 8*/ set_bit_nr(&mut cpu.reg_a, 6, &mut cpu.reg_pc),
        0xF8    => /*SET 7,B 2 8*/ set_bit_nr(&mut cpu.reg_b, 7, &mut cpu.reg_pc),
        0xF9    => /*SET 7,C 2 8*/ set_bit_nr(&mut cpu.reg_c, 7, &mut cpu.reg_pc),
        0xFA    => /*SET 7,D 2 8*/ set_bit_nr(&mut cpu.reg_d, 7, &mut cpu.reg_pc),
        0xFB    => /*SET 7,E 2 8*/ set_bit_nr(&mut cpu.reg_e, 7, &mut cpu.reg_pc),
        0xFC    => /*SET 7,H 2 8*/ set_bit_nr(&mut cpu.reg_h, 7, &mut cpu.reg_pc),
        0xFD    => /*SET 7,L 2 8*/ set_bit_nr(&mut cpu.reg_l, 7, &mut cpu.reg_pc),
        //0xFE    => /*
        0xFF    => /*SET 7,A 2 8*/ set_bit_nr(&mut cpu.reg_a, 7, &mut cpu.reg_pc),
        _       => {
            println!("Unrecognized/unimplemented instruction: 0xCB{:x}", ins);
            cpu.cont    = false;
        }
    }
}

fn set_bit_nr(reg: &mut u8, n: u8, reg_pc: &mut u16) {
    if !get_bit_at_8(*reg, n) {
        *reg += 2^n;
    }
    *reg_pc += 2;
}
fn reset_bit_nr(reg: &mut u8, n: u8, reg_pc: &mut u16) {
    if get_bit_at_8(*reg, n) {
        *reg -= 2^n;
    }
    *reg_pc += 2;
}

fn get_bit_at_16(input: u16, n: u8) -> bool {
    if n < 16 {
        input & (1 << n) != 0
    } else {
        false
    }
}
fn get_bit_at_8(input: u8, n: u8) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        false
    }
}

fn bit_br(cpu: &mut Cpu, file_buf: &Vec<u8>, reg: u8) {
	if !get_bit_at_8(file_buf[(cpu.reg_pc+1) as usize], reg) {
		cpu.set_z();
	}
	cpu.reset_n();
	cpu.set_h();

	cpu.reg_pc	+= 2;
}
