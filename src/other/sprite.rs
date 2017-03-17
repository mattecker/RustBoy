
pub struct Sprite {
	pub pixels:	[[u8; 8]; 8]
}

impl Sprite {
	pub fn new() -> Sprite {




		Sprite {
			pixels:	[[0 as u8; 8]; 8]
		}
	}

	pub fn init(&mut self, file_buf: &Vec<u8>, start: u16) {
		for y in 0..8 {
			for x in 0..8 {
				self.pixels[y as usize][x as usize]	= bytes_to_mono(get_bit_at_8(file_buf[(start + y as u16) as usize], ((x as i8 - 7) * -1) as u8), get_bit_at_8(file_buf[(start + 1 as u16 + y as u16) as usize], ((x as i8 - 7) * -1) as u8));
			}
		}
	}


}

fn bytes_to_mono(b1: bool, b2: bool) -> u8 {
	if !b1 & !b2 { // 0, 0-0
		0 as u8
	} else if b1 & !b2 { // 1, 1-0
		255 as u8
	} else if !b1 & b2 { // 2, 0-1
		184 as u8
	} else { // 3, 1-1
		104 as u8
	}
}

fn get_bit_at_8(input: u8, n: u8) -> bool {
	if n < 8 {
		input & (1 << n) != 0
	} else {
		false
	}
}
