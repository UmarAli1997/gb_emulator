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
        //println!("PC: {:#X}", self.cpu.register.pc);
        println!("Opcode: {:#X} \nA: {:#X} F: {:#X} B: {:#X} C: {:#X} D: {:#X} E: {:#X} H: {:#X} L: {:#X} SP: {:#X} PC: {:#X}", opcode, self.cpu.register.a,
        self.cpu.register.f, self.cpu.register.b, self.cpu.register.c, self.cpu.register.d, self.cpu.register.e, self.cpu.register.h, self.cpu.register.l, self.cpu.register.sp,
        self.cpu.register.pc);
        self.cpu.register.pc += 1;
        self.execute(opcode);
    }

    fn _half_carry_add_u8(&self, val_1: u8, val_2: u8) -> bool {
        ((val_1 & 0xF) + (val_2 & 0xF)) & 0x10 == 0x10
    }

    fn _half_carry_sub_u8(&self, value_a: u8, value_b: u8) -> bool {
        (value_a & 0xF) < (value_b & 0xF)
    }

    fn execute(&mut self, opcode: u8) {
        // Opcode table: https://izik1.github.io/gbops/index.html
        match opcode {
            // 0x0 opcodes
            0x00 => self.nop(),
            0x01 => self.ld_rr_nn(RegisterU16::BC),
            0x02 => self.ld_rr_a(RegisterU16::BC),
            0x03 => self.inc_rr(RegisterU16::BC),
            0x04 => self.inc_r(RegisterU8::B),
            0x05 => self.dec_r(RegisterU8::B),
            0x06 => self.ld_r_n(RegisterU8::B),
            0x07 => todo!(),
            0x08 => self.ld_nn_sp(),
            0x09 => todo!(),
            0x0A => self.ld_a_rr(RegisterU16::BC),
            0x0B => todo!(),
            0x0C => self.inc_r(RegisterU8::C),
            0x0D => self.dec_r(RegisterU8::C),
            0x0E => self.ld_r_n(RegisterU8::C),
            0x0F => todo!(),

            // 0x1 opcodes
            0x10 => todo!(),
            0x11 => self.ld_rr_nn(RegisterU16::DE),
            0x12 => self.ld_rr_a(RegisterU16::DE),
            0x13 => self.inc_rr(RegisterU16::DE),
            0x14 => self.inc_r(RegisterU8::D),
            0x15 => self.dec_r(RegisterU8::D),
            0x16 => self.ld_r_n(RegisterU8::D),
            0x17 => self.rla(),
            0x18 => self.jr_e(),
            0x19 => todo!(),
            0x1A => self.ld_a_rr(RegisterU16::DE),
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
            0x2A => self.ld_a_hl_plus(),
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
            0x34 => self.inc_hl(),
            0x35 => self.dec_hl(),
            0x36 => todo!(),
            0x37 => todo!(),
            0x38 => self.jr_cc_e(FlagConds::C),
            0x39 => todo!(),
            0x3A => self.ld_a_hl_minus(),
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

            //0x8 opcodes
            0x80 => self.add_r(RegisterU8::B),
            0x81 => self.add_r(RegisterU8::C),
            0x82 => self.add_r(RegisterU8::D),
            0x83 => self.add_r(RegisterU8::E),
            0x84 => self.add_r(RegisterU8::H),
            0x85 => self.add_r(RegisterU8::L),
            0x86 => self.add_hl(),
            0x87 => self.add_r(RegisterU8::A),
            0x88 => self.adc_r(RegisterU8::B),
            0x89 => self.adc_r(RegisterU8::C),
            0x8A => self.adc_r(RegisterU8::D),
            0x8B => self.adc_r(RegisterU8::E),
            0x8C => self.adc_r(RegisterU8::H),
            0x8D => self.adc_r(RegisterU8::L),
            0x8E => self.adc_hl(),
            0x8F => self.adc_r(RegisterU8::A),

            //0x9 opcodes
            0x90 => self.sub_r(RegisterU8::B),
            0x91 => self.sub_r(RegisterU8::C),
            0x92 => self.sub_r(RegisterU8::D),
            0x93 => self.sub_r(RegisterU8::E),
            0x94 => self.sub_r(RegisterU8::H),
            0x95 => self.sub_r(RegisterU8::L),
            0x96 => self.sub_hl(),
            0x97 => self.sub_r(RegisterU8::A),
            0x98 => self.sbc_r(RegisterU8::B),
            0x99 => self.sbc_r(RegisterU8::C),
            0x9A => self.sbc_r(RegisterU8::D),
            0x9B => self.sbc_r(RegisterU8::E),
            0x9C => self.sbc_r(RegisterU8::H),
            0x9D => self.sbc_r(RegisterU8::L),
            0x9E => self.sbc_hl(),
            0x9F => self.sbc_r(RegisterU8::A),

            //0xA opcodes
            0xA0 => self.and_r(RegisterU8::B),
            0xA1 => self.and_r(RegisterU8::C),
            0xA2 => self.and_r(RegisterU8::D),
            0xA3 => self.and_r(RegisterU8::E),
            0xA4 => self.and_r(RegisterU8::H),
            0xA5 => self.and_r(RegisterU8::L),
            0xA6 => self.and_hl(),
            0xA7 => self.and_r(RegisterU8::A),
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
            0xB8 => self.cp_r(RegisterU8::B),
            0xB9 => self.cp_r(RegisterU8::C),
            0xBA => self.cp_r(RegisterU8::D),
            0xBB => self.cp_r(RegisterU8::E),
            0xBC => self.cp_r(RegisterU8::H),
            0xBD => self.cp_r(RegisterU8::L),
            0xBE => self.cp_hl(),
            0xBF => self.cp_r(RegisterU8::A),

            //0xC opcodes
            0xC0 => todo!(),
            0xC1 => self.pop(RegisterU16::BC),
            0xC2 => todo!(),
            0xC3 => todo!(),
            0xC4 => todo!(),
            0xC5 => self.push(RegisterU16::BC),
            0xC6 => self.add_n(),
            0xC7 => todo!(),
            0xC8 => todo!(),
            0xC9 => self.ret(),
            0xCA => todo!(),
            0xCB => self.cb_prefix(),
            0xCC => todo!(),
            0xCD => self.call_nn(),
            0xCE => self.adc_n(),
            0xCF => todo!(),

            //0xD opcodes
            0xD0 => todo!(),
            0xD1 => self.pop(RegisterU16::DE),
            0xD2 => todo!(),
            0xD3 => todo!(),
            0xD4 => todo!(),
            0xD5 => self.push(RegisterU16::DE),
            0xD6 => self.sub_n(),
            0xD7 => todo!(),
            0xD8 => todo!(),
            0xD9 => todo!(),
            0xDA => todo!(),
            0xDB => todo!(),
            0xDC => todo!(),
            0xDD => todo!(),
            0xDE => self.sbc_n(),
            0xDF => todo!(),

            //0xE opcodes
            0xE0 => self.ldh_n_a(),
            0xE1 => self.pop(RegisterU16::HL),
            0xE2 => self.ldh_c_a(),
            0xE3 => todo!(),
            0xE4 => todo!(),
            0xE5 => self.push(RegisterU16::HL),
            0xE6 => todo!(),
            0xE7 => todo!(),
            0xE8 => todo!(),
            0xE9 => todo!(),
            0xEA => self.ld_nn_a(),
            0xEB => todo!(),
            0xEC => todo!(),
            0xED => todo!(),
            0xEE => todo!(),
            0xEF => todo!(),

            //0xF opcodes
            0xF0 => self.ldh_a_n(),
            0xF1 => self.pop(RegisterU16::AF),
            0xF2 => self.ldh_a_c(),
            0xF3 => todo!(),
            0xF4 => todo!(),
            0xF5 => self.push(RegisterU16::AF),
            0xF6 => todo!(),
            0xF7 => todo!(),
            0xF8 => todo!(),
            0xF9 => self.ld_sp_hl(),
            0xFA => self.ld_a_nn(),
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
        // 0x1 codes
        0x10 => self.rl_r(RegisterU8::B),
        0x11 => self.rl_r(RegisterU8::C),
        0x12 => self.rl_r(RegisterU8::D),
        0x13 => self.rl_r(RegisterU8::E),
        0x14 => self.rl_r(RegisterU8::H),
        0x15 => self.rl_r(RegisterU8::L),
        0x16 => todo!(),
        0x17 => self.rl_r(RegisterU8::A),
        0x18 => todo!(),
        0x19 => todo!(),
        0x1A => todo!(),
        0x1B => todo!(),
        0x1C => todo!(),
        0x1D => todo!(),
        0x1E => todo!(),
        0x1F => todo!(),

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

    // ld_a_bc/ld_a_de
    fn ld_a_rr(&mut self, r1: RegisterU16) {
        let address = self.cpu.register.read_u16(r1);
        let data = self.read_instruction(address);
        self.cpu.register.write_u8(RegisterU8::A, data);
    }

    // ld_bc_a/ld_de_a
    fn ld_rr_a(&mut self, r1: RegisterU16) {
        let address = self.cpu.register.read_u16(r1);
        let data = self.cpu.register.read_u8(RegisterU8::A);
        self.write_instruction(address, data);
    }

    fn ld_a_nn(&mut self) {
        let lsb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let msb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let nn = (msb as u16) << 8 | lsb as u16;
        let data = self.read_instruction(nn);
        self.cpu.register.write_u8(RegisterU8::A, data);
    }

    fn ld_nn_a(&mut self) {
        let lsb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let msb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let reg_data = self.cpu.register.read_u8(RegisterU8::A);
        let nn = (msb as u16) << 8 | lsb as u16;

        self.write_instruction(nn, reg_data);
    }

    fn ldh_a_c(&mut self) {
        let msb: u16 = 0xFF00;
        let lsb = self.cpu.register.read_u8(RegisterU8::C) as u16;
        let address = msb | lsb;

        let data = self.read_instruction(address);

        self.cpu.register.write_u8(RegisterU8::A, data);
    }

    fn ldh_c_a(&mut self) {
        let msb: u16 = 0xFF00;
        let lsb = self.cpu.register.read_u8(RegisterU8::C) as u16;
        let address = msb | lsb;

        let reg_a = self.cpu.register.read_u8(RegisterU8::A);

        self.write_instruction(address, reg_a);
    }

    fn ldh_a_n(&mut self) {
        let msb: u16 = 0xFF00;
        let lsb = self.read_instruction(self.cpu.register.pc) as u16;
        self.cpu.register.pc += 1;

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

    fn ld_a_hl_minus(&mut self) {
        let mut address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);

        address = address.wrapping_sub(1);
        self.cpu.register.write_u16(RegisterU16::HL, address);

        self.cpu.register.write_u8(RegisterU8::A, data);
    }

    fn ld_hl_minus_a(&mut self) {
        let mut address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.cpu.register.read_u8(RegisterU8::A);

        self.write_instruction(address, data);
        address = address.wrapping_sub(1);

        self.cpu.register.write_u16(RegisterU16::HL, address);
    }

    fn ld_a_hl_plus(&mut self) {
        let mut address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);

        self.cpu.register.write_u8(RegisterU8::A, data);

        address = address.wrapping_add(1);
        self.cpu.register.write_u16(RegisterU16::HL, address);
    }

    fn ld_hl_plus_a(&mut self) {
        let reg_data = self.cpu.register.read_u8(RegisterU8::A);
        let mut address = self.cpu.register.read_u16(RegisterU16::HL);

        self.write_instruction(address, reg_data);

        address = address.wrapping_add(1);
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

    fn ld_nn_sp(&mut self) {
        let lsb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let msb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let nn = (msb as u16) << 8 | lsb as u16;

        let [sp_lsb, sp_msb] = self.cpu.register.sp.to_le_bytes();

        self.write_instruction(nn, sp_lsb);
        self.write_instruction(nn.wrapping_add(1), sp_msb);
    }

    fn ld_sp_hl(&mut self) {
        let reg_data = self.cpu.register.read_u16(RegisterU16::HL);
        self.cpu.register.sp = reg_data;
    }

    fn push(&mut self, r1: RegisterU16) {
        let reg_data = self.cpu.register.read_u16(r1);
        let [lsb, msb] = reg_data.to_le_bytes();

        self.cpu.register.sp -= 1;
        self.write_instruction(self.cpu.register.sp, msb);
        self.cpu.register.sp -= 1;
        self.write_instruction(self.cpu.register.sp, lsb);
    }

    fn pop(&mut self, r1: RegisterU16) {
        let lsb = self.read_instruction(self.cpu.register.sp);
        self.cpu.register.sp += 1;
        
        let msb = self.read_instruction(self.cpu.register.sp);
        self.cpu.register.sp += 1;

        let data = (msb as u16) << 8 | lsb as u16;
        self.cpu.register.write_u16(r1, data);
    }

    // ALU Instructions

    // 8 bit ALU
    fn add_r(&mut self, r1: RegisterU8) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let reg_data = self.cpu.register.read_u8(r1);

        let (result, carry_flag) = reg_a.overflowing_add(reg_data);
        let half_carry_flag = self._half_carry_add_u8(reg_a, reg_data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn add_hl(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);

        let (result, carry_flag) = reg_a.overflowing_add(data);
        let half_carry_flag = self._half_carry_add_u8(reg_a, data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn add_n(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let data = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let (result, carry_flag) = reg_a.overflowing_add(data);
        let half_carry_flag = self._half_carry_add_u8(reg_a, data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn adc_r(&mut self, r1: RegisterU8) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let reg_data = self.cpu.register.read_u8(r1);
        let f_reg = self.cpu.register.get_f_reg(self.cpu.flags);

        let carry_flag_u8 = f_reg & 0b0001_0000;

        let (result, carry_flag_1) = reg_a.overflowing_add(carry_flag_u8);

        // Half carry calculation in the middle of adding to make use of the result value before it is overwritten
        let half_carry_flag_1 = self._half_carry_add_u8(reg_a, carry_flag_u8);
        let half_carry_flag_2 = self._half_carry_add_u8(result, reg_data);

        let (result, carry_flag_2) = result.overflowing_add(reg_data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        let carry_flag = carry_flag_1 | carry_flag_2;
        let half_carry_flag = half_carry_flag_1 | half_carry_flag_2;

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn adc_hl(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);
        let f_reg = self.cpu.register.get_f_reg(self.cpu.flags);

        let carry_flag_u8 = f_reg & 0b0001_0000;

        let (result, carry_flag_1) = reg_a.overflowing_add(carry_flag_u8);

        // Half carry calculation in the middle of adding to make use of the result value before it is overwritten
        let half_carry_flag_1 = self._half_carry_add_u8(reg_a, carry_flag_u8);
        let half_carry_flag_2 = self._half_carry_add_u8(result, data);

        let (result, carry_flag_2) = result.overflowing_add(data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        let carry_flag = carry_flag_1 | carry_flag_2;
        let half_carry_flag = half_carry_flag_1 | half_carry_flag_2;

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn adc_n(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let data = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let f_reg = self.cpu.register.get_f_reg(self.cpu.flags);
        let carry_flag_u8 = f_reg & 0b0001_0000;

        let (result, carry_flag_1) = reg_a.overflowing_add(carry_flag_u8);

        // Half carry calculation in the middle of adding to make use of the result value before it is overwritten
        let half_carry_flag_1 = self._half_carry_add_u8(reg_a, carry_flag_u8);
        let half_carry_flag_2 = self._half_carry_add_u8(result, data);

        let (result, carry_flag_2) = result.overflowing_add(data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        let carry_flag = carry_flag_1 | carry_flag_2;
        let half_carry_flag = half_carry_flag_1 | half_carry_flag_2;

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn sub_r(&mut self, r1: RegisterU8) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let reg_data = self.cpu.register.read_u8(r1);

        let (result, carry_flag) = reg_a.overflowing_sub(reg_data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        let half_carry_flag = self._half_carry_sub_u8(reg_a, reg_data);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn sub_hl(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);

        let (result, carry_flag) = reg_a.overflowing_sub(data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        let half_carry_flag = self._half_carry_sub_u8(reg_a, data);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn sub_n(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let data = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let (result, carry_flag) = reg_a.overflowing_sub(data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        let half_carry_flag = self._half_carry_sub_u8(reg_a, data);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn sbc_r(&mut self, r1: RegisterU8) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let reg_data = self.cpu.register.read_u8(r1);
        let f_reg = self.cpu.register.get_f_reg(self.cpu.flags);

        let carry_flag_u8 = f_reg & 0b0001_0000;

        let (result, carry_flag_1) = reg_a.overflowing_sub(carry_flag_u8);

        // Half carry calculation in the middle of adding to make use of the result value before it is overwritten
        let half_carry_flag_1 = self._half_carry_sub_u8(reg_a, carry_flag_u8);
        let half_carry_flag_2 = self._half_carry_sub_u8(result, reg_data);

        let (result, carry_flag_2) = result.overflowing_sub(reg_data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        let carry_flag = carry_flag_1 | carry_flag_2;
        let half_carry_flag = half_carry_flag_1 | half_carry_flag_2;

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn sbc_hl(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);
        let f_reg = self.cpu.register.get_f_reg(self.cpu.flags);

        let carry_flag_u8 = f_reg & 0b0001_0000;

        let (result, carry_flag_1) = reg_a.overflowing_sub(carry_flag_u8);

        // Half carry calculation in the middle of adding to make use of the result value before it is overwritten
        let half_carry_flag_1 = self._half_carry_sub_u8(reg_a, carry_flag_u8);
        let half_carry_flag_2 = self._half_carry_sub_u8(result, data);

        let (result, carry_flag_2) = result.overflowing_sub(data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        let carry_flag = carry_flag_1 | carry_flag_2;
        let half_carry_flag = half_carry_flag_1 | half_carry_flag_2;

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn sbc_n(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let data = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let f_reg = self.cpu.register.get_f_reg(self.cpu.flags);
        let carry_flag_u8 = f_reg & 0b0001_0000;

        let (result, carry_flag_1) = reg_a.overflowing_sub(carry_flag_u8);

        // Half carry calculation in the middle of adding to make use of the result value before it is overwritten
        let half_carry_flag_1 = self._half_carry_sub_u8(reg_a, carry_flag_u8);
        let half_carry_flag_2 = self._half_carry_sub_u8(result, data);

        let (result, carry_flag_2) = result.overflowing_sub(data);

        self.cpu.register.write_u8(RegisterU8::A, result);

        let carry_flag = carry_flag_1 | carry_flag_2;
        let half_carry_flag = half_carry_flag_1 | half_carry_flag_2;

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn cp_r(&mut self, r1: RegisterU8) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let reg_data = self.cpu.register.read_u8(r1);

        let (result, carry_flag) = reg_a.overflowing_sub(reg_data);
        let half_carry_flag = self._half_carry_sub_u8(reg_a, reg_data);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn cp_hl(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let address = self.cpu.register.read_u16(RegisterU16::HL);

        let data = self.read_instruction(address);

        let (result, carry_flag) = reg_a.overflowing_sub(data);
        let half_carry_flag = self._half_carry_sub_u8(reg_a, data);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn cp_n(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let data = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let (result, carry_flag): (u8, bool) = reg_a.overflowing_sub(data);

        let half_carry_flag = self._half_carry_sub_u8(reg_a, data);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn inc_r(&mut self, r1: RegisterU8) {
        let reg_data = self.cpu.register.read_u8(r1);
        let half_carry_flag = self._half_carry_add_u8(reg_data, 1);

        let reg_data = reg_data.wrapping_add(0x1);
        self.cpu.register.write_u8(r1, reg_data);

        if reg_data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn inc_hl(&mut self) {
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let mut data = self.read_instruction(address);

        let half_carry_flag = self._half_carry_add_u8(data, 1);

        data = data.wrapping_add(0x1);
        self.write_instruction(address, data);

        if data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn dec_r(&mut self, r1: RegisterU8) {
        let reg_data = self.cpu.register.read_u8(r1);
        let half_carry_flag = self._half_carry_sub_u8(reg_data, 0x1);

        let reg_data = reg_data.wrapping_sub(0x1);
        self.cpu.register.write_u8(r1, reg_data);

        if reg_data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn dec_hl(&mut self) {
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let mut data = self.read_instruction(address);

        let half_carry_flag = self._half_carry_sub_u8(data, 1);

        data = data.wrapping_sub(0x1);
        self.write_instruction(address, data);

        if data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn and_r(&mut self, r1: RegisterU8) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let reg_data = self.cpu.register.read_u8(r1);

        let result = reg_a & reg_data;

        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }
        
        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, true);
        self.cpu.flags.set_flag(Flag::C, false);
    }

    fn and_hl(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);

        let result = reg_a & data;

        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, true);
        self.cpu.flags.set_flag(Flag::C, false);
    }

    fn and_n(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let data = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let result = reg_a & data;

        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, true);
        self.cpu.flags.set_flag(Flag::C, false);
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
        self.cpu.register.update_f_reg(self.cpu.flags);
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
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    // 16 bit ALU
    fn inc_rr(&mut self, r1: RegisterU16) {
        let mut reg_data = self.cpu.register.read_u16(r1);
        let inc_reg_data = reg_data.overflowing_add(0x01);
        reg_data = inc_reg_data.0;

        self.cpu.register.write_u16(r1, reg_data);
    }

    // Control flow instructions
    fn jr_e(&mut self) {
        let offset = self.read_instruction(self.cpu.register.pc) as i8;
        self.cpu.register.pc += 1;
        let mut new_pc = self.cpu.register.pc;

        new_pc = new_pc.wrapping_add_signed(offset as i16);
        self.cpu.register.pc = new_pc;
    }

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

        let [lsb_pc, msb_pc] = self.cpu.register.pc.to_le_bytes();

        self.cpu.register.sp -= 1;
        self.write_instruction(self.cpu.register.sp, msb_pc);
        self.cpu.register.sp -= 1;
        self.write_instruction(self.cpu.register.sp, lsb_pc);
        self.cpu.register.pc = nn;
    }

    fn ret(&mut self) {
        let lsb = self.read_instruction(self.cpu.register.sp);
        self.cpu.register.sp += 1;
        
        let msb = self.read_instruction(self.cpu.register.sp);
        self.cpu.register.sp += 1;

        self.cpu.register.pc = (msb as u16) << 8 | lsb as u16;
    }

    // Rotate instructions
    fn rla(&mut self) {
        let carry_flag = self.cpu.flags.get_flag(Flag::C);
        let reg_data = self.cpu.register.read_u8(RegisterU8::A);

        let new_carry_flag: bool = (reg_data & 0b1000_0000) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let mut new_reg_data = reg_data << 1;
        new_reg_data = new_reg_data & 0b1111_1110;

        if carry_flag {
            new_reg_data += 1;
            self.cpu.register.write_u8(RegisterU8::A, new_reg_data);
        }
        else {
            self.cpu.register.write_u8(RegisterU8::A, new_reg_data);
        }

        self.cpu.flags.set_flag(Flag::Z, false);
        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }


    // CB Prefix codes
    fn rl_r(&mut self, r1: RegisterU8) {
        let carry_flag = self.cpu.flags.get_flag(Flag::C);
        let reg_data = self.cpu.register.read_u8(r1);

        let new_carry_flag: bool = (reg_data & 0b1000_0000) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let mut new_reg_data = reg_data << 1;
        new_reg_data = new_reg_data & 0b1111_1110;

        if carry_flag {
            new_reg_data += 1;
            self.cpu.register.write_u8(r1, new_reg_data);
        }
        else {
            self.cpu.register.write_u8(r1, new_reg_data);
        }

        if new_reg_data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }
        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
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
        self.cpu.register.update_f_reg(self.cpu.flags);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ld tests
    #[test]
    fn ld_r_r() {
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
    fn ld_r_n() {
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
    fn ld_r_hl() {
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
    fn ld_hl_r() {
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
    fn ld_a_rr() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU16::DE;

        // Set up gameboy state for test
        let address = 0xFE10;
        gameboy.cpu.register.write_u16(r1, address);
        gameboy.write_instruction(address, 0x01);

        gameboy.ld_a_rr(r1);

        let reg_a = gameboy.cpu.register.read_u8(RegisterU8::A);
        assert_eq!(reg_a, 0x01);
    }

    #[test]
    fn ld_rr_a() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU16::DE;

        // Set up gameboy state for test
        let address = 0xFE10;
        gameboy.cpu.register.write_u16(r1, address);
        gameboy.cpu.register.write_u8(RegisterU8::A, 0x01);

        // Run test and compare output
        gameboy.ld_rr_a(r1);
        
        let new_reg = gameboy.read_instruction(address);
        assert_eq!(new_reg, 0x01);
    }

    #[test]
    fn ld_a_nn() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0xFFFE, 0x01);
        gameboy.write_instruction(0x0, 0xFE);
        gameboy.write_instruction(0x01, 0xFF);

        // Run test and compare output
        gameboy.ld_a_nn();

        let reg_data = gameboy.cpu.register.read_u8(RegisterU8::A);
        assert_eq!(0x01, reg_data);
    }

    #[test]
    fn ld_nn_a() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0x01);
        gameboy.write_instruction(0x0, 0xFE);
        gameboy.write_instruction(0x01, 0xFF);

        // Run test and compare output
        gameboy.ld_nn_a();

        let data_in_memory = gameboy.read_instruction(0xFFFE);
        assert_eq!(data_in_memory, 0x01);
    }

    #[test]
    fn ldh_a_c() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::C, 0xFF);
        gameboy.write_instruction(0xFFFF, 0x01);

        // Run test and compare output
        gameboy.ldh_a_c();
        let new_reg = gameboy.cpu.register.read_u8(RegisterU8::A);

        assert_eq!(new_reg, 0x01);
    }

    #[test]
    fn ldh_c_a() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::C, 0xFF);
        gameboy.cpu.register.write_u8(RegisterU8::A, 0x01);

        // Run test and compare output
        gameboy.ldh_c_a();
        let data_in_memory = gameboy.read_instruction(0xFFFF);

        assert_eq!(data_in_memory, 0x01);
    }

    #[test]
    fn ldh_a_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0xFA);
        gameboy.write_instruction(0xFFFA, 0x01);

        // Run test and compare output
        gameboy.ldh_a_n();

        let new_r1 = gameboy.cpu.register.read_u8(r1);
        assert_eq!(new_r1, 0x01);
    }

    #[test]
    fn ldh_n_a() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        gameboy.cpu.register.write_u8(r1, 0x01);
        gameboy.ldh_n_a();

        let data_in_memory = gameboy.read_instruction(0xFFFF);
        assert_eq!(data_in_memory, 0x01);
    }

    #[test]
    fn ld_a_hl_minus() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u16(RegisterU16::HL, 0xFFFE);
        gameboy.write_instruction(0xFFFE, 0x01);

        gameboy.ld_a_hl_minus();

        let new_reg = gameboy.cpu.register.read_u8(RegisterU8::A);
        assert_eq!(new_reg, 0x01);
    }

    #[test]
    fn ld_hl_minus_a() {
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
    fn ld_a_hl_plus() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::H, 0x11);
        gameboy.cpu.register.write_u8(RegisterU8::L, 0x11);
        gameboy.write_instruction(0x1111, 0x01);

        // Run test and compare output
        gameboy.ld_a_hl_plus();

        let reg_data = gameboy.cpu.register.read_u8(r1);
        let new_hl = gameboy.cpu.register.read_u16(RegisterU16::HL);

        assert_eq!(reg_data, 0x01);
        assert_eq!(new_hl, 0x1112);
    }

    #[test]
    fn ld_hl_plus_a() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u16(RegisterU16::HL, 0xFFFE);
        gameboy.cpu.register.write_u8(RegisterU8::A, 0x01);

        // Run test and compare output
        gameboy.ld_hl_plus_a();

        let data_in_memory = gameboy.read_instruction(0xFFFE);
        let hl_reg = gameboy.cpu.register.read_u16(RegisterU16::HL);

        assert_eq!(data_in_memory, 0x01);
        assert_eq!(hl_reg, 0xFFFF);
    }

    #[test]
    fn ld_rr_nn() {
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
    fn ld_nn_sp() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let sp = RegisterU16::SP;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u16(sp, 0xFFFE);
        gameboy.write_instruction(0x0, 0x01);
        gameboy.write_instruction(0x1, 0x02);

        // Run test and compare output
        gameboy.ld_nn_sp();

        let sp_lsb = gameboy.read_instruction(0x201);
        let sp_msb = gameboy.read_instruction(0x202);

        assert_eq!(sp_lsb, 0xFE);
        assert_eq!(sp_msb, 0xFF);
    }

    #[test]
    fn ld_sp_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u16(RegisterU16::HL, 0xFFFE);

        // Run test and compare output
        gameboy.ld_sp_hl();

        let new_sp = gameboy.cpu.register.read_u16(RegisterU16::SP);
        assert_eq!(new_sp, 0xFFFE);
    }

    #[test]
    fn push() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU16::BC;

        // Set up gameboy state for test
        gameboy.cpu.register.sp = 0xFFFE;
        gameboy.cpu.register.write_u16(r1, 0xFAFB);

        // Run test and compare output
        gameboy.push(r1);

        let msb = gameboy.read_instruction(0xFFFE - 1);
        let lsb = gameboy.read_instruction(0xFFFE - 2);

        assert_eq!(msb, 0xFA);
        assert_eq!(lsb, 0xFB);
    }

    #[test]
    fn pop() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU16::BC;

        // Set up gameboy state for test
        gameboy.cpu.register.sp = 0xFFFC;
        gameboy.write_instruction(0xFFFC, 0xFB);
        gameboy.write_instruction(0xFFFC + 1, 0xFA);

        gameboy.pop(r1);
        let data = gameboy.cpu.register.read_u16(r1);
        let [lsb, msb] = data.to_le_bytes();

        assert_eq!(msb, 0xFA);
        assert_eq!(lsb, 0xFB);
    }

    // ALU tests
    #[test]
    fn add_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xFF);

        // Run test and compare output
        gameboy.add_r(RegisterU8::A);

        let reg_a = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(reg_a, 0xFE);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn add_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xFF);
        gameboy.write_instruction(0x0, 0x1);

        // Run test and compare output
        gameboy.add_hl();

        let reg_a = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(reg_a, 0x0);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, true);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn add_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0xFF);

        // Run test and compare output
        gameboy.add_n();

        let reg_a = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(reg_a, 0xFF);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, false);
        assert_eq!(c_flag, false);
    }

    #[test]
    fn adc_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xFF);
        gameboy.cpu.register.write_u8(RegisterU8::B, 0x01);
        gameboy.cpu.flags.set_flag(Flag::C, true);

        // Run test and compare output
        gameboy.adc_r(RegisterU8::B);
        let new_r1 = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0x10);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn adc_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xFF);
        gameboy.write_instruction(0x0, 0x01);
        gameboy.cpu.flags.set_flag(Flag::C, true);

        // Run test and compare output
        gameboy.adc_hl();
        let new_r1 = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0x10);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn adc_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xFF);
        gameboy.write_instruction(0x0, 0x01);
        gameboy.cpu.flags.set_flag(Flag::C, true);

        // Run test and compare output
        gameboy.adc_n();
        let new_r1 = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0x10);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn sub_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::B, 0x1);

        // Run test and compare output
        gameboy.sub_r(RegisterU8::B);
        let new_r1 = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0xFF);
        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn sub_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x1);

        // Run test and compare output
        gameboy.sub_hl();
        let new_r1 = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0xFF);
        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn sub_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x1);

        // Run test and compare output
        gameboy.sub_n();
        let new_r1 = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0xFF);
        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn sbc_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::B, 0x01);
        gameboy.cpu.flags.set_flag(Flag::C, true);

        // Run test and compare output
        gameboy.sbc_r(RegisterU8::B);
        let new_r1 = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0xEF);
        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn sbc_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x01);
        gameboy.cpu.flags.set_flag(Flag::C, true);

        // Run test and compare output
        gameboy.sbc_hl();
        let new_r1 = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0xEF);
        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn sbc_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x01);
        gameboy.cpu.flags.set_flag(Flag::C, true);

        // Run test and compare output
        gameboy.sbc_n();
        let new_r1 = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0xEF);
        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn cp_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::B, 0x01);

        // Run test and compare output
        gameboy.cp_r(RegisterU8::B);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H);
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn cp_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u16(RegisterU16::HL, 0xFFFA);
        gameboy.write_instruction(0xFFFA, 0x01);

        // Run test and compare output
        gameboy.cp_hl();

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn cp_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x1);

        // Run test and compare output
        gameboy.cp_n();

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn inc_r() {
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
    fn inc_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0xFF);

        // Run test and compare output
        gameboy.inc_hl();
        let data_in_memory = gameboy.read_instruction(0x0);
        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);

        assert_eq!(data_in_memory, 0x0);
        assert_eq!(hc_flag, true);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, true);
    }

    #[test]
    fn dec_r() {
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
    fn dec_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        gameboy.dec_hl();
        let new_r1 = gameboy.read_instruction(0x0);
        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);

        assert_eq!(new_r1, 0xFE);
        assert_eq!(hc_flag, false);
        assert_eq!(n_flag, true);
        assert_eq!(z_flag, false);
    }

    #[test]
    fn and_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xF0);
        gameboy.cpu.register.write_u8(RegisterU8::B, 0x0F);

        // Run test and compare output
        gameboy.and_r(RegisterU8::B);
        let reg_data = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(reg_data, 0x0);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, true);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, false);
    }

    #[test]
    fn and_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xFA);
        gameboy.write_instruction(0x0, 0xCD);

        // Run test and compare output
        gameboy.and_hl();
        let reg_data = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(reg_data, 0xC8);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, false);
    }

    #[test]
    fn and_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xFA);
        gameboy.write_instruction(0x0, 0xCD);

        // Run test and compare output
        gameboy.and_n();
        let reg_data = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(reg_data, 0xC8);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, true);
        assert_eq!(c_flag, false);
    }

    #[test]
    fn xor() {
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
    fn xor_hl() {
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
    fn inc_rr() {
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
    fn jr_e() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.pc = 0x0B;
        gameboy.write_instruction(0x0B, 0xFB);

        // Run test and compare output
        gameboy.jr_e();
        let new_pc = gameboy.cpu.register.pc;
        assert_eq!(new_pc, 0x07);
    }

    #[test]
    fn jr_cc_e() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.pc = 0x0B;
        gameboy.write_instruction(0x0B, 0xFB);
        gameboy.cpu.flags.set_flag(Flag::C, false);

        // Run test and compare output
        gameboy.jr_cc_e(FlagConds::NC);

        let new_pc = gameboy.cpu.register.pc;
        assert_eq!(new_pc, 0x07);
    }

    #[test]
    fn call_nn() {
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
        assert_eq!(msb, 0x0);
        assert_eq!(lsb, 0x02);
    }

    #[test]
    fn ret() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.sp = 0xFFFC;
        gameboy.write_instruction(0xFFFC, 0xFB);
        gameboy.write_instruction(0xFFFC + 1, 0xFA);

        gameboy.ret();

        let new_pc = gameboy.cpu.register.pc;
        assert_eq!(new_pc, 0xFAFB);
    }

    // Rotate instructions tests
    #[test]
    fn rla() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::C, true);
        gameboy.cpu.register.write_u8(r1, 0b1101_0010);

        gameboy.rl_r(r1);
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1010_0101);
        assert_eq!(carry_flag, true);
    }

    // CB prefix tests
    #[test]
    fn rl_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::C, true);
        gameboy.cpu.register.write_u8(r1, 0b1101_0010);

        gameboy.rl_r(r1);
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1010_0101);
        assert_eq!(carry_flag, true);
    }

    #[test]
    fn bit_r() {
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