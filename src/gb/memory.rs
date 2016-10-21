#[allow(dead_code)]
#[derive(Default)]
pub struct Memory {
    pub memory_array:   Vec<u8>
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory_array:   vec![0x00; 0x10000],

        }
    }

    pub fn initialize(&mut self) {
        self.memory_array[0xFF05]    = 0x00;
        self.memory_array[0xFF06]    = 0x00;
        self.memory_array[0xFF07]    = 0x00;
        self.memory_array[0xFF10]    = 0x80;
        self.memory_array[0xFF11]    = 0xBF;
        self.memory_array[0xFF12]    = 0xF3;
        self.memory_array[0xFF14]    = 0xBF;
        self.memory_array[0xFF16]    = 0x3F;
        self.memory_array[0xFF17]    = 0x00;
        self.memory_array[0xFF19]    = 0xBF;
        self.memory_array[0xFF1A]    = 0x7F;
        self.memory_array[0xFF1B]    = 0xFF;
        self.memory_array[0xFF1C]    = 0x9F;
        self.memory_array[0xFF1E]    = 0xBF;
        self.memory_array[0xFF20]    = 0xFF;
        self.memory_array[0xFF21]    = 0x00;
        self.memory_array[0xFF22]    = 0x00;
        self.memory_array[0xFF23]    = 0xBF;
        self.memory_array[0xFF24]    = 0x77;
        self.memory_array[0xFF25]    = 0xF3;
        self.memory_array[0xFF26]    = 0xF1; // 0xF0 on SGB
        self.memory_array[0xFF40]    = 0x91;
        self.memory_array[0xFF42]    = 0x00;
        self.memory_array[0xFF43]    = 0x00;
        self.memory_array[0xFF45]    = 0x00;
        self.memory_array[0xFF47]    = 0xFC;
        self.memory_array[0xFF48]    = 0xFF;
        self.memory_array[0xFF49]    = 0xFF;
        self.memory_array[0xFF4A]    = 0x00;
        self.memory_array[0xFF4B]    = 0x00;
        self.memory_array[0xFFFF]    = 0x00;
    }
}
