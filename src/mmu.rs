pub struct MemoryBus {
    pub ram: [u8; 0x10000 as usize],
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            ram: [0xFF; 0x10000 as usize],
        }
    }

    pub fn copy_to_ram(&mut self, address: u16, rom_file: &[u8]) {
        self.ram[(address as usize)..(address as usize + rom_file.len())].copy_from_slice(rom_file);
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.ram[address as usize] = data;
    }
}