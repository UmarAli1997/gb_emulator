use crate::cpu::{CPU, RegisterU8, RegisterU16};
use crate::mmu::MemoryBus;

pub struct Gameboy {
    pub cpu: CPU,
    pub memory: MemoryBus,
    // clock: u8
}

impl Gameboy {

    pub fn new() -> Gameboy {
        let gameboy = Gameboy {
            cpu: CPU::new(),
            memory: MemoryBus::new()
        }; 
        gameboy
    }

    pub fn read_instruction(&self, address: u16) -> u8 {
        return self.memory.read_byte(address);
    }

    pub fn write_instruction(&mut self, address: u16, data: u8) {
        self.memory.write_byte(address, data);
    }

    pub fn fetch (&mut self) {
        let opcode = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;
        self.execute(opcode);
    }

    fn execute(&mut self, opcode: u8) {
        // Opcode table: https://izik1.github.io/gbops/index.html
        match opcode {
            // 0x0 opcodes
            0x00 => self.nop(),
            0x01 => self.ld_rr_nn(RegisterU16::BC),
            0x02 => todo!(),
            0x03 => todo!(),
            0x04 => todo!(),
            0x05 => todo!(),
            0x06 => self.ld_r_n(RegisterU8::B),
            0x07 => todo!(),
            0x08 => todo!(),
            0x09 => todo!(),
            0x0A => todo!(),
            0x0B => todo!(),
            0x0C => todo!(),
            0x0D => todo!(),
            0x0E => self.ld_r_n(RegisterU8::C),
            0x0F => todo!(),

            // 0x1 opcodes
            0x10 => todo!(),
            0x11 => self.ld_rr_nn(RegisterU16::DE),
            0x12 => todo!(),
            0x13 => todo!(),
            0x14 => todo!(),
            0x15 => todo!(),
            0x16 => self.ld_r_n(RegisterU8::D),
            0x17 => todo!(),
            0x18 => todo!(),
            0x19 => todo!(),
            0x1A => todo!(),
            0x1B => todo!(),
            0x1C => todo!(),
            0x1D => todo!(),
            0x1E => self.ld_r_n(RegisterU8::E),
            0x1F => todo!(),

            // 0x2 opcodes
            0x20 => todo!(),
            0x21 => self.ld_rr_nn(RegisterU16::HL),
            0x22 => todo!(),
            0x23 => todo!(),
            0x24 => todo!(),
            0x25 => todo!(),
            0x26 => self.ld_r_n(RegisterU8::H),
            0x27 => todo!(),
            0x28 => todo!(),
            0x29 => todo!(),
            0x2A => todo!(),
            0x2B => todo!(),
            0x2C => todo!(),
            0x2D => todo!(),
            0x2E => self.ld_r_n(RegisterU8::L),
            0x2F => todo!(),

            // 0x3 opcodes
            0x30 => todo!(),
            0x31 => self.ld_rr_nn(RegisterU16::SP),
            0x32 => todo!(),
            0x33 => todo!(),
            0x34 => todo!(),
            0x35 => todo!(),
            0x36 => todo!(),
            0x37 => todo!(),
            0x38 => todo!(),
            0x39 => todo!(),
            0x3A => todo!(),
            0x3B => todo!(),
            0x3C => todo!(),
            0x3D => todo!(),
            0x3E => self.ld_r_n(RegisterU8::A),
            0x3F => todo!(),

            // 0x4 opcodes
            0x40 => self.ld_r_r(RegisterU8::B, RegisterU8::B),
            0x41 => self.ld_r_r(RegisterU8::B, RegisterU8::C),
            0x42 => self.ld_r_r(RegisterU8::B, RegisterU8::D),
            0x43 => self.ld_r_r(RegisterU8::B, RegisterU8::E),
            0x44 => self.ld_r_r(RegisterU8::B, RegisterU8::H),
            0x45 => self.ld_r_r(RegisterU8::B, RegisterU8::L),
            0x46 => self.ld_r_hl(RegisterU8::B),
            0x47 => self.ld_r_r(RegisterU8::B, RegisterU8::A),
            0x48 => self.ld_r_r(RegisterU8::C, RegisterU8::B),
            0x49 => self.ld_r_r(RegisterU8::C, RegisterU8::C),
            0x4A => self.ld_r_r(RegisterU8::C, RegisterU8::D),
            0x4B => self.ld_r_r(RegisterU8::C, RegisterU8::E),
            0x4C => self.ld_r_r(RegisterU8::C, RegisterU8::H),
            0x4D => self.ld_r_r(RegisterU8::C, RegisterU8::L),
            0x4E => self.ld_r_hl(RegisterU8::C),
            0x4F => self.ld_r_r(RegisterU8::C, RegisterU8::A),

            // 0x5 opcodes
            0x50 => self.ld_r_r(RegisterU8::D, RegisterU8::B),
            0x51 => self.ld_r_r(RegisterU8::D, RegisterU8::C),
            0x52 => self.ld_r_r(RegisterU8::D, RegisterU8::D),
            0x53 => self.ld_r_r(RegisterU8::D, RegisterU8::E),
            0x54 => self.ld_r_r(RegisterU8::D, RegisterU8::H),
            0x55 => self.ld_r_r(RegisterU8::D, RegisterU8::L),
            0x56 => self.ld_r_hl(RegisterU8::D),
            0x57 => self.ld_r_r(RegisterU8::D, RegisterU8::A),
            0x58 => self.ld_r_r(RegisterU8::E, RegisterU8::B),
            0x59 => self.ld_r_r(RegisterU8::E, RegisterU8::C),
            0x5A => self.ld_r_r(RegisterU8::E, RegisterU8::D),
            0x5B => self.ld_r_r(RegisterU8::E, RegisterU8::E),
            0x5C => self.ld_r_r(RegisterU8::E, RegisterU8::H),
            0x5D => self.ld_r_r(RegisterU8::E, RegisterU8::L),
            0x5E => self.ld_r_hl(RegisterU8::E),
            0x5F => self.ld_r_r(RegisterU8::E, RegisterU8::A),

            // 0x6 opcodes
            0x60 => self.ld_r_r(RegisterU8::H, RegisterU8::B),
            0x61 => self.ld_r_r(RegisterU8::H, RegisterU8::C),
            0x62 => self.ld_r_r(RegisterU8::H, RegisterU8::D),
            0x63 => self.ld_r_r(RegisterU8::H, RegisterU8::E),
            0x64 => self.ld_r_r(RegisterU8::H, RegisterU8::H),
            0x65 => self.ld_r_r(RegisterU8::H, RegisterU8::L),
            0x66 => self.ld_r_hl(RegisterU8::H),
            0x67 => self.ld_r_r(RegisterU8::H, RegisterU8::A),
            0x68 => self.ld_r_r(RegisterU8::L, RegisterU8::B),
            0x69 => self.ld_r_r(RegisterU8::L, RegisterU8::C),
            0x6A => self.ld_r_r(RegisterU8::L, RegisterU8::D),
            0x6B => self.ld_r_r(RegisterU8::L, RegisterU8::E),
            0x6C => self.ld_r_r(RegisterU8::L, RegisterU8::H),
            0x6D => self.ld_r_r(RegisterU8::L, RegisterU8::L),
            0x6E => self.ld_r_hl(RegisterU8::L),
            0x6F => self.ld_r_r(RegisterU8::L, RegisterU8::A),

            //0x7 opcodes
            0x70 => self.ld_hl_r(RegisterU8::B),
            0x71 => self.ld_hl_r(RegisterU8::C),
            0x72 => self.ld_hl_r(RegisterU8::D),
            0x73 => self.ld_hl_r(RegisterU8::E),
            0x74 => self.ld_hl_r(RegisterU8::H),
            0x75 => self.ld_hl_r(RegisterU8::L),
            0x76 => todo!(),
            0x77 => self.ld_hl_r(RegisterU8::A),
            0x78 => self.ld_r_r(RegisterU8::A, RegisterU8::B),
            0x79 => self.ld_r_r(RegisterU8::A, RegisterU8::C),
            0x7A => self.ld_r_r(RegisterU8::A, RegisterU8::D),
            0x7B => self.ld_r_r(RegisterU8::A, RegisterU8::E),
            0x7C => self.ld_r_r(RegisterU8::A, RegisterU8::H),
            0x7D => self.ld_r_r(RegisterU8::A, RegisterU8::L),
            0x7E => self.ld_r_hl(RegisterU8::A),
            0x7F => self.ld_r_r(RegisterU8::A, RegisterU8::A),

            //0xA opcodes
            0xA0 => todo!(),
            0xA1 => todo!(),
            0xA2 => todo!(),
            0xA3 => todo!(),
            0xA4 => todo!(),
            0xA5 => todo!(),
            0xA6 => todo!(),
            0xA7 => todo!(),
            0xA8 => self.xor(RegisterU8::B),
            0xA9 => self.xor(RegisterU8::C),
            0xAA => self.xor(RegisterU8::D),
            0xAB => self.xor(RegisterU8::E),
            0xAC => self.xor(RegisterU8::H),
            0xAD => self.xor(RegisterU8::L),
            0xAE => self.xor_hl(),
            0xAF => self.xor(RegisterU8::A),



            _ => panic!("Opcode not implemented: {:#X}", opcode),
        }
    }


    // CPU instructions
    // Instructions intepreted from https://gekkio.fi/files/gb-docs/gbctr.pdf
    fn nop(&self) { }

    fn ld_r_r(&mut self, r1: RegisterU8, r2: RegisterU8) {
        //println!("LD_r_r")
        let reg2 = self.cpu.register.read_u8(r2);
        self.cpu.register.write_u8(r1, reg2)
        
    }

    fn ld_r_n(&mut self, r1: RegisterU8) {
        let n = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;
        self.cpu.register.write_u8(r1, n)
    }

    fn ld_r_hl(&mut self, r1: RegisterU8) {
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);
        self.cpu.register.write_u8(r1, data);
    }

    fn ld_hl_r(&mut self, r1: RegisterU8) {
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.cpu.register.read_u8(r1);
        self.write_instruction(address, data);
    }


    // 16 bit load instructions
    fn ld_rr_nn(&mut self, r1: RegisterU16) {
        let lsb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let msb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let nn = (msb as u16) << 8 | lsb as u16;
        self.cpu.register.write_u16(r1, nn);
    }

    // ALU Instructions
    fn xor(&mut self, r1: RegisterU8) {
        let reg_data = self.cpu.register.read_u8(r1);
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);

        let result = reg_a ^ reg_data;
        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.z = true;
        }
    }

    fn xor_hl(&mut self) {
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);

        let result = reg_a ^ data;
        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.z = true;
        }

    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ld_r_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;
        let r2 = RegisterU8::B;

        // Set the source register to a value
        gameboy.cpu.register.write_u8(r2, 1);

        // Run the load from r2 to r1 function
        gameboy.ld_r_r(r1, r2);

        // Read the destination register value
        let new_r1 = gameboy.cpu.register.read_u8(r1);

        assert_eq!(new_r1, 1);
    }

    #[test]
    fn test_ld_r_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set pc and memory addresses
        gameboy.cpu.register.pc = 1;
        gameboy.memory.write_byte(0, 0x01);
        gameboy.memory.write_byte(1, 0x02);

        // Run load value from memory into register
        gameboy.ld_r_n(r1);

        // Read destination register value
        let new_r1 = gameboy.cpu.register.read_u8(r1);

        assert_eq!(new_r1, 0x02);
    }

    #[test]
    fn test_ld_r_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::H, 0x11);
        gameboy.cpu.register.write_u8(RegisterU8::L, 0x11);
        gameboy.write_instruction(gameboy.cpu.register.read_u16(RegisterU16::HL), 0x01);

        // Run test and compare output with expected output
        gameboy.ld_r_hl(r1);
        let new_r1 = gameboy.cpu.register.read_u8(r1);

        assert_eq!(new_r1, 0x01);
    }

    #[test]
    fn test_ld_hl_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::H, 0x11);
        gameboy.cpu.register.write_u8(RegisterU8::L, 0x11);
        gameboy.cpu.register.write_u8(r1, 0x01);

        // Run test and compare output
        gameboy.ld_hl_r(r1);
        let data_in_memory = gameboy.read_instruction(gameboy.cpu.register.read_u16(RegisterU16::HL));
        assert_eq!(data_in_memory, 0x01);
    }

    #[test]
    fn test_ld_rr_nn() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU16::AF;
        let sp = RegisterU16::SP;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u16(RegisterU16::PC, 0);
        gameboy.memory.write_byte(0, 0x01);
        gameboy.memory.write_byte(1, 0x02);
        gameboy.memory.write_byte(2, 0xFF);
        gameboy.memory.write_byte(3, 0xFF);

        // Run test and compare output
        gameboy.ld_rr_nn(r1);
        gameboy.ld_rr_nn(sp);
        let msb = gameboy.cpu.register.read_u8(RegisterU8::A);
        let lsb = gameboy.cpu.register.read_u8(RegisterU8::F);
        let new_sp = gameboy.cpu.register.read_u16(RegisterU16::SP);

        assert_eq!((msb, lsb), (0x02, 0x01));
        assert_eq!(new_sp, 0xFFFF);
    }

    #[test]
    fn test_xor() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::B;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(r1, 0xFF);

        // Run test and compare output
        gameboy.xor(r1);
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let flag_check = gameboy.cpu.flags.z;

        assert_eq!(new_r1, 0xFF);
        assert_eq!(flag_check, false);
    }

    #[test]
    fn test_xor_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(r1, 0xFF);
        gameboy.cpu.register.write_u16(RegisterU16::HL, 0x01);
        gameboy.write_instruction(0x01, 0xF0);

        // Run test and compare output
        gameboy.xor_hl();

        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let flag_check = gameboy.cpu.flags.z;

        assert_eq!(new_r1, 0x0F);
        assert_eq!(flag_check, false);
    }
}