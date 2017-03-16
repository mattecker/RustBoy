use gb;
use gb::cpu::Cpu;
use gb::memory::Memory;

pub struct Emu {
	cpu:	gb::cpu::Cpu,
	memory:	gb::memory::Memory,
	file_buf:	Vec<u8>,
	ins:	u8,
	ins_count:	usize
}

impl Emu {
	pub fn new(file_buf: Vec<u8>) -> Emu {
		Emu {
			cpu:		gb::cpu::Cpu::new(),
		    memory:		gb::memory::Memory::new(),
			file_buf:	file_buf,
			ins:		0,
			ins_count:	0
		}
	}

	pub fn init(&mut self) {
		self.memory.initialize();
		self.ins	= self.file_buf[self.cpu.reg_pc as usize]; // keeps track of current instruction
	}

	pub fn exec(&mut self) {
		while self.cpu.cont {
			self.cpu.print();

	        if self.ins == 0xCB {
	            self.ins = self.file_buf[(self.cpu.reg_pc + 1) as usize];
	            gb::instructions_cb::exec_ins_cb(&mut self.cpu, &mut self.memory, &self.file_buf, self.ins);
	        } else {
	            gb::instructions::exec_ins(&mut self.cpu, &mut self.memory, &self.file_buf, self.ins);
	        }

			self.ins_count	+= 1;
			if self.ins_count == 50 {
				self.cpu.cont	= false;
			}

	        self.ins = self.file_buf[self.cpu.reg_pc as usize];

			if self.cpu.interrupt_count != 0 {
				self.cpu.interrupt_handler();
			}
	    }
	}
}
