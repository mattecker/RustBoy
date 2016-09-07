#[allow(unused_imports)]
use std::env;




#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn ret_info (file_buf: &Vec<u8>) {
	let mut	rom_name		= String::new();
    let mut	gameboy_color	= String::new();
    let mut	super_gameboy	= String::new();
    let mut	cartridge_type	= String::new();
    let mut	rom_size		= String::new();
    let mut	ram_size		= String::new();
    let mut	destination		= String::new();

    for x in 0x0134..0x0143 {
    	rom_name.push(file_buf[x] as char);
    }
    println!("ROM Name:       {}", rom_name);

    if file_buf[0x0143] == 128 {
    	gameboy_color = "Yes".to_string();
    } else {
    	gameboy_color = "No".to_string();
    }
    println!("GameBoy Color:  {}", gameboy_color);

    if file_buf[0x0146] == 3 {
    	super_gameboy = "Yes".to_string();
    } else {
    	super_gameboy = "No".to_string();
    }
    println!("Super GameBoy:  {}", super_gameboy);

    match file_buf[0x0147] {
    	0	=> cartridge_type = "ROM ONLY".to_string(),
    	1	=> cartridge_type = "ROM+MBC".to_string(),
    	2	=> cartridge_type = "ROM+MBC1+RAM".to_string(),
    	3	=> cartridge_type = "ROM+MBC1+RAM+BATTERY".to_string(),
    	5	=> cartridge_type = "ROM+MBC2".to_string(),
    	6	=> cartridge_type = "ROM+MBC2+BATTERY".to_string(),
    	8	=> cartridge_type = "ROM+RAM".to_string(),
    	9	=> cartridge_type = "ROM+RAM+BATTERY".to_string(),
    	11	=> cartridge_type = "ROM+MMM01".to_string(),
    	12	=> cartridge_type = "ROM+MMM01+SRAM".to_string(),
    	13	=> cartridge_type = "ROM+MMM01+SRAM+BATTERY".to_string(),
    	15	=> cartridge_type = "ROM+MBC3+TIMER+BATTERY".to_string(),
    	16	=> cartridge_type = "ROM+MBC3+TIMER+RAM+BATTERY".to_string(),
    	17	=> cartridge_type = "ROM+MBC3".to_string(),
    	18	=> cartridge_type = "ROM+MBC3+RAM".to_string(),
    	19	=> cartridge_type = "ROM+MBC3+RAM+BATTERY".to_string(),
    	25	=> cartridge_type = "ROM+MBC5".to_string(),
    	26	=> cartridge_type = "ROM+MBC5+RAM".to_string(),
    	27	=> cartridge_type = "ROM+MBC5+RAM+BATTERY".to_string(),
    	28	=> cartridge_type = "ROM+MBC5+RUMBLE".to_string(),
    	29	=> cartridge_type = "ROM+MBC5+RUMBLE+SRAM".to_string(),
    	30	=> cartridge_type = "ROM+MBC5+RUMBLE+SRAM+BATTERY".to_string(),
    	31	=> cartridge_type = "Pocket Camera".to_string(),
    	253	=> cartridge_type = "Bandai TAMA5".to_string(),
    	254	=> cartridge_type = "Hudson HuC -3".to_string(),
    	255	=> cartridge_type = "Hudson HuC -1".to_string(),
    	_	=> cartridge_type = "Unrecognized value, try blowing on the cartridge".to_string(),
    }
    println!("Cartridge Type: {}", cartridge_type);

    match file_buf[0x0148] {
    	0  => rom_size = "256Kbit".to_string(),
    	1  => rom_size = "512Kbit".to_string(),
    	2  => rom_size = "1Mbit".to_string(),
    	3  => rom_size = "2Mbit".to_string(),
    	4  => rom_size = "4Mbit".to_string(),
    	5  => rom_size = "8Mbit".to_string(),
    	6  => rom_size = "16Mbit".to_string(),
    	82 => rom_size = "9Mbit".to_string(),
    	83 => rom_size = "10Mbit".to_string(),
    	84 => rom_size = "12Mbit".to_string(),
    	_  => rom_size = "Unrecognized value, try blowing on the cartridge".to_string(),
    }
    println!("ROM Size:       {}", rom_size);

    match file_buf[0x0149] {
    	0 => ram_size = "None".to_string(),
    	1 => ram_size = "16kBit".to_string(),
    	2 => ram_size = "64kBit".to_string(),
    	3 => ram_size = "256kBit".to_string(),
    	4 => ram_size = "1MBit".to_string(),
    	_ => ram_size = "Unrecognized value, try blowing on the cartridge".to_string(),
    }
    println!("RAM Size:       {}", ram_size);

    
    if file_buf[0x014A] == 0 {
    	destination = "Japanese".to_string();
    } else {
    	destination = "Non-Japanese".to_string();
    }
    println!("Destination:    {}", destination);
}