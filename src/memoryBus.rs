pub struct MemoryBus {
    ram: [u8; 65535],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    fn write_byte(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }
}