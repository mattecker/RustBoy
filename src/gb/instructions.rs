use gb::cpu::Cpu;
#[allow(dead_code)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]

// will hopefully look cleaner than individual functions when executing an instruction
// comment format: [name] [parameters (optional)] [byte length] [cycles]
pub fn exec_ins(cpu: &mut Cpu, file_buf: &Vec<u8>, ins: u8) {
    println!("Executing instruction 0x{:x}", ins);

    match ins {
        0x00    =>  { // NOP 1 4
            cpu.reg_pc  += 1;
        }
        0x01    =>  { // LD (BC),d16 3 12
            cpu.reg_b   = file_buf[(cpu.reg_pc+2) as usize];
            cpu.reg_c   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 3;
        }
        0x02    =>  { // LD (BC),A 1 8
            cpu.reg_c   = cpu.reg_a;
            cpu.reg_b   = 0b00000000u8;
            cpu.reg_pc  += 1;
        }
        0x03    =>  { // INC BC 1 8

            cpu.reg_pc  += 1;
        }
        0x04    =>  { // INC B 1 4
            cpu.reg_b   += 1;
            cpu.reg_pc  += 1;
        }
        0x05    =>  { // DEC B 1 4
            cpu.reg_b   -= 1;
            cpu.reg_pc  += 1;
        }
        0x06    =>  { // LD B,d8 2 8
            cpu.reg_b   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 2;
        }
        0x07    =>  { // RLCA 1 4

            cpu.reg_pc  += 1;
        }
        0x08    =>  { //LD (a16),SP 3 20

            cpu.reg_pc  += 3;
        }
        0x09    =>  { // ADD HL,BC 1 8

            cpu.reg_pc  += 1;
        }
        0x0A    =>  { // LD A,(BC) 1 8

            cpu.reg_pc  += 1;
        }
        0x0B    =>  { // DEC BC 1 8

            cpu.reg_pc  += 1;
        }
        0x0C    =>  { // INC C 1 4
            cpu.reg_c   += 1;
            cpu.reg_pc  += 1;
        }
        0x0D    =>  { // DEC C 1 4
            cpu.reg_c   -= 1;
            cpu.reg_pc  += 1;
        }
        0x0E    =>  { // LD C,d8 2 8
            cpu.reg_c   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 2;
        }
        0x0F    =>  { // RRCA 1 4

            cpu.reg_pc  += 1;
        }
        0x10    =>  { // STOP 0 2 4

            cpu.reg_pc  += 2;
        }
        0x11    =>  { // LD DE,d16 3 12
            cpu.reg_d   = file_buf[(cpu.reg_pc+2) as usize];
            cpu.reg_e   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 3;
        }
        0x12    =>  { // LD (DE),A 1 8
            cpu.reg_e   = cpu.reg_a;
            cpu.reg_d   = 0b00000000u8;
            cpu.reg_pc  += 1;
        }
        0x13    =>  { // INC DE 1 8

            cpu.reg_pc  += 1;
        }
        0x14    =>  { // INC D 1 4
            cpu.reg_d   += 1;
            cpu.reg_pc  += 1;
        }
        0x15    =>  { // DEC D 1 4
            cpu.reg_d   -= 1;
            cpu.reg_pc  += 1;
        }
        0x16    =>  { // LD D,d8 2 8
            cpu.reg_d   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 2;
        }
        0x17    =>  { // RLA 1 4

            cpu.reg_pc  += 1;
        }
        0x18    =>  { // JR r8 2 12

            cpu.reg_pc  += 2;
        }
        0x19    =>  { // ADD HL,DE 1 8

            cpu.reg_pc  += 1;
        }
        0x1A    =>  { // LD A,(DE) 1 8

            cpu.reg_pc  += 1;
        }
        0x1B    =>  { // DEC DE 1 8

            cpu.reg_pc  += 1;
        }
        0x1C    =>  { // INC E 1 4
            cpu.reg_e   += 1;
            cpu.reg_pc  += 1;
        }
        0x1D    =>  { // DEC E 1 4
            cpu.reg_e   -= 1;
            cpu.reg_pc  += 1;
        }
        0x1E    =>  { // LD E,d8 2 8
            cpu.reg_e   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 2;
        }
        0x1F    =>  { // RRA 1 4

            cpu.reg_pc  += 1;
        }
        0x20    =>  { // JR NZ,r8 2 12/8

            cpu.reg_pc  += 2;
        }
        0x21    =>  { // LD HL,d16 3 12

            cpu.reg_pc  += 3;
        }
        0x22    =>  { // LD (HL+),A 1 8

            cpu.reg_pc  += 1;
        }
        0x23    =>  { // INC HL 1 8

            cpu.reg_pc  += 1;
        }
        0x24    =>  { // INC H 1 4
            cpu.reg_h   += 1;
            cpu.reg_pc  += 1;
        }
        0x25    =>  { // DEC H 1 4
            cpu.reg_h   -= 1;
            cpu.reg_pc  += 1;
        }
        0x26    =>  { // LD H,d8 2 8
            cpu.reg_h   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 2;
        }
        0x27    =>  { // DAA 1 4

            cpu.reg_pc  += 1;
        }
        0x28    =>  { // JR Z,r8 2 12/8

            cpu.reg_pc  += 2;
        }
        0x29    =>  { // ADD HL,HL 1 8

            cpu.reg_pc  += 1;
        }
        0x2A    =>  { // LD A,(HL+) 1 8

            cpu.reg_pc  += 1;
        }
        0x2B    =>  { // DEC HL 1 8

            cpu.reg_pc  += 1;
        }
        0x2C    =>  { // INC L 1 4
            cpu.reg_l   += 1;
            cpu.reg_pc  += 1;
        }
        0x2D    =>  { // DEC L 1 4
            cpu.reg_l   -= 1;
            cpu.reg_pc  += 1;
        }
        0x2E    =>  { // LD L,d8 2 8
            cpu.reg_l   = file_buf[(cpu.reg_pc+1) as usize];
            cpu.reg_pc  += 2;
        }
        0x2F    =>  { // CPL 1 4

            cpu.reg_pc  += 1;
        }
        0x30    =>  { //

        }
        0x31    =>  { //

        }
        0x32    =>  { //

        }
        0x33    =>  { //

        }
        0x34    =>  { //

        }
        0x35    =>  { //

        }
        0x36    =>  { //

        }
        0x37    =>  { //

        }
        0x38    =>  { //

        }
        0x39    =>  { //

        }
        0x3A    =>  { //

        }
        0x3B    =>  { //

        }
        0x3C    =>  { //

        }
        0x3D    =>  { //

        }
        0x3E    =>  { //

        }
        0x3F    =>  { //

        }
        0x40    =>  { //

        }
        0x41    =>  { //

        }
        0x42    =>  { //

        }
        0x43    =>  { //

        }
        0x44    =>  { //

        }
        0x45    =>  { //

        }
        0x46    =>  { //

        }
        0x47    =>  { //

        }
        0x48    =>  { //

        }
        0x49    =>  { //

        }
        0x4A    =>  { //

        }
        0x4B    =>  { //

        }
        0x4C    =>  { //

        }
        0x4D    =>  { //

        }
        0x4E    =>  { //

        }
        0x4F    =>  { //

        }
        0x50    =>  { //

        }
        0x51    =>  { //

        }
        0x52    =>  { //

        }
        0x53    =>  { //

        }
        0x54    =>  { //

        }
        0x55    =>  { //

        }
        0x56    =>  { //

        }
        0x57    =>  { //

        }
        0x58    =>  { //

        }
        0x59    =>  { //

        }
        0x5A    =>  { //

        }
        0x5B    =>  { //

        }
        0x5C    =>  { //

        }
        0x5D    =>  { //

        }
        0x5E    =>  { //

        }
        0x5F    =>  { //

        }
        0x60    =>  { //

        }
        0x61    =>  { //

        }
        0x62    =>  { //

        }
        0x63    =>  { //

        }
        0x64    =>  { //

        }
        0x65    =>  { //

        }
        0x66    =>  { //

        }
        0x67    =>  { //

        }
        0x68    =>  { //

        }
        0x69    =>  { //

        }
        0x6A    =>  { //

        }
        0x6B    =>  { //

        }
        0x6C    =>  { //

        }
        0x6D    =>  { //

        }
        0x6E    =>  { //

        }
        0x6F    =>  { //

        }
        0x70    =>  { //

        }
        0x71    =>  { //

        }
        0x72    =>  { //

        }
        0x73    =>  { //

        }
        0x74    =>  { //

        }
        0x75    =>  { //

        }
        0x76    =>  { //

        }
        0x77    =>  { //

        }
        0x78    =>  { //

        }
        0x79    =>  { //

        }
        0x7A    =>  { //

        }
        0x7B    =>  { //

        }
        0x7C    =>  { //

        }
        0x7D    =>  { //

        }
        0x7E    =>  { //

        }
        0x7F    =>  { //

        }
        0x80    =>  { //

        }
        0x81    =>  { //

        }
        0x82    =>  { //

        }
        0x83    =>  { //

        }
        0x84    =>  { //

        }
        0x85    =>  { //

        }
        0x86    =>  { //

        }
        0x87    =>  { //

        }
        0x88    =>  { //

        }
        0x89    =>  { //

        }
        0x8A    =>  { //

        }
        0x8B    =>  { //

        }
        0x8C    =>  { //

        }
        0x8D    =>  { //

        }
        0x8E    =>  { //

        }
        0x8F    =>  { //

        }
        0x90    =>  { //

        }
        0x91    =>  { //

        }
        0x92    =>  { //

        }
        0x93    =>  { //

        }
        0x94    =>  { //

        }
        0x95    =>  { //

        }
        0x96    =>  { //

        }
        0x97    =>  { //

        }
        0x98    =>  { //

        }
        0x99    =>  { //

        }
        0x9A    =>  { //

        }
        0x9B    =>  { //

        }
        0x9C    =>  { //

        }
        0x9D    =>  { //

        }
        0x9E    =>  { //

        }
        0x9F    =>  { //

        }

        0xC3    =>  { // JP a16 3 16
            //cpu.reg_pc = de_endian(file_buf[(cpu.reg_pc+1) as usize], file_buf[(cpu.reg_pc+2) as usize]);
            cpu.reg_pc  = ((file_buf[(cpu.reg_pc+2) as usize] * 128) + file_buf[(cpu.reg_pc+1) as usize]) as u16;
        }
        0xFE    =>  { // CP d8 2 8

        }
        0xD3|0xDB|0xDD|0xE3|0xE4|0xEB|0xEC|0xED|0xF4|0xFC|0xFD => { // unsupported instructions
            println!{"Instruction {} is not supported by the GameBoy's CPU", ins};
            cpu.reg_pc  += 1;
        }
        // catch-all will result in an infinite loop if read from a ROM, need to implement shutdown code
        _       =>  println!("Unrecognized/unimplemented instruction: {}", ins),
    }
}

// reverses the order of two 8-bit values so that they can be used as a 16-bit address
/* probably superfluous, can do the operation inside instructions
fn de_endian(first: u8, second: u8) -> u16 {
    let bit_data: u16 = ((second * 128) + first) as u16;
    bit_data
}*/
