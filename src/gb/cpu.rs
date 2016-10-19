#[allow(dead_code)]
#[derive(Default)]
pub struct Cpu { // some variables public for debugging at the moment
    pub reg_a:  u8,	// accumulator
    pub reg_b:  u8,
    pub reg_c:  u8,
    pub reg_d:  u8,
    pub reg_e:  u8,
    pub reg_f:  u8,	// flags
    pub reg_h:  u8,
    pub reg_l:  u8,

    pub reg_sp: u16,// stack pointer
    pub reg_pc: u16	// program counter
}

#[allow(dead_code)]
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
        	reg_a:	0,
        	reg_b:  0,
		    reg_c:  0,
		    reg_d:  0,
		    reg_e:  0,
		    reg_f:  0,
		    reg_h:  0,
		    reg_l:  0,

        	reg_sp:	0xFFFE,
        	reg_pc:	0x0100,
        }
    }
}
