//use gb::memory::Memory;
use std::sync::mpsc::channel;
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
    pub reg_pc: u16,// program counter

    pub cont:   bool,
	pub allow_interrupts:	bool,
	pub interrupt_count:	i8,
	pub cycle_count:	usize
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
        	reg_a:	0x01,
        	reg_b:  0x00,
		    reg_c:  0x14,
		    reg_d:  0x00,
		    reg_e:  0x00,
		    reg_f:  0x00,
		    reg_h:  0xC0,
		    reg_l:  0x60,

        	reg_sp:	0xFFFE,
        	reg_pc:	0x0100,

            cont:   true,
			allow_interrupts:	false,
			interrupt_count:	0,
			cycle_count:		0
        }
    }

    // combination register getters
    pub fn get_reg_af(&self) -> u16 {
        (((self.reg_a as u16 * 0x100)) + (self.reg_f as u16)) as u16
    }

    pub fn get_reg_bc(&self) -> u16 {
        (((self.reg_b as u16 * 0x100)) + (self.reg_c as u16)) as u16
    }

    pub fn get_reg_de(&self) -> u16 {
        (((self.reg_d as u16 * 0x100)) + (self.reg_e as u16)) as u16
    }

    pub fn get_reg_hl(&self) -> u16 {
        (((self.reg_h as u16 * 0x100)) + (self.reg_l as u16)) as u16
    }

    // combination register setters
    pub fn set_reg_af(&mut self, data: u16) {
        self.reg_a  = (data >> 8) as u8;
        self.reg_f  = data as u8;
    }

    pub fn set_reg_bc(&mut self, data: u16) {
        self.reg_b  = (data >> 8) as u8;
        self.reg_c  = data as u8;
    }

    pub fn set_reg_de(&mut self, data: u16) {
        self.reg_d  = (data >> 8) as u8;
        self.reg_e  = data as u8;
    }

    pub fn set_reg_hl(&mut self, data: u16) {
        self.reg_h  = (data >> 8) as u8;
        self.reg_l  = data as u8;
    }

    // flag setters
    pub fn set_z(&mut self) {
        if !get_bit_at_8(self.reg_f, 7) {
            self.reg_f += 0b10000000u8;
        }
    }
    pub fn set_n(&mut self) {
        if !get_bit_at_8(self.reg_f, 6) {
            self.reg_f += 0b01000000u8;
        }
    }
    pub fn set_h(&mut self) {
        if !get_bit_at_8(self.reg_f, 5) {
            self.reg_f += 0b00100000u8;
        }
    }
    pub fn set_c(&mut self) {
        if !get_bit_at_8(self.reg_f, 4) {
            self.reg_f += 0b00010000u8;
        }
    }

    // flag resetters
    pub fn reset_z(&mut self) {
        if get_bit_at_8(self.reg_f, 7) {
            self.reg_f -= 0b10000000u8;
        }
    }
    pub fn reset_n(&mut self) {
        if get_bit_at_8(self.reg_f, 6) {
            self.reg_f -= 0b01000000u8;
        }
    }
    pub fn reset_h(&mut self) {
        if get_bit_at_8(self.reg_f, 5) {
            self.reg_f -= 0b00100000u8;
        }
    }
    pub fn reset_c(&mut self) {
        if get_bit_at_8(self.reg_f, 4) {
            self.reg_f -= 0b00010000u8;
        }
    }

    // flag getters
    pub fn get_z(&self) -> bool {
        get_bit_at_8(self.reg_f, 7)
    }
    pub fn get_n(&self) -> bool {
        get_bit_at_8(self.reg_f, 6)
    }
    pub fn get_h(&self) -> bool {
        get_bit_at_8(self.reg_f, 5)
    }
    pub fn get_c(&self) -> bool {
        get_bit_at_8(self.reg_f, 4)
    }

	pub fn print(&self) {
		println!("\nRegisters: \naf: {:04X} \nbc: {:04X} \nde: {:04X} \nhl: {:04X}", self.get_reg_af(), self.get_reg_bc(), self.get_reg_de(), self.get_reg_hl());
		println!("sp: {:04X} \npc: {:04X}\n", self.reg_sp, self.reg_pc);
	}


	pub fn interrupt_handler(&mut self) {
		match self.interrupt_count {
			2	=>	self.interrupt_count	-= 1,
			1	=>	{
				self.allow_interrupts	= true;
				self.interrupt_count	-= 1;
				println!("Interrupts enabled");
			}
			-1	=>	{
				self.allow_interrupts	= false;
				self.interrupt_count	+= 1;
				println!("Interrupts disabled");
			}
			-2	=>	self.interrupt_count	+= 1,
			_	=>	{
				println!("Error: invalid interrupt_count value in cpu.interrupt_handler");
				self.cont	= false;
			}
		}
	}

	// pub fn sleep(&mut self) {
	//
	// }
}

// modified from https://www.reddit.com/r/rust/comments/3xgeo0/biginner_question_how_can_i_get_the_value_of_a/cy4ei5n/
fn get_bit_at_16(input: u16, n: u8) -> bool {
    if n < 16 {
        input & (1 << n) != 0
    } else {
        false
    }
}
pub fn get_bit_at_8(input: u8, n: u8) -> bool {
    if n < 8 {
        input & (1 << n) != 0
    } else {
        false
    }
}
