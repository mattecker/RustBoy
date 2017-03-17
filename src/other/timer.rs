use std::{thread, time};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;

pub struct Timer {
	machine_cycle_freq:	u32,
	pub rx:	Receiver<i32>
}

impl Timer {
	pub fn new(speed_mod: f32) -> Timer {
		let (tx1, rx1)	= channel();
		Timer {
			machine_cycle_freq:	(1_048_576.0 * speed_mod) as u32,
			rx:	rx1
		}
	}

	pub fn start(&mut self) {
		let (tx, rx) = channel();
		self.rx	= rx;

		let handle	= thread::Builder::new().name("timer".to_string()).spawn(move || {
			loop {
	            thread::sleep(time::Duration::from_millis(1));
	            if let Err(_) = tx.send(1) {
	                // End thread
					println!("Timer died");
	                return;
	            }
	        }
		}).unwrap();

		handle.join().expect("Couldn't join on the associated thread");
	}
}
