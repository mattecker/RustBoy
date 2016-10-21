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

    pub fn reg_af(&self) -> u16 {
        let reg_af = ((&self.reg_a * 128) + &self.reg_f) as u16;
        reg_af
    }

    pub fn reg_bc(&self) -> u16 {
        let reg_bc = ((&self.reg_b * 128) + &self.reg_c) as u16;
        reg_bc
    }

    pub fn reg_de(&self) -> u16 {
        let reg_de = ((&self.reg_d * 128) + &self.reg_e) as u16;
        reg_de
    }

    pub fn reg_hl(&self) -> u16 {
        let reg_hl = ((&self.reg_h * 128) + &self.reg_l) as u16;
        reg_hl
    }

    pub fn set_z(&mut self) {
        if self.reg_f.leading_zeros() == 1 {
            self.reg_f += 0b10000000u8;
        }
    }
    pub fn set_n(&mut self) {

    }
    pub fn set_h(&mut self) {

    }
    pub fn set_c(&mut self) {
        if self.reg_f.trailing_zeros() == 5 {
            self.reg_f += 0b00010000u8;
        }
    }

    pub fn reset_z(&mut self) {
        if self.reg_f.leading_zeros() == 0 {
            self.reg_f -= 0b10000000u8;
        }
    }
    pub fn reset_n(&mut self) {

    }
    pub fn reset_h(&mut self) {

    }
    pub fn reset_c(&mut self) {
        if self.reg_f.trailing_zeros() == 4 {
            self.reg_f -= 0b00010000u8;
        }
    }

    pub fn get_z(&self) -> bool {
        let mut flag_z: bool;

        if self.reg_f.leading_zeros() == 0 {
            flag_z = true;
        } else {
            flag_z = false;
        }

        flag_z
    }
    pub fn get_n(&self) -> bool { // todo
        false
    }
    pub fn get_h(&self) -> bool { // todo
        false
    }
    pub fn get_c(&self) -> bool {
        let mut flag_c: bool;

        if self.reg_f.trailing_zeros() == 4 {
            flag_c = true;
        } else {
            flag_c = false;
        }

        flag_c
    }

}
