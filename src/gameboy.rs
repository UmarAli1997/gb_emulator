use crate::cpu::{CPU, RegisterU8};
use crate::mmu::MemoryBus;
use crate::Instruction;

pub struct Gameboy {
    pub cpu: CPU,
    pub memory: MemoryBus,
    // clock: u8
}

impl Gameboy {
    pub fn read_instruction(&self, memory: &MemoryBus, address: u16) -> u8 {
        return memory.read_byte(address);
    }

    pub fn write_instruction(&self, memory: &mut MemoryBus, address: u16, data: u8) {
        memory.write_byte(address, data)
    }

    pub fn fetch (&mut self) {
        let opcode = self.read_instruction(&self.memory, self.cpu.pc);
        self.cpu.pc += 1;
        self.execute(opcode);
    }

    fn execute(&mut self, opcode: u8) {
        match opcode {
            // 0x opcodes
            0x00 => self.nop(),
            0x01 => todo!(),
            0x02 => todo!(),

            // 0x4 opcodes
            0x40 => self.ld_r_r(RegisterU8::B, RegisterU8::B),
            0x41 => self.ld_r_r(RegisterU8::B, RegisterU8::C),
            0x42 => self.ld_r_r(RegisterU8::B, RegisterU8::D),
            0x43 => self.ld_r_r(RegisterU8::B, RegisterU8::E),
            0x44 => self.ld_r_r(RegisterU8::B, RegisterU8::H),
            0x45 => self.ld_r_r(RegisterU8::B, RegisterU8::L),
            0x46 => todo!(),
            0x47 => self.ld_r_r(RegisterU8::B, RegisterU8::A),
            0x48 => self.ld_r_r(RegisterU8::C, RegisterU8::B),
            0x49 => self.ld_r_r(RegisterU8::C, RegisterU8::C),
            0x4A => self.ld_r_r(RegisterU8::C, RegisterU8::D),
            0x4B => self.ld_r_r(RegisterU8::C, RegisterU8::E),
            0x4C => self.ld_r_r(RegisterU8::C, RegisterU8::H),
            0x4D => self.ld_r_r(RegisterU8::C, RegisterU8::L),
            0x4E => todo!(),
            0x4F => self.ld_r_r(RegisterU8::C, RegisterU8::A),

            //0x5 opcodes
            0x50 => self.ld_r_r(RegisterU8::D, RegisterU8::B),
            0x51 => self.ld_r_r(RegisterU8::D, RegisterU8::C),
            0x52 => self.ld_r_r(RegisterU8::D, RegisterU8::D),
            0x53 => self.ld_r_r(RegisterU8::D, RegisterU8::E),
            0x54 => self.ld_r_r(RegisterU8::D, RegisterU8::H),
            0x55 => self.ld_r_r(RegisterU8::D, RegisterU8::L),
            0x56 => todo!(),
            0x57 => self.ld_r_r(RegisterU8::D, RegisterU8::A),
            0x58 => self.ld_r_r(RegisterU8::E, RegisterU8::B),
            0x59 => self.ld_r_r(RegisterU8::E, RegisterU8::C),
            0x5A => self.ld_r_r(RegisterU8::E, RegisterU8::D),
            0x5B => self.ld_r_r(RegisterU8::E, RegisterU8::E),
            0x5C => self.ld_r_r(RegisterU8::E, RegisterU8::H),
            0x5D => self.ld_r_r(RegisterU8::E, RegisterU8::L),
            0x5E => todo!(),
            0x5F => self.ld_r_r(RegisterU8::E, RegisterU8::A),
            _ => panic!("Opcode not implemented"),
        }
    }

    fn nop(&self) { }

    fn ld_r_r(&mut self, r1: RegisterU8, r2: RegisterU8) {
        //println!("LD_r_r")
        let reg2 = self.cpu.register.read_u8(r2);
        self.cpu.register.write_u8(r1, reg2)
    }

    // fn execute(&mut self, instruction: &Instruction) {
    //     match instruction {
    //         Instruction::LD_r_r => instructions::LD_r_r(),
    //         _ => panic!("Instruction not implemented"),
    //     }
    // }

}