use gb;
use gb::cpu::Cpu;
use gb::memory::Memory;
use other::sprite::Sprite;
use other::timer::Timer;
use std::time::SystemTime;

pub struct Emu {
	cpu:		Cpu,
	memory:		Memory,
	file_buf:	Vec<u8>,
	ins:		u8,
	ins_count:	usize,
	timer:		Timer
}

impl Emu {
	pub fn new(file_buf: Vec<u8>) -> Emu {
		Emu {
			cpu:		Cpu::new(),
		    memory:		Memory::new(),
			file_buf:	file_buf,
			ins:		0,
			ins_count:	0,
			timer:		Timer::new(1 as f32)
		}
	}

	pub fn init(&mut self) {
		self.memory.initialize();
		self.ins	= self.file_buf[self.cpu.reg_pc as usize]; // keeps track of current instruction
		self.timer.start();
	}

	pub fn exec(&mut self) -> Sprite {
		let mut spr	= Sprite::new();

		if self.cpu.cont {
			// self.cpu.print();

	        if self.ins == 0xCB {
	            self.ins = self.file_buf[(self.cpu.reg_pc + 1) as usize];
	            gb::instructions_cb::exec_ins_cb(&mut self.cpu, &mut self.memory, &self.file_buf, self.ins);
	        } else {
	            gb::instructions::exec_ins(&mut self.cpu, &mut self.memory, &self.file_buf, self.ins);
	        }

			// self.ins_count	+= 1;
			// if self.ins_count == 300 {
			// 	self.cpu.cont	= false;
			// }

	        self.ins = self.file_buf[self.cpu.reg_pc as usize];

			if self.cpu.interrupt_count != 0 {
				self.cpu.interrupt_handler();
			}

			spr.init(&self.file_buf, 0x8016 as u16);
	    }

		// sleep logic, broken for now
		if self.cpu.cycle_count >= 1_048 { // rounded down, since the number of instructions will be at or above depending on the last instruction
			self.cpu.cycle_count	= 0;	// reset cycle counter
			// if let Err(e) = self.timer.rx.recv() { // wait for next timer cycle
	        //      panic!("Error: Timer not responding {:?}", e);
	        // }
			self.timer.rx.recv();
			let now = SystemTime::now();
			println!("{:?}", now);
		}



		spr
	}
}
