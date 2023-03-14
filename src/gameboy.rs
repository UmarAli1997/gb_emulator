use crate::cpu::*;
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
            0x03 => self.inc_rr(RegisterU16::BC),
            0x04 => self.inc_r(RegisterU8::B),
            0x05 => self.dec_r(RegisterU8::B),
            0x06 => self.ld_r_n(RegisterU8::B),
            0x07 => todo!(),
            0x08 => todo!(),
            0x09 => todo!(),
            0x0A => todo!(),
            0x0B => todo!(),
            0x0C => self.inc_r(RegisterU8::C),
            0x0D => self.dec_r(RegisterU8::C),
            0x0E => self.ld_r_n(RegisterU8::C),
            0x0F => todo!(),

            // 0x1 opcodes
            0x10 => todo!(),
            0x11 => self.ld_rr_nn(RegisterU16::DE),
            0x12 => todo!(),
            0x13 => self.inc_rr(RegisterU16::DE),
            0x14 => self.inc_r(RegisterU8::D),
            0x15 => self.dec_r(RegisterU8::D),
            0x16 => self.ld_r_n(RegisterU8::D),
            0x17 => todo!(),
            0x18 => todo!(),
            0x19 => todo!(),
            0x1A => self.ld_a_de(),
            0x1B => todo!(),
            0x1C => self.inc_r(RegisterU8::E),
            0x1D => self.dec_r(RegisterU8::E),
            0x1E => self.ld_r_n(RegisterU8::E),
            0x1F => todo!(),

            // 0x2 opcodes
            0x20 => self.jr_cc_e(FlagConds::NZ),
            0x21 => self.ld_rr_nn(RegisterU16::HL),
            0x22 => self.ld_hl_plus_a(),
            0x23 => self.inc_rr(RegisterU16::HL),
            0x24 => self.inc_r(RegisterU8::H),
            0x25 => self.dec_r(RegisterU8::H),
            0x26 => self.ld_r_n(RegisterU8::H),
            0x27 => todo!(),
            0x28 => self.jr_cc_e(FlagConds::Z),
            0x29 => todo!(),
            0x2A => todo!(),
            0x2B => todo!(),
            0x2C => self.inc_r(RegisterU8::L),
            0x2D => self.dec_r(RegisterU8::L),
            0x2E => self.ld_r_n(RegisterU8::L),
            0x2F => todo!(),

            // 0x3 opcodes
            0x30 => self.jr_cc_e(FlagConds::NC),
            0x31 => self.ld_rr_nn(RegisterU16::SP),
            0x32 => self.ld_hl_minus_a(),
            0x33 => self.inc_rr(RegisterU16::SP),
            0x34 => todo!(),
            0x35 => todo!(),
            0x36 => todo!(),
            0x37 => todo!(),
            0x38 => self.jr_cc_e(FlagConds::NC),
            0x39 => todo!(),
            0x3A => todo!(),
            0x3B => todo!(),
            0x3C => self.inc_r(RegisterU8::A),
            0x3D => self.dec_r(RegisterU8::A),
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

            //0xB opcodes
            0xB0 => todo!(),
            0xB1 => todo!(),
            0xB2 => todo!(),
            0xB3 => todo!(),
            0xB4 => todo!(),
            0xB5 => todo!(),
            0xB6 => todo!(),
            0xB7 => todo!(),
            0xB8 => todo!(),
            0xB9 => todo!(),
            0xBA => todo!(),
            0xBB => todo!(),
            0xBC => todo!(),
            0xBD => todo!(),
            0xBE => todo!(),
            0xBF => todo!(),

            //0xC opcodes
            0xC0 => todo!(),
            0xC1 => todo!(),
            0xC2 => todo!(),
            0xC3 => todo!(),
            0xC4 => todo!(),
            0xC5 => self.push(RegisterU16::BC),
            0xC6 => todo!(),
            0xC7 => todo!(),
            0xC8 => todo!(),
            0xC9 => todo!(),
            0xCA => todo!(),
            0xCB => self.cb_prefix(),
            0xCC => todo!(),
            0xCD => self.call_nn(),
            0xCE => todo!(),
            0xCF => todo!(),

            //0xD opcodes
            0xD0 => todo!(),
            0xD1 => todo!(),
            0xD2 => todo!(),
            0xD3 => todo!(),
            0xD4 => todo!(),
            0xD5 => self.push(RegisterU16::DE),
            0xD6 => todo!(),
            0xD7 => todo!(),
            0xD8 => todo!(),
            0xD9 => todo!(),
            0xDA => todo!(),
            0xDB => todo!(),
            0xDC => todo!(),
            0xDD => todo!(),
            0xDE => todo!(),
            0xDF => todo!(),

            //0xE opcodes
            0xE0 => self.ldh_n_a(),
            0xE1 => todo!(),
            0xE2 => self.ldh_c_a(),
            0xE3 => todo!(),
            0xE4 => todo!(),
            0xE5 => self.push(RegisterU16::HL),
            0xE6 => todo!(),
            0xE7 => todo!(),
            0xE8 => todo!(),
            0xE9 => todo!(),
            0xEA => todo!(),
            0xEB => todo!(),
            0xEC => todo!(),
            0xED => todo!(),
            0xEE => todo!(),
            0xEF => todo!(),

            //0xF opcodes
            0xF0 => todo!(),
            0xF1 => todo!(),
            0xF2 => todo!(),
            0xF3 => todo!(),
            0xF4 => todo!(),
            0xF5 => self.push(RegisterU16::AF),
            0xF6 => todo!(),
            0xF7 => todo!(),
            0xF8 => todo!(),
            0xF9 => todo!(),
            0xFA => todo!(),
            0xFB => todo!(),
            0xFC => todo!(),
            0xFD => todo!(),
            0xFE => self.cp_n(),
            0xFF => todo!(),

            _ => panic!("Opcode not implemented: {:#X}", opcode),
        }
    }

fn cb_prefix(&mut self) {
    let cb_code = self.read_instruction(self.cpu.register.pc);
    self.cpu.register.pc += 1;

    match cb_code {
        // 0x4 codes
        0x40 => self.bit_r(RegisterU8::B, 0),
        0x41 => self.bit_r(RegisterU8::C, 0),
        0x42 => self.bit_r(RegisterU8::D, 0),
        0x43 => self.bit_r(RegisterU8::E, 0),
        0x44 => self.bit_r(RegisterU8::H, 0),
        0x45 => self.bit_r(RegisterU8::L, 0),
        0x46 => todo!(),
        0x47 => self.bit_r(RegisterU8::A, 0),
        0x48 => self.bit_r(RegisterU8::B, 1),
        0x49 => self.bit_r(RegisterU8::C, 1),
        0x4A => self.bit_r(RegisterU8::D, 1),
        0x4B => self.bit_r(RegisterU8::E, 1),
        0x4C => self.bit_r(RegisterU8::H, 1),
        0x4D => self.bit_r(RegisterU8::L, 1),
        0x4E => todo!(),
        0x4F => self.bit_r(RegisterU8::A, 1),

        // 0x5 codes
        0x50 => self.bit_r(RegisterU8::B, 2),
        0x51 => self.bit_r(RegisterU8::C, 2),
        0x52 => self.bit_r(RegisterU8::D, 2),
        0x53 => self.bit_r(RegisterU8::E, 2),
        0x54 => self.bit_r(RegisterU8::H, 2),
        0x55 => self.bit_r(RegisterU8::L, 2),
        0x56 => todo!(),
        0x57 => self.bit_r(RegisterU8::A, 2),
        0x58 => self.bit_r(RegisterU8::B, 3),
        0x59 => self.bit_r(RegisterU8::C, 3),
        0x5A => self.bit_r(RegisterU8::D, 3),
        0x5B => self.bit_r(RegisterU8::E, 3),
        0x5C => self.bit_r(RegisterU8::H, 3),
        0x5D => self.bit_r(RegisterU8::L, 3),
        0x5E => todo!(),
        0x5F => self.bit_r(RegisterU8::A, 3),

        // 0x6 codes
        0x60 => self.bit_r(RegisterU8::B, 4),
        0x61 => self.bit_r(RegisterU8::C, 4),
        0x62 => self.bit_r(RegisterU8::D, 4),
        0x63 => self.bit_r(RegisterU8::E, 4),
        0x64 => self.bit_r(RegisterU8::H, 4),
        0x65 => self.bit_r(RegisterU8::L, 4),
        0x66 => todo!(),
        0x67 => self.bit_r(RegisterU8::A, 6),
        0x68 => self.bit_r(RegisterU8::B, 5),
        0x69 => self.bit_r(RegisterU8::C, 5),
        0x6A => self.bit_r(RegisterU8::D, 5),
        0x6B => self.bit_r(RegisterU8::E, 5),
        0x6C => self.bit_r(RegisterU8::H, 5),
        0x6D => self.bit_r(RegisterU8::L, 5),
        0x6E => todo!(),
        0x6F => self.bit_r(RegisterU8::A, 5),

        // 0x7 codes
        0x70 => self.bit_r(RegisterU8::B, 6),
        0x71 => self.bit_r(RegisterU8::C, 6),
        0x72 => self.bit_r(RegisterU8::D, 6),
        0x73 => self.bit_r(RegisterU8::E, 6),
        0x74 => self.bit_r(RegisterU8::H, 6),
        0x75 => self.bit_r(RegisterU8::L, 6),
        0x76 => todo!(),
        0x77 => self.bit_r(RegisterU8::A, 6),
        0x78 => self.bit_r(RegisterU8::B, 7),
        0x79 => self.bit_r(RegisterU8::C, 7),
        0x7A => self.bit_r(RegisterU8::D, 7),
        0x7B => self.bit_r(RegisterU8::E, 7),
        0x7C => self.bit_r(RegisterU8::H, 7),
        0x7D => self.bit_r(RegisterU8::L, 7),
        0x7E => todo!(),
        0x7F => self.bit_r(RegisterU8::A, 7),

        _ => panic!("CB Prefixed code not implemented: {:#X}", cb_code)
    }
}

    // CPU instructions
    // Instructions intepreted from https://gekkio.fi/files/gb-docs/gbctr.pdf
    fn nop(&self) { }

    // 8 bit load instructions
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

    fn ld_a_de(&mut self) {
        let address = self.cpu.register.read_u16(RegisterU16::DE);
        let data = self.read_instruction(address);
        self.cpu.register.write_u8(RegisterU8::A, data);
    }

    fn ldh_c_a(&mut self) {
        let msb: u16 = 0xFF00;
        let lsb = self.cpu.register.read_u8(RegisterU8::C) as u16;
        let address = msb | lsb;

        let data = self.read_instruction(address);
        self.cpu.register.write_u8(RegisterU8::A, data);
    }

    fn ldh_n_a(&mut self) {
        let msb: u16 = 0xFF00;
        let lsb = self.read_instruction(self.cpu.register.pc) as u16;
        self.cpu.register.pc += 1;
        let address = msb | lsb;

        let data = self.cpu.register.read_u8(RegisterU8::A);
        self.write_instruction(address, data);
    }

    fn ld_hl_minus_a(&mut self) {
        let mut address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.cpu.register.read_u8(RegisterU8::A);
        self.write_instruction(address, data);
        address -= 1;
        self.cpu.register.write_u16(RegisterU16::HL, address);
    }

    fn ld_hl_plus_a(&mut self) {
        let reg_data = self.cpu.register.read_u8(RegisterU8::A);
        let mut address = self.cpu.register.read_u16(RegisterU16::HL);

        self.write_instruction(address, reg_data);

        address += 1;
        self.cpu.register.write_u16(RegisterU16::HL, address);
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

    fn push(&mut self, r1: RegisterU16) {
        let reg_data = self.cpu.register.read_u16(r1);
        let msb_and_lsb = Registers::u16_to_u8(reg_data);

        let msb = msb_and_lsb[0];
        let lsb = msb_and_lsb[1];

        self.cpu.register.sp -= 1;
        self.write_instruction(self.cpu.register.sp, msb);
        self.cpu.register.sp -= 1;
        self.write_instruction(self.cpu.register.sp, lsb);
    }

    // ALU Instructions

    // 8 bit ALU
    fn _half_carry_add_u8(&self, val_1: u8, val_2: u8) -> bool {
        ((val_1 & 0xF) + (val_2 & 0xF)) & 0x10 == 0x10
    }

    fn _half_carry_sub_u8(&self, value_a: u8, value_b: u8) -> bool {
        (value_a & 0xF) < (value_b & 0xF)
    }

    fn cp_n(&mut self) {
        let data = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let reg_a = self.cpu.register.read_u8(RegisterU8::A);

        let sub_result: (u8, bool) = reg_a.overflowing_sub(data);
        let carry_flag = sub_result.1;

        let half_carry_flag = self._half_carry_sub_u8(reg_a, data);

        if sub_result.0 == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
    }

    fn inc_r(&mut self, r1: RegisterU8) {
        let mut reg_data = self.cpu.register.read_u8(r1);
        let half_carry_flag = self._half_carry_add_u8(reg_data, 1);

        let inc_reg_data = reg_data.overflowing_add(0x1);
        reg_data = inc_reg_data.0;
        self.cpu.register.write_u8(r1, reg_data);

        if reg_data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
    }

    fn dec_r(&mut self, r1: RegisterU8) {
        let mut reg_data = self.cpu.register.read_u8(r1);
        let half_carry_flag = self._half_carry_sub_u8(reg_data, 0x1);

        let dec_reg_data = reg_data.overflowing_sub(0x1);
        reg_data = dec_reg_data.0;
        self.cpu.register.write_u8(r1, reg_data);

        if reg_data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
    }

    fn xor(&mut self, r1: RegisterU8) {
        let reg_data = self.cpu.register.read_u8(r1);
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);

        let result = reg_a ^ reg_data;
        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.flags.set_flag(Flag::C, false);
    }

    fn xor_hl(&mut self) {
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);

        let result = reg_a ^ data;
        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.flags.set_flag(Flag::C, false);
    }

    // 16 bit ALU
    fn inc_rr(&mut self, r1: RegisterU16) {
        let mut reg_data = self.cpu.register.read_u16(r1);
        let inc_reg_data = reg_data.overflowing_add(0x01);
        reg_data = inc_reg_data.0;

        self.cpu.register.write_u16(r1, reg_data);
    }

    // Control flow instructions
    fn jr_cc_e(&mut self, jp_cond: FlagConds) {
        let offset = self.read_instruction(self.cpu.register.pc) as i8;
        self.cpu.register.pc += 1;

        match jp_cond {
            FlagConds::NZ => {
                if !self.cpu.flags.get_flag(Flag::Z) {
                    let mut new_pc = self.cpu.register.pc;
                    new_pc = new_pc.wrapping_add_signed(offset as i16);
                    self.cpu.register.pc = new_pc;
                }
            },

            FlagConds::Z => {
                if self.cpu.flags.get_flag(Flag::Z) {
                    let mut new_pc = self.cpu.register.pc;
                    new_pc = new_pc.wrapping_add_signed(offset as i16);
                    self.cpu.register.pc = new_pc;
                }
            },

            FlagConds::NC => {
                if !self.cpu.flags.get_flag(Flag::C) {
                    let mut new_pc = self.cpu.register.pc;
                    new_pc = new_pc.wrapping_add_signed(offset as i16);
                    self.cpu.register.pc = new_pc;
                }
            },

            FlagConds::C => {
                if self.cpu.flags.get_flag(Flag::C) {
                    let mut new_pc = self.cpu.register.pc;
                    new_pc = new_pc.wrapping_add_signed(offset as i16);
                    self.cpu.register.pc = new_pc;
                }
            },
        }
    }

    fn call_nn(&mut self) {
        let lsb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let msb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;
        let nn = (msb as u16) << 8 | lsb as u16;

        self.cpu.register.sp -= 1;
        self.write_instruction(self.cpu.register.sp, msb);
        self.cpu.register.sp -= 1;
        self.write_instruction(self.cpu.register.sp, lsb);
        self.cpu.register.pc = nn;
    }


    // CB Prefix codes
    fn rl_r(&mut self, r1: RegisterU8) {
        todo!()
    }

    fn bit_r(&mut self, r1: RegisterU8, check_bit: u8) {
        let mask: u8 = 1;
        let mut data = self.cpu.register.read_u8(r1);
        data = data >> check_bit;

        if (data & mask) == 1 {
            self.cpu.flags.set_flag(Flag::Z, false);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, true);
        }

        // Setting half carry flag and unsetting negative flag as per instruction
        self.cpu.flags.set_flag(Flag::H, true);
        self.cpu.flags.set_flag(Flag::N, false);
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    // ld tests
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
    fn test_ld_a_de() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        let address = 0xFE10;
        gameboy.cpu.register.write_u16(RegisterU16::DE, address);
        gameboy.write_instruction(address, 0x01);

        gameboy.ld_a_de();

        let reg_a = gameboy.cpu.register.read_u8(RegisterU8::A);
        assert_eq!(reg_a, 0x01);
    }

    #[test]
    fn test_ldh_c_a() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::C, 0xFF);
        gameboy.write_instruction(0xFFFF, 0x01);

        gameboy.ldh_c_a();
        let new_a = gameboy.cpu.register.read_u8(r1);
        //println!("\n new_a: {:#X}\n", new_a);
        assert_eq!(new_a, 0x01);
    }

    #[test]
    fn test_ldh_n_a() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        gameboy.cpu.register.write_u8(r1, 0x01);
        gameboy.ldh_n_a();

        let data_in_memory = gameboy.read_instruction(0xFF00);
        assert_eq!(data_in_memory, 0x01);
    }

    #[test]
    fn test_ld_hl_minus_a() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::H, 0x11);
        gameboy.cpu.register.write_u8(RegisterU8::L, 0x11);
        gameboy.cpu.register.write_u8(r1, 0x01);

        // Run test and compare output
        gameboy.ld_hl_minus_a();
        let data_in_memory = gameboy.read_instruction(0x1111);
        let new_hl = gameboy.cpu.register.read_u16(RegisterU16::HL);

        assert_eq!(data_in_memory, 0x01);
        assert_eq!(new_hl, 0x1110);
    }

    #[test]
    fn test_ld_hl_plus_a() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        gameboy.cpu.register.write_u16(RegisterU16::HL, 0xFFFE);
        gameboy.cpu.register.write_u8(RegisterU8::A, 0x01);

        gameboy.ld_hl_plus_a();

        let data_in_memory = gameboy.read_instruction(0xFFFE);
        let hl_reg = gameboy.cpu.register.read_u16(RegisterU16::HL);

        assert_eq!(data_in_memory, 0x01);
        assert_eq!(hl_reg, 0xFFFF);
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
    fn test_push() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU16::BC;

        // Set up gameboy state for test
        gameboy.cpu.register.sp = 0xFFFE;
        gameboy.cpu.register.write_u16(r1, 0xFAFB);

        gameboy.push(r1);

        let msb = gameboy.read_instruction(0xFFFE - 1);
        let lsb = gameboy.read_instruction(0xFFFE - 2);

        assert_eq!(msb, 0xFA);
        assert_eq!(lsb, 0xFB);
    }

    #[test]
    fn test_cp_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x1);

        // Run test and compare output
        gameboy.cp_n();
        let new_r1 = gameboy.cpu.register.read_u8(r1);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    // ALU tests
    #[test]
    fn test_inc_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(r1, 0xFF);

        // Run test and compare output
        gameboy.inc_r(r1);
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);

        assert_eq!(new_r1, 0x0);
        assert_eq!(hc_flag, true);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, true);
    }

    #[test]
    fn test_dec_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        gameboy.dec_r(r1);
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);

        assert_eq!(new_r1, 0xFF);
        assert_eq!(hc_flag, true);
        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
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
        let flag_check = gameboy.cpu.flags.get_flag(Flag::Z);

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
        let flag_check = gameboy.cpu.flags.get_flag(Flag::Z);

        assert_eq!(new_r1, 0x0F);
        assert_eq!(flag_check, false);
    }

    // 16 bit ALU test
    #[test]
    fn test_inc_rr() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU16::DE;

        // Run test and compare output
        gameboy.inc_rr(r1);
        let reg_data = gameboy.cpu.register.read_u16(r1);

        assert_eq!(reg_data, 1);
    }


    // Control flow tests
    #[test]
    fn test_jr_cc_e() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.pc = 0x0B;
        gameboy.write_instruction(0x0B, 0xFB);
        gameboy.cpu.flags.set_flag(Flag::C, false);
        gameboy.jr_cc_e(FlagConds::NC);


        let new_pc = gameboy.cpu.register.pc;
        assert_eq!(new_pc, 0x07);
    }

    #[test]
    fn test_call_nn() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x01);
        gameboy.write_instruction(0x01, 0x02);
        gameboy.cpu.register.sp = 0xFFFE;

        // Run test and compare output
        gameboy.call_nn();

        let new_sp = gameboy.cpu.register.sp;
        let new_pc = gameboy.cpu.register.pc;
        let msb = gameboy.read_instruction(0xFFFE - 1);
        let lsb = gameboy.read_instruction(0xFFFE - 2);

        assert_eq!(new_sp, 0xFFFC);
        assert_eq!(new_pc, 0x0201);
        assert_eq!(msb, 0x02);
        assert_eq!(lsb, 0x01);
    }

    #[test]
    fn test_bit_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(r1, 0x9F);

        // Run test and compare output
        gameboy.bit_r(r1, 6);

        assert_eq!(gameboy.cpu.flags.get_flag(Flag::Z), true);
        assert_eq!(gameboy.cpu.flags.get_flag(Flag::N), false);
        assert_eq!(gameboy.cpu.flags.get_flag(Flag::H), true);
    }

}