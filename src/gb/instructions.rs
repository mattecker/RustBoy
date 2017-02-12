use gb::cpu::Cpu;
use gb::memory::Memory;
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]

// will hopefully look cleaner than individual functions when executing an instruction
// comment format: [name] [parameters (if any)] [byte length] [cycles] [flags affected (if any)]
pub fn exec_ins(cpu: &mut Cpu, memory: &mut Memory, file_buf: &Vec<u8>, ins: u8) {
    println!("cpu.reg_pc: 0x{:X}", cpu.reg_pc);
    if ins != 0x00 {
        println!("Executing instruction 0x{:02X}", ins);
    }

    match ins {
        0x00    => { // NOP 1 4
            cpu.reg_pc  += 1;
        }
        0x01    => { // LD (BC),d16 3 12
            cpu.reg_b   = file_buf[(cpu.reg_pc+2) as usize];
            cpu.reg_c   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 3;
        }
        0x02    => { // LD (BC),A 1 8
            let address = &mut memory.memory_array[cpu.get_reg_bc() as usize]; // still trying to figure out how this will work
            ld_8_test(address, cpu.reg_a, &mut cpu.reg_pc, 1, 8);
        }
        0x03    => { // INC BC 1 8
            let new_reg_bc: u16 = cpu.get_reg_bc() + 1;
            cpu.set_reg_bc(new_reg_bc);
            cpu.reg_pc  += 1;
        }
        0x04    => { // INC B 1 4 Z0H-
            cpu.reg_b   += 1;
            if cpu.reg_b == 0 {
                cpu.set_z();
            } else {
                cpu.reset_z();
            }
            cpu.reset_n();
            cpu.reg_pc  += 1;
        }
        0x05    => { // DEC B 1 4 Z1H-
            cpu.reg_b   -= 1;
            if cpu.reg_b == 0 {
                cpu.set_z();
            } else {
                cpu.reset_z();
            }
            cpu.set_n();
            cpu.reg_pc  += 1;
        }
        0x06    => /*LD B,d8 2 8*/ld_8_rd(&mut cpu.reg_b, &file_buf, &mut cpu.reg_pc),
        0x07    => { // RLCA 1 4 000C
            cpu.reset_z();
            cpu.reset_n();
            cpu.reset_h();
            cpu.reg_pc  += 1;
        }
        0x08    => { //LD (a16),SP 3 20
            memory.memory_array[((file_buf[(cpu.reg_pc+2) as usize] * 16) + file_buf[(cpu.reg_pc+1) as usize]) as usize] = cpu.reg_sp as u8;
            memory.memory_array[(((file_buf[(cpu.reg_pc+2) as usize] * 16) + file_buf[(cpu.reg_pc+1) as usize])+1) as usize] = (cpu.reg_sp >> 8) as u8;
            cpu.reg_pc  += 3;
        }
        0x09    => { // ADD HL,BC 1 8 -0HC

            cpu.reset_n();
            cpu.reg_pc  += 1;
        }
        0x0A    => { // LD A,(BC) 1 8
            let address = memory.memory_array[cpu.get_reg_bc() as usize];
            ld_8_test(&mut cpu.reg_a, address,&mut cpu.reg_pc, 1, 8);
        }
        0x0B    => { // DEC BC 1 8
            let new_reg_bc: u16 = cpu.get_reg_bc() - 1;
            cpu.set_reg_bc(new_reg_bc);
            cpu.reg_pc  += 1;
        }
        0x0C    => { // INC C 1 4 Z0H-
            cpu.reg_c   += 1;
            if cpu.reg_c == 0 {
                cpu.set_z();
            } else {
                cpu.reset_z();
            }
            cpu.reset_n();
            cpu.reg_pc  += 1;
        }
        0x0D    => { // DEC C 1 4 Z1H-
            cpu.reg_c   -= 1;
            if cpu.reg_c == 0 {
                cpu.set_z();
            } else {
                cpu.reset_z();
            }
            cpu.set_n();
            cpu.reg_pc  += 1;
        }
        0x0E    => /*LD C,d8 2 8*/ld_8_rd(&mut cpu.reg_c, &file_buf, &mut cpu.reg_pc),
        0x0F    => { // RRCA 1 4 000C
            cpu.reset_z();
            cpu.reset_n();
            cpu.reset_h();
            cpu.reg_pc  += 1;
        }
        0x10    => { // STOP 0 2 4

            cpu.reg_pc  += 2;
        }
        0x11    => { // LD DE,d16 3 12
            cpu.reg_d   = file_buf[(cpu.reg_pc+2) as usize];
            cpu.reg_e   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 3;
        }
        0x12    => { // LD (DE),A 1 8
            cpu.reg_e   = cpu.reg_a;
            cpu.reg_d   = 0b00000000u8;
            cpu.reg_pc  += 1;
        }
        0x13    => { // INC DE 1 8

            cpu.reg_pc  += 1;
        }
        0x14    => { // INC D 1 4 Z0H-
            cpu.reg_d   += 1;
            if cpu.reg_d == 0 {
                cpu.set_z();
            } else {
                cpu.reset_z();
            }
            cpu.reset_n();
            cpu.reg_pc  += 1;
        }
        0x15    => { // DEC D 1 4 Z1H-
            cpu.reg_d   -= 1;
            if cpu.reg_d == 0 {
                cpu.set_z();
            } else {
                cpu.reset_z();
            }
            cpu.set_n();
            cpu.reg_pc  += 1;
        }
        0x16    => /*LD D,d8 2 8*/ ld_8_rd(&mut cpu.reg_d, &file_buf, &mut cpu.reg_pc),
        0x17    => { // RLA 1 4 000C
            cpu.reset_z();
            cpu.reset_n();
            cpu.reset_h();
            cpu.reg_pc  += 1;
        }
        0x18    => { // JR r8 2 12
            cpu.reg_pc  += file_buf[(cpu.reg_pc+1) as usize] as u16;
        }
        0x19    => { // ADD HL,DE 1 8

            cpu.reg_pc  += 1;
        }
        0x1A    => { // LD A,(DE) 1 8
            cpu.reg_a   = memory.memory_array[cpu.get_reg_de() as usize];
            cpu.reg_pc  += 1;
        }
        0x1B    => { // DEC DE 1 8

            cpu.reg_pc  += 1;
        }
        0x1C    => { // INC E 1 4
            cpu.reg_e   += 1;
            cpu.reg_pc  += 1;
        }
        0x1D    => { // DEC E 1 4
            cpu.reg_e   -= 1;
            cpu.reg_pc  += 1;
        }
        0x1E    => /*LD E,d8 2 8*/ld_8_rd(&mut cpu.reg_e, &file_buf, &mut cpu.reg_pc),
        0x1F    => { // RRA 1 4

            cpu.reg_pc  += 1;
        }
        0x20    => { // JR NZ,r8 2 12/8

            cpu.reg_pc  += 2;
        }
        0x21    => { // LD HL,d16 3 12
            cpu.reg_h   = file_buf[(cpu.reg_pc+2) as usize];
            cpu.reg_l   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 3;
        }
        0x22    => { // LD (HL+),A 1 8

            cpu.reg_pc  += 1;
        }
        0x23    => { // INC HL 1 8

            cpu.reg_pc  += 1;
        }
        0x24    => { // INC H 1 4
            cpu.reg_h   += 1;
            cpu.reg_pc  += 1;
        }
        0x25    => { // DEC H 1 4
            cpu.reg_h   -= 1;
            cpu.reg_pc  += 1;
        }
        0x26    => /*LD H,d8 2 8*/ld_8_rd(&mut cpu.reg_h, &file_buf, &mut cpu.reg_pc),
        0x27    => { // DAA 1 4

            cpu.reg_pc  += 1;
        }
        0x28    => { // JR Z,r8 2 12/8
            if cpu.get_z() {
                cpu.reg_pc  += file_buf[(cpu.reg_pc+1) as usize] as u16;
            } else {
                cpu.reg_pc  += 2;
            }
        }
        0x29    => { // ADD HL,HL 1 8

            cpu.reg_pc  += 1;
        }
        0x2A    => { // LD A,(HL+) 1 8

            cpu.reg_pc  += 1;
        }
        0x2B    => { // DEC HL 1 8

            cpu.reg_pc  += 1;
        }
        0x2C    => { // INC L 1 4
            cpu.reg_l   += 1;
            cpu.reg_pc  += 1;
        }
        0x2D    => { // DEC L 1 4
            cpu.reg_l   -= 1;
            cpu.reg_pc  += 1;
        }
        0x2E    => /*LD L,d8 2 8*/ld_8_rd(&mut cpu.reg_l, &file_buf, &mut cpu.reg_pc),
        0x2F    => { // CPL 1 4

            cpu.reg_pc  += 1;
        }
        0x30    => { // JR NC,r8 2 12/8

        }
        0x31    => { // LD SP,d16 3 12
            cpu.reg_sp  = ((file_buf[(cpu.reg_pc+2) as usize] as u16 * 0x100) + file_buf[(cpu.reg_pc+1) as usize] as u16) as u16;
            cpu.reg_pc  += 3;
        }
        0x32    => { // LD (HL-),A 1 8

        }
        0x33    => { // INC SP 1 8
            cpu.reg_sp  += 1;
            cpu.reg_pc  += 1;
        }
        0x34    => { // INC (HL) 1 12

            cpu.reg_pc  += 1;
        }
        0x35    => { // DEC (HL) 1 12

            cpu.reg_pc  += 1;
        }
        0x36    => { // LD (HL),d8 2 12
            let data: u16   = file_buf[(cpu.reg_pc+1) as usize] as u16;
            cpu.set_reg_hl(data);
            cpu.reg_pc  += 2;
        }
        0x37    => { // SCF 1 4

            cpu.reg_pc  += 1;
        }
        0x38    => { // JR C,r8 2 12/8

        }
        0x39    => { // ADD HL,SP 1 8

            cpu.reg_pc  += 1;
        }
        0x3A    => { // LD A,(HL-) 1 8

            cpu.reg_pc  += 1;
        }
        0x3B    => { // DEC SP 1 8
            cpu.reg_sp  -= 1;
            cpu.reg_pc  += 1;
        }
        0x3C    => { // INC A 1 4
            cpu.reg_a   += 1;
            cpu.reg_pc  += 1;
        }
        0x3D    => { // DEC A 1 4
            cpu.reg_a   -= 1;
            cpu.reg_pc  += 1;
        }
        0x3E    => /*LD A,d8 2 8*/ld_8_rd(&mut cpu.reg_a, &file_buf, &mut cpu.reg_pc),
        0x3F    => { // CCF 1 4

            cpu.reg_pc  += 1;
        }
        0x40    => /*LD B,B 1 4*/ cpu.reg_pc  += 1,
        0x41    => /*LD B,C 1 4*/ ld_8_rr(&mut cpu.reg_b, cpu.reg_c, &mut cpu.reg_pc),
        0x42    => /*LD B,D 1 4*/ ld_8_rr(&mut cpu.reg_b, cpu.reg_d, &mut cpu.reg_pc),
        0x43    => /*LD B,E 1 4*/ ld_8_rr(&mut cpu.reg_b, cpu.reg_e, &mut cpu.reg_pc),
        0x44    => /*LD B,H 1 4*/ ld_8_rr(&mut cpu.reg_b, cpu.reg_h, &mut cpu.reg_pc),
        0x45    => /*LD B,L 1 4*/ ld_8_rr(&mut cpu.reg_b, cpu.reg_l, &mut cpu.reg_pc),
        0x46    => { // LD B,(HL) 1 8
            //cpu.reg_b   = ;
            cpu.reg_pc  += 1;
        }
        0x47    => /*LD B,A 1 4*/ ld_8_rr(&mut cpu.reg_b, cpu.reg_a, &mut cpu.reg_pc),
        0x48    => /*LD C,B 1 4*/ ld_8_rr(&mut cpu.reg_c, cpu.reg_b, &mut cpu.reg_pc),
        0x49    => /*LD C,C 1 4*/ cpu.reg_pc  += 1,
        0x4A    => /*LD C,D 1 4*/ ld_8_rr(&mut cpu.reg_c, cpu.reg_d, &mut cpu.reg_pc),
        0x4B    => /*LD C,E 1 4*/ ld_8_rr(&mut cpu.reg_c, cpu.reg_e, &mut cpu.reg_pc),
        0x4C    => /*LD C,H 1 4*/ ld_8_rr(&mut cpu.reg_c, cpu.reg_h, &mut cpu.reg_pc),
        0x4D    => /*LD C,L 1 4*/ ld_8_rr(&mut cpu.reg_c, cpu.reg_l, &mut cpu.reg_pc),
        0x4E    => { //LD C,(HL) 1 8
            //cpu.reg_c   = ;
            cpu.reg_pc  += 1;
        }
        0x4F    => /*LD C,A 1 4*/ ld_8_rr(&mut cpu.reg_c, cpu.reg_a, &mut cpu.reg_pc),
        0x50    => /*LD D,B 1 4*/ ld_8_rr(&mut cpu.reg_d, cpu.reg_b, &mut cpu.reg_pc),
        0x51    => /*LD D,C 1 4*/ ld_8_rr(&mut cpu.reg_d, cpu.reg_c, &mut cpu.reg_pc),
        0x52    => /*LD D,D 1 4*/ cpu.reg_pc    += 1,
        0x53    => /*LD D,E 1 4*/ ld_8_rr(&mut cpu.reg_d, cpu.reg_e, &mut cpu.reg_pc),
        0x54    => /*LD D,H 1 4*/ ld_8_rr(&mut cpu.reg_d, cpu.reg_h, &mut cpu.reg_pc),
        0x55    => /*LD D,L 1 4*/ ld_8_rr(&mut cpu.reg_d, cpu.reg_l, &mut cpu.reg_pc),
        0x56    => { // LD D,(HL) 1 8
            //cpu.reg_d   = ;
            cpu.reg_pc  += 1;
        }
        0x57    => { // LD D,A 1 4
            cpu.reg_d   = cpu.reg_a;
            cpu.reg_pc  += 1;
        }
        0x58    => { // LD E,B 1 4
            cpu.reg_e   = cpu.reg_b;
            cpu.reg_pc  += 1;
        }
        0x59    => { // LD E,C 1 4
            cpu.reg_e   = cpu.reg_c;
            cpu.reg_pc  += 1;
        }
        0x5A    => { // LD E,D 1 4
            cpu.reg_e   = cpu.reg_d;
            cpu.reg_pc  += 1;
        }
        0x5B    => { // LD E,E 1 4
            //cpu.reg_e   = cpu.reg_e;
            cpu.reg_pc  += 1;
        }
        0x5C    => { // LD E,H 1 4
            cpu.reg_e   = cpu.reg_h;
            cpu.reg_pc  += 1;
        }
        0x5D    => { // LD E,L 1 4
            cpu.reg_e   = cpu.reg_l;
            cpu.reg_pc  += 1;
        }
        0x5E    => { // LD E,(HL) 1 8
            cpu.reg_e   = memory.memory_array[cpu.get_reg_hl() as usize];
            cpu.reg_pc  += 1;
        }
        0x5F    => { // LD E,A 1 4
            cpu.reg_e   = cpu.reg_a;
            cpu.reg_pc  += 1;
        }
        0x60    => { // LD H,B 1 4
            cpu.reg_h   = cpu.reg_b;
            cpu.reg_pc  += 1;
        }
        0x61    => { // LD H,C 1 4
            cpu.reg_h   = cpu.reg_c;
            cpu.reg_pc  += 1;
        }
        0x62    => { // LD H,D 1 4
            cpu.reg_h   = cpu.reg_d;
            cpu.reg_pc  += 1;
        }
        0x63    => { // LD H,E 1 4
            cpu.reg_h   = cpu.reg_e;
            cpu.reg_pc  += 1;
        }
        0x64    => { // LD H,H 1 4
            //cpu.reg_h   = cpu.reg_h;
            cpu.reg_pc  += 1;
        }
        0x65    => { // LD H,L 1 4
            cpu.reg_h   = cpu.reg_l;
            cpu.reg_pc  += 1;
        }
        0x66    => { // LD H,(HL) 1 8
            cpu.reg_h   = memory.memory_array[cpu.get_reg_hl() as usize];
            cpu.reg_pc  += 1;
        }
        0x67    => { // LD H,A 1 4
            cpu.reg_h   = cpu.reg_a;
            cpu.reg_pc  += 1;
        }
        0x68    => { // LD L,B 1 4
            cpu.reg_l   = cpu.reg_b;
            cpu.reg_pc  += 1;
        }
        0x69    => { // LD L,C 1 4
            cpu.reg_l   = cpu.reg_c;
            cpu.reg_pc  += 1;
        }
        0x6A    => { // LD L,D 1 4
            cpu.reg_l   = cpu.reg_d;
            cpu.reg_pc  += 1;
        }
        0x6B    => { // LD L,E 1 4
            cpu.reg_l   = cpu.reg_e;
            cpu.reg_pc  += 1;
        }
        0x6C    => { // LD L,H 1 4
            cpu.reg_l   = cpu.reg_h;
            cpu.reg_pc  += 1;
        }
        0x6D    => { // LD L,L 1 4
            //cpu.reg_l   = cpu.reg_l;
            cpu.reg_pc  += 1;
        }
        0x6E    => { // LD L,(HL) 1 8
            cpu.reg_l   = memory.memory_array[cpu.get_reg_hl() as usize];
            cpu.reg_pc  += 1;
        }
        0x6F    => { // LD L,A 1 4
            cpu.reg_l   = cpu.reg_a;
            cpu.reg_pc  += 1;
        }
        0x70    => { // LD (HL),B 1 8
            memory.memory_array[cpu.get_reg_hl() as usize]  = cpu.reg_b;
            cpu.reg_pc  += 1;
        }
        0x71    => { // LD (HL),C 1 8
            memory.memory_array[cpu.get_reg_hl() as usize]  = cpu.reg_c;
            cpu.reg_pc  += 1;
        }
        0x72    => { // LD (HL),D 1 8
            memory.memory_array[cpu.get_reg_hl() as usize]  = cpu.reg_d;
            cpu.reg_pc  += 1;
        }
        0x73    => { // LD (HL),E 1 8
            memory.memory_array[cpu.get_reg_hl() as usize]  = cpu.reg_e;
            cpu.reg_pc  += 1;
        }
        0x74    => { // LD (HL),H 1 8
            memory.memory_array[cpu.get_reg_hl() as usize]  = cpu.reg_h;
            cpu.reg_pc  += 1;
        }
        0x75    => { // LD (HL),L 1 8
            memory.memory_array[cpu.get_reg_hl() as usize]  = cpu.reg_l;
            cpu.reg_pc  += 1;
        }
        0x76    => { // HALT 1 4

            cpu.reg_pc  += 1;
        }
        0x77    => { // LD (HL),A 1 8
            memory.memory_array[cpu.get_reg_hl() as usize]  = cpu.reg_a;
            cpu.reg_pc  += 1;
        }
        0x78    => { // LD A,B 1 4
            cpu.reg_a   = cpu.reg_b;
            cpu.reg_pc  += 1;
        }
        0x79    => { // LD A,C 1 4
            cpu.reg_a   = cpu.reg_c;
            cpu.reg_pc  += 1;
        }
        0x7A    => { // LD A,D 1 4
            cpu.reg_a   = cpu.reg_d;
            cpu.reg_pc  += 1;
        }
        0x7B    => { // LD A,E 1 4
            cpu.reg_a   = cpu.reg_e;
            cpu.reg_pc  += 1;
        }
        0x7C    => { // LD A,H 1 4
            cpu.reg_a   = cpu.reg_h;
            cpu.reg_pc  += 1;
        }
        0x7D    => { // LD A,L 1 4
            cpu.reg_a   = cpu.reg_l;
            cpu.reg_pc  += 1;
        }
        0x7E    => { // LD A,(HL) 1 8
            cpu.reg_a   = memory.memory_array[cpu.get_reg_hl() as usize];
            cpu.reg_pc  += 1;
        }
        0x7F    => { // LD A,A 1 4
            //cpu.reg_a   = cpu.reg_a;
            cpu.reg_pc  += 1;
        }/*
        0x80    => { //

        }
        0x81    => { //

        }
        0x82    => { //

        }
        0x83    => { //

        }
        0x84    => { //

        }
        0x85    => { //

        }
        0x86    => { //

        }
        0x87    => { //

        }
        0x88    => { //

        }
        0x89    => { //

        }
        0x8A    => { //

        }
        0x8B    => { //

        }
        0x8C    => { //

        }
        0x8D    => { //

        }
        0x8E    => { //

        }
        0x8F    => { //

        }
        0x90    => { //

        }
        0x91    => { //

        }
        0x92    => { //

        }
        0x93    => { //

        }
        0x94    => { //

        }
        0x95    => { //

        }
        0x96    => { //

        }
        0x97    => { //

        }
        0x98    => { //

        }
        0x99    => { //

        }
        0x9A    => { //

        }
        0x9B    => { //

        }
        0x9C    => { //

        }
        0x9D    => { //

        }
        0x9E    => { //

        }
        0x9F    => { //

        }
        0xA0    => { //

        }
        0xA1    => { //

        }
        0xA2    => { //

        }
        0xA3    => { //

        }
        0xA4    => { //

        }
        0xA5    => { //

        }
        0xA6    => { //

        }
        0xA7    => { //

        }
        0xA8    => { //

        }
        0xA9    => { //

        }
        0xAA    => { //

        }
        0xAB    => { //

        }
        0xAC    => { //

        }
        0xAD    => { //

        }
        0xAE    => { //

        }*/
        0xAF    => { // XOR 1 4 Z000
            cpu.reset_n();
            cpu.reset_h();
            cpu.reset_c();
            cpu.reg_pc  += 1;
        }

        0xC3    => { // JP a16 3 16
            cpu.reg_pc  = ((file_buf[(cpu.reg_pc+2) as usize] as u16 * 0x100) + file_buf[(cpu.reg_pc+1) as usize] as u16) as u16;
        }/*
        0xC4    => { // CALL NZ,a16 3 24/12

            cpu.reg_pc  += 3;
        }
        0xC5    => { // PUSH BC 1 16

            cpu.reg_pc  += 1;
        }
        0xC6    => { // ADD A,d8 2 8 Z0HC

            cpu.reg_pc  += 2;
        }
        0xC7    => { // RST 00H 1 16

            cpu.reg_pc  += 1;
        }
        0xC8    => { // RET Z 1 20/8

            cpu.reg_pc  += 1;
        }
        0xC9    => { // RET 1 16

            cpu.reg_pc  += 1;
        }
        0xCA    => { // JP Z,a16 3 16/12

            cpu.reg_pc  += 3;
        }
        0xCB    => { // PREFIX CB 1 4

            cpu.reg_pc  += 1;
        }
        0xCC    => { // CALL Z,a16 3 24/12

            cpu.reg_pc  += 3;
        }
        0xCD    => { // CALL a16 3 24

            cpu.reg_pc  += 3;
        }*/
        0xCE    => { // ADC A,d8 2 8 Z0HC
            cpu.reset_n();
            cpu.reg_pc  += 2;
        }/*
        0xCF    => { // RST 08H 1 16

            cpu.reg_pc  += 1;
        }

        0xD9    => { // RETI 1 16

            cpu.reg_pc  += 1;
        }

        0xE0    => { // LDH (a8),A 2 12

            cpu.reg_pc  += 2;
        }
        0xE1    => { // POP HL 1 12

            cpu.reg_pc  += 1;
        }
        0xE2    => { // LD (C),A 2 8

            cpu.reg_pc  += 2;
        }
        0xE5    => { // PUSH HL 1 16

            cpu.reg_pc  += 1;
        }*/
        0xE6    => { // AND d8 2 8 Z010
            cpu.reset_n();
            cpu.set_h();
            cpu.reset_c();
            cpu.reg_pc  += 2;
        }/*
        0xE7    => { // RST 20H 1 16

            cpu.reg_pc  += 1;
        }*/
        0xE8    => { // ADD SP,r8 2 16 00HC
            cpu.reset_z();
            cpu.reset_n();
            cpu.reg_pc  += 2;
        }/*
        0xE9    => { // JP (HL) 1 4

            cpu.reg_pc  += 1;
        }*/
        0xEA    => { // LD (a16),A 3 16
			memory.memory_array[((file_buf[(cpu.reg_pc+2) as usize] as u16 * 0x100) + file_buf[(cpu.reg_pc+1) as usize] as u16) as usize]	= cpu.reg_a;
            cpu.reg_pc  += 3;
        }
        0xEE    => { // XOR d8 2 8 Z000
            cpu.reset_n();
            cpu.reset_h();
            cpu.reset_c();
            cpu.reg_pc  += 2;
        }/*
        0xEF    => { // RST 28H 1 16

            cpu.reg_pc  += 1;
        }
        0xF0    => { // LDH A,(a8) 2 12

            cpu.reg_pc  += 2;
        }
        0xF1    => { //

        }
        0xF2    => { //

        }*/
        0xF3    => { // DI 1 4
            // disables interrupts after next instruction
			cpu.interrupt_count	= -2;
			cpu.reg_pc	+= 1;
        }/*
        0xF5    => { //

        }
        0xF6    => { //

        }
        0xF7    => { //

        }
        0xF8    => { //

        }
        0xF9    => { //

        }
        0xFA    => { //

        }
        0xFB    => { //

        }*/
        0xFE    => { // CP d8 2 8

            cpu.reg_pc  += 2;
        }
        0xFF    => { // RST 38H 1 16

            cpu.reg_pc  += 1;
        }
        0xD3|0xDB|0xDD|0xE3|0xE4|0xEB|0xEC|0xED|0xF4|0xFC|0xFD => {
            println!("Instruction 0x{:x} is not supported by the GameBoy's CPU", ins);
            cpu.cont    = false;
        }
        _       => {
            println!("Unrecognized/unimplemented instruction: 0x{:x}", ins);
            cpu.cont    = false;
        }
    }
}

fn get_imm_16(file_buf: Vec<u8>, reg_pc: u16) -> u16 {
	((file_buf[(reg_pc+2) as usize] as u16 * 0x100) + file_buf[(reg_pc+1) as usize] as u16) as u16
}

fn ld_8_ar(address: u16, reg: u8, memory: &mut Memory, reg_pc: &mut u16) {

}
fn ld_8_ra(reg: &mut u8, memory: &Memory, reg_pc: &mut u16) {
    //*reg    = memory.memory_array[cpu.get_reg_bc() as usize];
}
fn ld_8_rd(reg: &mut u8, file_buf: &Vec<u8>, reg_pc: &mut u16) {
    *reg    = file_buf[(*reg_pc+1) as usize];
    *reg_pc += 2;
}
fn ld_8_rr(reg1: &mut u8, reg2: u8, reg_pc: &mut u16) {
    *reg1   = reg2;
    *reg_pc += 1;
}

fn ld_8_test(value1: &mut u8, value2: u8, reg_pc: &mut u16, length: u16, cycles: u8) {
    *value1 = value2;
    *reg_pc += length;
}
