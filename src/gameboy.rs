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

    fn _half_carry_add_u16(&self, val_1: u16, val_2: u16) -> bool {
        ((val_1 & 0xFFF) + (val_2 & 0xFFF)) & 0x1000 == 0x1000
    }

    fn _half_carry_sub_u16(&self, val_1: u16, val_2: u16) -> bool {
        (val_1 & 0xFFF) < (val_2 & 0xFFF)
    }

    fn _half_carry_add_u8(&self, val_1: u8, val_2: u8) -> bool {
        ((val_1 & 0xF) + (val_2 & 0xF)) & 0x10 == 0x10
    }

    fn _half_carry_sub_u8(&self, val_1: u8, val_2: u8) -> bool {
        (val_1 & 0xF) < (val_2 & 0xF)
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
            0x07 => self.rlca(),
            0x08 => self.ld_nn_sp(),
            0x09 => self.add_hl_rr(RegisterU16::BC),
            0x0A => self.ld_a_rr(RegisterU16::BC),
            0x0B => self.dec_rr(RegisterU16::BC),
            0x0C => self.inc_r(RegisterU8::C),
            0x0D => self.dec_r(RegisterU8::C),
            0x0E => self.ld_r_n(RegisterU8::C),
            0x0F => self.rrca(),

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
            0x19 => self.add_hl_rr(RegisterU16::DE),
            0x1A => self.ld_a_rr(RegisterU16::DE),
            0x1B => self.dec_rr(RegisterU16::DE),
            0x1C => self.inc_r(RegisterU8::E),
            0x1D => self.dec_r(RegisterU8::E),
            0x1E => self.ld_r_n(RegisterU8::E),
            0x1F => self.rra(),

            // 0x2 opcodes
            0x20 => self.jr_cc_e(FlagConds::NZ),
            0x21 => self.ld_rr_nn(RegisterU16::HL),
            0x22 => self.ld_hl_plus_a(),
            0x23 => self.inc_rr(RegisterU16::HL),
            0x24 => self.inc_r(RegisterU8::H),
            0x25 => self.dec_r(RegisterU8::H),
            0x26 => self.ld_r_n(RegisterU8::H),
            0x27 => self.daa(),
            0x28 => self.jr_cc_e(FlagConds::Z),
            0x29 => self.add_hl_rr(RegisterU16::HL),
            0x2A => self.ld_a_hl_plus(),
            0x2B => self.dec_rr(RegisterU16::HL),
            0x2C => self.inc_r(RegisterU8::L),
            0x2D => self.dec_r(RegisterU8::L),
            0x2E => self.ld_r_n(RegisterU8::L),
            0x2F => self.cpl(),

            // 0x3 opcodes
            0x30 => self.jr_cc_e(FlagConds::NC),
            0x31 => self.ld_rr_nn(RegisterU16::SP),
            0x32 => self.ld_hl_minus_a(),
            0x33 => self.inc_rr(RegisterU16::SP),
            0x34 => self.inc_hl(),
            0x35 => self.dec_hl(),
            0x36 => self.ld_hl_n(),
            0x37 => self.scf(),
            0x38 => self.jr_cc_e(FlagConds::C),
            0x39 => self.add_hl_rr(RegisterU16::SP),
            0x3A => self.ld_a_hl_minus(),
            0x3B => self.dec_rr(RegisterU16::SP),
            0x3C => self.inc_r(RegisterU8::A),
            0x3D => self.dec_r(RegisterU8::A),
            0x3E => self.ld_r_n(RegisterU8::A),
            0x3F => self.ccf(),

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
            0xA8 => self.xor_r(RegisterU8::B),
            0xA9 => self.xor_r(RegisterU8::C),
            0xAA => self.xor_r(RegisterU8::D),
            0xAB => self.xor_r(RegisterU8::E),
            0xAC => self.xor_r(RegisterU8::H),
            0xAD => self.xor_r(RegisterU8::L),
            0xAE => self.xor_hl(),
            0xAF => self.xor_r(RegisterU8::A),

            //0xB opcodes
            0xB0 => self.or_r(RegisterU8::B),
            0xB1 => self.or_r(RegisterU8::C),
            0xB2 => self.or_r(RegisterU8::D),
            0xB3 => self.or_r(RegisterU8::E),
            0xB4 => self.or_r(RegisterU8::H),
            0xB5 => self.or_r(RegisterU8::L),
            0xB6 => self.or_hl(),
            0xB7 => self.or_r(RegisterU8::A),
            0xB8 => self.cp_r(RegisterU8::B),
            0xB9 => self.cp_r(RegisterU8::C),
            0xBA => self.cp_r(RegisterU8::D),
            0xBB => self.cp_r(RegisterU8::E),
            0xBC => self.cp_r(RegisterU8::H),
            0xBD => self.cp_r(RegisterU8::L),
            0xBE => self.cp_hl(),
            0xBF => self.cp_r(RegisterU8::A),

            //0xC opcodes
            0xC0 => self.ret_cc(FlagConds::NZ),
            0xC1 => self.pop(RegisterU16::BC),
            0xC2 => self.jp_cc_nn(FlagConds::NZ),
            0xC3 => self.jp_nn(),
            0xC4 => self.call_cc_nn(FlagConds::NZ),
            0xC5 => self.push(RegisterU16::BC),
            0xC6 => self.add_n(),
            0xC7 => self.rst_n(0x0),
            0xC8 => self.ret_cc(FlagConds::Z),
            0xC9 => self.ret(),
            0xCA => self.jp_cc_nn(FlagConds::Z),
            0xCB => self.cb_prefix(),
            0xCC => self.jp_cc_nn(FlagConds::Z),
            0xCD => self.call_nn(),
            0xCE => self.adc_n(),
            0xCF => self.rst_n(0x08),

            //0xD opcodes
            0xD0 => self.ret_cc(FlagConds::NC),
            0xD1 => self.pop(RegisterU16::DE),
            0xD2 => self.jp_cc_nn(FlagConds::NC),
            0xD3 => panic!("Illegal Opcode: {:#X}", opcode),
            0xD4 => self.call_cc_nn(FlagConds::NC),
            0xD5 => self.push(RegisterU16::DE),
            0xD6 => self.sub_n(),
            0xD7 => self.rst_n(0x10),
            0xD8 => self.ret_cc(FlagConds::C),
            0xD9 => todo!(),
            0xDA => self.jp_cc_nn(FlagConds::C),
            0xDB => panic!("Illegal Opcode: {:#X}", opcode),
            0xDC => self.call_cc_nn(FlagConds::C),
            0xDD => panic!("Illegal Opcode: {:#X}", opcode),
            0xDE => self.sbc_n(),
            0xDF => self.rst_n(0x18),

            //0xE opcodes
            0xE0 => self.ldh_n_a(),
            0xE1 => self.pop(RegisterU16::HL),
            0xE2 => self.ldh_c_a(),
            0xE3 => panic!("Illegal Opcode: {:#X}", opcode),
            0xE4 => panic!("Illegal Opcode: {:#X}", opcode),
            0xE5 => self.push(RegisterU16::HL),
            0xE6 => self.and_n(),
            0xE7 => self.rst_n(0x20),
            0xE8 => self.add_sp_e(),
            0xE9 => self.jp_hl(),
            0xEA => self.ld_nn_a(),
            0xEB => panic!("Illegal Opcode: {:#X}", opcode),
            0xEC => panic!("Illegal Opcode: {:#X}", opcode),
            0xED => panic!("Illegal Opcode: {:#X}", opcode),
            0xEE => self.xor_n(),
            0xEF => self.rst_n(0x28),

            //0xF opcodes
            0xF0 => self.ldh_a_n(),
            0xF1 => self.pop(RegisterU16::AF),
            0xF2 => self.ldh_a_c(),
            0xF3 => todo!(),
            0xF4 => panic!("Illegal Opcode: {:#X}", opcode),
            0xF5 => self.push(RegisterU16::AF),
            0xF6 => self.or_n(),
            0xF7 => self.rst_n(0x30),
            0xF8 => self.ld_hl_sp_e(),
            0xF9 => self.ld_sp_hl(),
            0xFA => self.ld_a_nn(),
            0xFB => todo!(),
            0xFC => panic!("Illegal Opcode: {:#X}", opcode),
            0xFD => panic!("Illegal Opcode: {:#X}", opcode),
            0xFE => self.cp_n(),
            0xFF => self.rst_n(0x38),
        }
    }

fn cb_prefix(&mut self) {
    let cb_code = self.read_instruction(self.cpu.register.pc);
    self.cpu.register.pc += 1;

    match cb_code {
        // 0x0 codes
        0x00 => self.rlc_r(RegisterU8::B),
        0x01 => self.rlc_r(RegisterU8::C),
        0x02 => self.rlc_r(RegisterU8::D),
        0x03 => self.rlc_r(RegisterU8::E),
        0x04 => self.rlc_r(RegisterU8::H),
        0x05 => self.rlc_r(RegisterU8::L),
        0x06 => self.rlc_hl(),
        0x07 => self.rlc_r(RegisterU8::A),
        0x08 => self.rrc_r(RegisterU8::B),
        0x09 => self.rrc_r(RegisterU8::C),
        0x0A => self.rrc_r(RegisterU8::D),
        0x0B => self.rrc_r(RegisterU8::E),
        0x0C => self.rrc_r(RegisterU8::H),
        0x0D => self.rrc_r(RegisterU8::L),
        0x0E => self.rrc_hl(),
        0x0F => self.rrc_r(RegisterU8::A),

        // 0x1 codes
        0x10 => self.rl_r(RegisterU8::B),
        0x11 => self.rl_r(RegisterU8::C),
        0x12 => self.rl_r(RegisterU8::D),
        0x13 => self.rl_r(RegisterU8::E),
        0x14 => self.rl_r(RegisterU8::H),
        0x15 => self.rl_r(RegisterU8::L),
        0x16 => self.rl_hl(),
        0x17 => self.rl_r(RegisterU8::A),
        0x18 => self.rr_r(RegisterU8::B),
        0x19 => self.rr_r(RegisterU8::C),
        0x1A => self.rr_r(RegisterU8::D),
        0x1B => self.rr_r(RegisterU8::E),
        0x1C => self.rr_r(RegisterU8::H),
        0x1D => self.rr_r(RegisterU8::L),
        0x1E => todo!(),
        0x1F => self.rr_r(RegisterU8::A),

        // 0x4 codes
        0x40 => self.bit_r(RegisterU8::B, 0),
        0x41 => self.bit_r(RegisterU8::C, 0),
        0x42 => self.bit_r(RegisterU8::D, 0),
        0x43 => self.bit_r(RegisterU8::E, 0),
        0x44 => self.bit_r(RegisterU8::H, 0),
        0x45 => self.bit_r(RegisterU8::L, 0),
        0x46 => self.bit_hl(0),
        0x47 => self.bit_r(RegisterU8::A, 0),
        0x48 => self.bit_r(RegisterU8::B, 1),
        0x49 => self.bit_r(RegisterU8::C, 1),
        0x4A => self.bit_r(RegisterU8::D, 1),
        0x4B => self.bit_r(RegisterU8::E, 1),
        0x4C => self.bit_r(RegisterU8::H, 1),
        0x4D => self.bit_r(RegisterU8::L, 1),
        0x4E => self.bit_hl(1),
        0x4F => self.bit_r(RegisterU8::A, 1),

        // 0x5 codes
        0x50 => self.bit_r(RegisterU8::B, 2),
        0x51 => self.bit_r(RegisterU8::C, 2),
        0x52 => self.bit_r(RegisterU8::D, 2),
        0x53 => self.bit_r(RegisterU8::E, 2),
        0x54 => self.bit_r(RegisterU8::H, 2),
        0x55 => self.bit_r(RegisterU8::L, 2),
        0x56 => self.bit_hl(2),
        0x57 => self.bit_r(RegisterU8::A, 2),
        0x58 => self.bit_r(RegisterU8::B, 3),
        0x59 => self.bit_r(RegisterU8::C, 3),
        0x5A => self.bit_r(RegisterU8::D, 3),
        0x5B => self.bit_r(RegisterU8::E, 3),
        0x5C => self.bit_r(RegisterU8::H, 3),
        0x5D => self.bit_r(RegisterU8::L, 3),
        0x5E => self.bit_hl(3),
        0x5F => self.bit_r(RegisterU8::A, 3),

        // 0x6 codes
        0x60 => self.bit_r(RegisterU8::B, 4),
        0x61 => self.bit_r(RegisterU8::C, 4),
        0x62 => self.bit_r(RegisterU8::D, 4),
        0x63 => self.bit_r(RegisterU8::E, 4),
        0x64 => self.bit_r(RegisterU8::H, 4),
        0x65 => self.bit_r(RegisterU8::L, 4),
        0x66 => self.bit_hl(4),
        0x67 => self.bit_r(RegisterU8::A, 6),
        0x68 => self.bit_r(RegisterU8::B, 5),
        0x69 => self.bit_r(RegisterU8::C, 5),
        0x6A => self.bit_r(RegisterU8::D, 5),
        0x6B => self.bit_r(RegisterU8::E, 5),
        0x6C => self.bit_r(RegisterU8::H, 5),
        0x6D => self.bit_r(RegisterU8::L, 5),
        0x6E => self.bit_hl(5),
        0x6F => self.bit_r(RegisterU8::A, 5),

        // 0x7 codes
        0x70 => self.bit_r(RegisterU8::B, 6),
        0x71 => self.bit_r(RegisterU8::C, 6),
        0x72 => self.bit_r(RegisterU8::D, 6),
        0x73 => self.bit_r(RegisterU8::E, 6),
        0x74 => self.bit_r(RegisterU8::H, 6),
        0x75 => self.bit_r(RegisterU8::L, 6),
        0x76 => self.bit_hl(6),
        0x77 => self.bit_r(RegisterU8::A, 6),
        0x78 => self.bit_r(RegisterU8::B, 7),
        0x79 => self.bit_r(RegisterU8::C, 7),
        0x7A => self.bit_r(RegisterU8::D, 7),
        0x7B => self.bit_r(RegisterU8::E, 7),
        0x7C => self.bit_r(RegisterU8::H, 7),
        0x7D => self.bit_r(RegisterU8::L, 7),
        0x7E => self.bit_hl(7),
        0x7F => self.bit_r(RegisterU8::A, 7),

        _ => panic!("CB Prefixed code not implemented: {:#X}", cb_code)
    }
}

    // CPU instructions
    // Instructions intepreted from https://gekkio.fi/files/gb-docs/gbctr.pdf
    // and https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/
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

    fn ld_hl_n(&mut self) {
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

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
        self.cpu.register.update_f_reg(self.cpu.flags);
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
        self.cpu.register.update_f_reg(self.cpu.flags);
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
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn or_r(&mut self, r1: RegisterU8) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let reg_data = self.cpu.register.read_u8(r1);

        let result = reg_a | reg_data;

        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.flags.set_flag(Flag::C, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn or_hl(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);

        let result = reg_a | data;

        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.flags.set_flag(Flag::C, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn or_n(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let data = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let result = reg_a | data;

        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.flags.set_flag(Flag::C, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn xor_r(&mut self, r1: RegisterU8) {
        let reg_data = self.cpu.register.read_u8(r1);
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);

        let result = reg_a ^ reg_data;
        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
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
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.flags.set_flag(Flag::C, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn xor_n(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);
        let data = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let result = reg_a ^ data;
        self.cpu.register.write_u8(RegisterU8::A, result);

        if result == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.flags.set_flag(Flag::C, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn ccf(&mut self) {
        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);

        let carry_flag = self.cpu.flags.get_flag(Flag::C);

        if carry_flag {
            self.cpu.flags.set_flag(Flag::C, false);
        }
        else {
            self.cpu.flags.set_flag(Flag::C, true);
        }

        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn scf(&mut self) {
        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.flags.set_flag(Flag::C, true);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    // Need to check line 1360 ||
    fn daa(&mut self) {
        let mut reg_a = self.cpu.register.read_u8(RegisterU8::A);

        let n_flag = self.cpu.flags.get_flag(Flag::N);
        let c_flag = self.cpu.flags.get_flag(Flag::C);
        let h_flag = self.cpu.flags.get_flag(Flag::H);

        if !n_flag {
            if c_flag || reg_a > 0x99 {
                reg_a = reg_a.wrapping_add(0x60);
                self.cpu.flags.set_flag(Flag::C, true);
            }
            if h_flag || (reg_a & 0x0F) > 0x09 {
                reg_a = reg_a.wrapping_add(0x6);
            }
        }
        else {
            if c_flag { reg_a = reg_a.wrapping_sub(0x60); }
            if h_flag { reg_a = reg_a.wrapping_sub(0x6); }
        }

        if reg_a == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.register.write_u8(RegisterU8::A, reg_a);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn cpl(&mut self) {
        let mut reg_a = self.cpu.register.read_u8(RegisterU8::A);

        reg_a = reg_a ^ 0xFF;
        self.cpu.register.write_u8(RegisterU8::A, reg_a);

        self.cpu.flags.set_flag(Flag::N, true);
        self.cpu.flags.set_flag(Flag::H, true);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    // 16 bit ALU
    fn add_hl_rr(&mut self, r1: RegisterU16) {
        let reg_hl = self.cpu.register.read_u16(RegisterU16::HL);
        let reg_data = self.cpu.register.read_u16(r1);

        let (result, carry_flag) = reg_hl.overflowing_add(reg_data);

        let half_carry = self._half_carry_add_u16(reg_hl, reg_data);

        self.cpu.register.write_u16(RegisterU16::HL, result);

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn inc_rr(&mut self, r1: RegisterU16) {
        let reg_data = self.cpu.register.read_u16(r1);
        let reg_data = reg_data.wrapping_add(0x01);

        self.cpu.register.write_u16(r1, reg_data);
    }

    fn dec_rr(&mut self, r1: RegisterU16) {
        let reg_data = self.cpu.register.read_u16(r1);
        let reg_data = reg_data.wrapping_sub(0x01);

        self.cpu.register.write_u16(r1, reg_data);
    }

    fn add_sp_e(&mut self) {
        let offset = self.read_instruction(self.cpu.register.pc) as i8;
        self.cpu.register.pc += 1;
        let new_sp = self.cpu.register.sp;

        let half_carry_flag = self._half_carry_add_u16(new_sp, offset as u16);
        let (new_sp, carry_flag) = new_sp.overflowing_add_signed(offset as i16);

        self.cpu.register.sp = new_sp;

        self.cpu.flags.set_flag(Flag::Z, false);
        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    // Essentially the same as add_sp_e but saves the result to the HL register
    // Not sure if there is a more elegant way than repeating code
    fn ld_hl_sp_e(&mut self) {
        let offset = self.read_instruction(self.cpu.register.pc) as i8;
        self.cpu.register.pc += 1;
        let new_sp = self.cpu.register.sp;

        let half_carry_flag = self._half_carry_add_u16(new_sp, offset as u16);
        let (new_sp, carry_flag) = new_sp.overflowing_add_signed(offset as i16);

        self.cpu.register.write_u16(RegisterU16::HL, new_sp);

        self.cpu.flags.set_flag(Flag::Z, false);
        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, half_carry_flag);
        self.cpu.flags.set_flag(Flag::C, carry_flag);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    // Control flow instructions
    fn jp_nn(&mut self) {
        let lsb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let msb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let nn = (msb as u16) << 8 | lsb as u16;
        self.cpu.register.pc = nn;
    }

    fn jp_hl(&mut self) {
        self.cpu.register.pc = self.cpu.register.read_u16(RegisterU16::HL);
    }

    fn jp_cc_nn(&mut self, jp_cond: FlagConds) {
        let lsb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let msb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let nn = (msb as u16) << 8 | lsb as u16;

        match jp_cond {
            FlagConds::NZ => {
                if !self.cpu.flags.get_flag(Flag::Z) {
                    self.cpu.register.pc = nn;
                }
            },

            FlagConds::Z => {
                if self.cpu.flags.get_flag(Flag::Z) {
                    self.cpu.register.pc = nn;
                }
            },

            FlagConds::NC => {
                if !self.cpu.flags.get_flag(Flag::C) {
                    self.cpu.register.pc = nn;
                }
            },

            FlagConds::C => {
                if self.cpu.flags.get_flag(Flag::C) {
                    self.cpu.register.pc = nn;
                }
            }
        }
    }

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
            }
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

    fn call_cc_nn(&mut self, jp_cond: FlagConds) {
        let lsb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let msb = self.read_instruction(self.cpu.register.pc);
        self.cpu.register.pc += 1;

        let nn = (msb as u16) << 8 | lsb as u16;

        let [lsb_pc, msb_pc] = self.cpu.register.pc.to_le_bytes();

        match jp_cond {
            FlagConds::NZ => {
                if !self.cpu.flags.get_flag(Flag::Z) {
                    self.cpu.register.sp -= 1;
                    self.write_instruction(self.cpu.register.sp, msb_pc);
                    self.cpu.register.sp -= 1;
                    self.write_instruction(self.cpu.register.sp, lsb_pc);
                    self.cpu.register.pc = nn;
                }
            },

            FlagConds::Z => {
                if self.cpu.flags.get_flag(Flag::Z) {
                    self.cpu.register.sp -= 1;
                    self.write_instruction(self.cpu.register.sp, msb_pc);
                    self.cpu.register.sp -= 1;
                    self.write_instruction(self.cpu.register.sp, lsb_pc);
                    self.cpu.register.pc = nn;
                }
            },

            FlagConds::NC => {
                if !self.cpu.flags.get_flag(Flag::C) {
                    self.cpu.register.sp -= 1;
                    self.write_instruction(self.cpu.register.sp, msb_pc);
                    self.cpu.register.sp -= 1;
                    self.write_instruction(self.cpu.register.sp, lsb_pc);
                    self.cpu.register.pc = nn;
                }
            },

            FlagConds::C => {
                if self.cpu.flags.get_flag(Flag::C) {
                    self.cpu.register.sp -= 1;
                    self.write_instruction(self.cpu.register.sp, msb_pc);
                    self.cpu.register.sp -= 1;
                    self.write_instruction(self.cpu.register.sp, lsb_pc);
                    self.cpu.register.pc = nn;
                }
            }
        }
    }

    fn ret(&mut self) {
        let lsb = self.read_instruction(self.cpu.register.sp);
        self.cpu.register.sp += 1;
        
        let msb = self.read_instruction(self.cpu.register.sp);
        self.cpu.register.sp += 1;

        self.cpu.register.pc = (msb as u16) << 8 | lsb as u16;
    }

    fn ret_cc(&mut self, jp_cond: FlagConds) {
        match jp_cond {
            FlagConds::NZ => {
                if !self.cpu.flags.get_flag(Flag::Z) {
                    let lsb = self.read_instruction(self.cpu.register.sp);
                    self.cpu.register.sp += 1;
                    
                    let msb = self.read_instruction(self.cpu.register.sp);
                    self.cpu.register.sp += 1;
            
                    self.cpu.register.pc = (msb as u16) << 8 | lsb as u16;
                }
            },

            FlagConds::Z => {
                if self.cpu.flags.get_flag(Flag::Z) {
                    let lsb = self.read_instruction(self.cpu.register.sp);
                    self.cpu.register.sp += 1;
                    
                    let msb = self.read_instruction(self.cpu.register.sp);
                    self.cpu.register.sp += 1;
            
                    self.cpu.register.pc = (msb as u16) << 8 | lsb as u16;
                }
            },

            FlagConds::NC => {
                if !self.cpu.flags.get_flag(Flag::C) {
                    let lsb = self.read_instruction(self.cpu.register.sp);
                    self.cpu.register.sp += 1;
                    
                    let msb = self.read_instruction(self.cpu.register.sp);
                    self.cpu.register.sp += 1;
            
                    self.cpu.register.pc = (msb as u16) << 8 | lsb as u16;
                }
            },

            FlagConds::C => {
                if self.cpu.flags.get_flag(Flag::C) {
                    let lsb = self.read_instruction(self.cpu.register.sp);
                    self.cpu.register.sp += 1;
                    
                    let msb = self.read_instruction(self.cpu.register.sp);
                    self.cpu.register.sp += 1;
            
                    self.cpu.register.pc = (msb as u16) << 8 | lsb as u16;
                }
            }
        }
    }

    fn rst_n(&mut self, jp_addr: u8) {
        let [lsb_pc, msb_pc] = self.cpu.register.pc.to_le_bytes();

        self.cpu.register.sp -= 1;
        self.write_instruction(self.cpu.register.sp, msb_pc);
        self.cpu.register.sp -= 1;
        self.write_instruction(self.cpu.register.sp, lsb_pc);
        self.cpu.register.pc = jp_addr as u16;
    }


    // Rotate, shift and bit operations
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

    fn rlca(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);

        let new_carry_flag: bool = (reg_a & 0b1000_0000) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let rot_a = reg_a.rotate_left(1);
        self.cpu.register.write_u8(RegisterU8::A, rot_a);

        self.cpu.flags.set_flag(Flag::Z, false);
        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn rlc_r(&mut self, r1: RegisterU8) {
        let reg_data =  self.cpu.register.read_u8(r1);

        let new_carry_flag: bool = (reg_data & 0b1000_0000) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let rot_data = reg_data.rotate_left(1);
        self.cpu.register.write_u8(r1, rot_data);

        if rot_data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn rlc_hl(&mut self) {
        let address =  self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);

        let new_carry_flag: bool = (data & 0b1000_0000) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let rot_data = data.rotate_left(1);
        self.write_instruction(address, rot_data);

        if rot_data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

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

    fn rl_hl(&mut self) {
        let carry_flag = self.cpu.flags.get_flag(Flag::C);
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);

        let new_carry_flag: bool = (data & 0b1000_0000) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let mut new_data = data << 1;
        new_data = new_data & 0b1111_1110;

        if carry_flag {
            new_data += 1;
            self.write_instruction(address, new_data);
        }
        else {
            self.write_instruction(address, new_data);
        }

        if new_data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn rra(&mut self) {
        let carry_flag = self.cpu.flags.get_flag(Flag::C);
        let reg_data = self.cpu.register.read_u8(RegisterU8::A);

        let new_carry_flag: bool = (reg_data & 0b0000_0001) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let mut new_reg_data = reg_data >> 1;
        new_reg_data = new_reg_data & 0b0111_1111;

        if carry_flag {
            new_reg_data += 0b1000_0000;
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

    fn rrca(&mut self) {
        let reg_a = self.cpu.register.read_u8(RegisterU8::A);

        let new_carry_flag: bool = (reg_a & 0b0000_0001) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let rot_a = reg_a.rotate_right(1);
        self.cpu.register.write_u8(RegisterU8::A, rot_a);

        self.cpu.flags.set_flag(Flag::Z, false);
        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn rrc_r(&mut self, r1: RegisterU8) {
        let reg_data =  self.cpu.register.read_u8(r1);

        let new_carry_flag: bool = (reg_data & 0b0000_0001) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let rot_data = reg_data.rotate_right(1);
        self.cpu.register.write_u8(r1, rot_data);

        if rot_data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn rrc_hl(&mut self) {
        let address =  self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);

        let new_carry_flag: bool = (data & 0b0000_0001) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let rot_data = data.rotate_right(1);
        self.write_instruction(address, rot_data);

        if rot_data == 0 {
            self.cpu.flags.set_flag(Flag::Z, true);
        }
        else {
            self.cpu.flags.set_flag(Flag::Z, false);
        }

        self.cpu.flags.set_flag(Flag::N, false);
        self.cpu.flags.set_flag(Flag::H, false);
        self.cpu.register.update_f_reg(self.cpu.flags);
    }

    fn rr_r(&mut self, r1: RegisterU8) {
        let carry_flag = self.cpu.flags.get_flag(Flag::C);
        let reg_data = self.cpu.register.read_u8(r1);

        let new_carry_flag: bool = (reg_data & 0b0000_0001) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let mut new_reg_data = reg_data >> 1;
        new_reg_data = new_reg_data & 0b0111_1111;

        if carry_flag {
            new_reg_data += 0b1000_0000;
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

    fn rr_hl(&mut self) {
        let carry_flag = self.cpu.flags.get_flag(Flag::C);
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let data = self.read_instruction(address);

        let new_carry_flag: bool = (data & 0b0000_0001) != 0;
        self.cpu.flags.set_flag(Flag::C, new_carry_flag);

        let mut new_data = data >> 1;
        new_data = new_data & 0b0111_1111;

        if carry_flag {
            new_data += 0b1000_0000;
            self.write_instruction(address, new_data);
        }
        else {
            self.write_instruction(address, new_data);
        }

        if new_data == 0 {
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

    fn bit_hl(&mut self, check_bit: u8) {
        let mask: u8 = 1;
        let address = self.cpu.register.read_u16(RegisterU16::HL);
        let mut data = self.read_instruction(address);
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
    fn ld_hl_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u16(RegisterU16::HL, 0x1111);
        gameboy.write_instruction(0x0, 0x01);

        // Run test and compare output
        gameboy.ld_hl_n();
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
    fn or_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xF0);
        gameboy.cpu.register.write_u8(RegisterU8::B, 0x0F);

        // Run test and compare output
        gameboy.or_r(RegisterU8::B);
        let reg_data = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(reg_data, 0xFF);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, false);
        assert_eq!(c_flag, false);
    }

    #[test]
    fn or_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xF0);
        gameboy.write_instruction(0x0, 0x0F);

        // Run test and compare output
        gameboy.or_hl();
        let reg_data = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(reg_data, 0xFF);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, false);
        assert_eq!(c_flag, false);
    }

    #[test]
    fn or_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0xF0);
        gameboy.write_instruction(0x0, 0x0F);

        // Run test and compare output
        gameboy.or_n();
        let reg_data = gameboy.cpu.register.read_u8(RegisterU8::A);

        let hc_flag = gameboy.cpu.flags.get_flag(Flag::H); 
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let z_flag = gameboy.cpu.flags.get_flag(Flag::Z);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(reg_data, 0xFF);
        assert_eq!(n_flag, false);
        assert_eq!(z_flag, false);
        assert_eq!(hc_flag, false);
        assert_eq!(c_flag, false);
    }

    #[test]
    fn xor_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::B;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(r1, 0xFF);

        // Run test and compare output
        gameboy.xor_r(r1);
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

    #[test]
    fn xor_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(r1, 0xFF);
        gameboy.write_instruction(0x0, 0xF0);

        // Run test and compare output
        gameboy.xor_n();

        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let flag_check = gameboy.cpu.flags.get_flag(Flag::Z);

        assert_eq!(new_r1, 0x0F);
        assert_eq!(flag_check, false);
    }

    #[test]
    fn ccf() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::N, true);
        gameboy.cpu.flags.set_flag(Flag::H, true);
        gameboy.cpu.flags.set_flag(Flag::C, true);

        gameboy.ccf();

        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let h_flag = gameboy.cpu.flags.get_flag(Flag::H);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(n_flag, false);
        assert_eq!(h_flag, false);
        assert_eq!(c_flag, false);
    }

    #[test]
    fn scf() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::N, true);
        gameboy.cpu.flags.set_flag(Flag::H, true);
        gameboy.cpu.flags.set_flag(Flag::C, false);

        gameboy.scf();

        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let h_flag = gameboy.cpu.flags.get_flag(Flag::H);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(n_flag, false);
        assert_eq!(h_flag, false);
        assert_eq!(c_flag, true);
    }

    #[test]
    fn cpl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(RegisterU8::A, 0b1010_0101);

        // Run test and compare output
        gameboy.cpl();

        let reg_a = gameboy.cpu.register.read_u8(RegisterU8::A);
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let h_flag = gameboy.cpu.flags.get_flag(Flag::H);

        assert_eq!(reg_a, 0b0101_1010);
        assert_eq!(n_flag, true);
        assert_eq!(h_flag, true);
    }

    // 16 bit ALU test
    #[test]
    fn add_hl_rr() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU16::HL;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u16(r1, 0xFFFF);
        gameboy.cpu.register.write_u16(RegisterU16::DE, 1);

        // Run test and compare output
        gameboy.add_hl_rr(RegisterU16::DE);

        let new_r1 = gameboy.cpu.register.read_u16(r1);
        let n_flag = gameboy.cpu.flags.get_flag(Flag::N);
        let h_flag = gameboy.cpu.flags.get_flag(Flag::H);
        let c_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0x0);
        assert_eq!(n_flag, false);
        assert_eq!(h_flag, true);
        assert_eq!(c_flag, true);
    }


    #[test]
    fn inc_rr() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU16::DE;

        // Run test and compare output
        gameboy.inc_rr(r1);
        let reg_data = gameboy.cpu.register.read_u16(r1);

        assert_eq!(reg_data, 0x1);
    }

    #[test]
    fn dec_rr() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU16::DE;

        // Run test and compare output
        gameboy.dec_rr(r1);
        let reg_data = gameboy.cpu.register.read_u16(r1);

        assert_eq!(reg_data, 0xFFFF);
    }

    #[test]
    fn add_sp_e() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x1);
        gameboy.cpu.register.sp = 0xFFFF;

        // Run test and compare output
        gameboy.add_sp_e();

        let new_sp = gameboy.cpu.register.sp;
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);
        let half_carry_flag = gameboy.cpu.flags.get_flag(Flag::H);
        assert_eq!(new_sp, 0x0);
        assert_eq!(carry_flag, true);
        assert_eq!(half_carry_flag, true);
    }

    #[test]
    fn ld_hl_sp_e() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x1);
        gameboy.cpu.register.sp = 0xFFFD;

        // Run test and compare output
        gameboy.ld_hl_sp_e();

        let reg_hl = gameboy.cpu.register.read_u16(RegisterU16::HL);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);
        let half_carry_flag = gameboy.cpu.flags.get_flag(Flag::H);
        assert_eq!(reg_hl, 0xFFFE);
        assert_eq!(carry_flag, false);
        assert_eq!(half_carry_flag, false);
    }

    // Control flow tests
    #[test]
    fn jp_nn() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0xFB);
        gameboy.write_instruction(0x1, 0xFA);

        // Run test and compare output
        gameboy.jp_nn();

        let new_pc = gameboy.cpu.register.pc;
        assert_eq!(new_pc, 0xFAFB);
    }

    #[test]
    fn jp_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.write_u16(RegisterU16::HL, 0xFFFB);

        // Run test and compare output
        gameboy.jp_hl();

        let new_pc = gameboy.cpu.register.pc;
        assert_eq!(new_pc, 0xFFFB);
    }

    #[test]
    fn jp_cc_nn() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0xFB);
        gameboy.write_instruction(0x1, 0xFA);
        gameboy.cpu.flags.set_flag(Flag::Z, false);

        // Run test and compare output
        gameboy.jp_cc_nn(FlagConds::NZ);

        let new_pc = gameboy.cpu.register.pc;
        assert_eq!(new_pc, 0xFAFB);
    }

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
    fn call_cc_nn() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x01);
        gameboy.write_instruction(0x01, 0x02);
        gameboy.cpu.register.sp = 0xFFFE;
        gameboy.cpu.flags.set_flag(Flag::C, false);

        // Run test and compare output
        gameboy.call_cc_nn(FlagConds::NC);

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

    #[test]
    fn ret_cc() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.sp = 0xFFFC;
        gameboy.write_instruction(0xFFFC, 0xFB);
        gameboy.write_instruction(0xFFFC + 1, 0xFA);
        gameboy.cpu.flags.set_flag(Flag::C, false);
        gameboy.ret_cc(FlagConds::NC);

        let new_pc = gameboy.cpu.register.pc;
        assert_eq!(new_pc, 0xFAFB);
    }

    #[test]
    fn rst_n() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.register.pc = 0x1011;
        gameboy.cpu.register.sp = 0xFFFE;

        // Run test and compare output
        gameboy.rst_n(0x38);

        let new_sp = gameboy.cpu.register.sp;
        let new_pc = gameboy.cpu.register.pc;
        let msb = gameboy.read_instruction(0xFFFE - 1);
        let lsb = gameboy.read_instruction(0xFFFE - 2);

        assert_eq!(new_sp, 0xFFFC);
        assert_eq!(new_pc, 0x0038);
        assert_eq!(msb, 0x10);
        assert_eq!(lsb, 0x11);
    }

    // Rotate, shift and bit operations tests
    #[test]
    fn rla() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::C, true);
        gameboy.cpu.register.write_u8(r1, 0b1101_0010);

        // Run test and compare output
        gameboy.rla();
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1010_0101);
        assert_eq!(carry_flag, true);
    }

    #[test]
    fn rlca() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(r1, 0b1101_0010);

        // Run test and compare output
        gameboy.rlca();
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1010_0101);
        assert_eq!(carry_flag, true);
    }

    #[test]
    fn rlc_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::B;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(r1, 0b1101_0010);

        // Run test and compare output
        gameboy.rlc_r(r1);
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1010_0101);
        assert_eq!(carry_flag, true);
    }

    #[test]
    fn rlc_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::C, true);
        gameboy.write_instruction(0x0, 0b1101_0010);

        // Run test and compare output
        gameboy.rlc_hl();
        let new_r1 = gameboy.read_instruction(0x0);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1010_0101);
        assert_eq!(carry_flag, true);
    }

    #[test]
    fn rl_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::C, true);
        gameboy.cpu.register.write_u8(r1, 0b1101_0010);

        // Run test and compare output
        gameboy.rl_r(r1);
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1010_0101);
        assert_eq!(carry_flag, true);
    }

    #[test]
    fn rl_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::C, true);
        gameboy.write_instruction(0x0, 0b1101_0010);

        // Run test and compare output
        gameboy.rl_hl();
        let new_r1 = gameboy.read_instruction(0x0);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1010_0101);
        assert_eq!(carry_flag, true);
    }

    #[test]
    fn rra() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::C, true);
        gameboy.cpu.register.write_u8(r1, 0b1101_0010);

        // Run test and compare output
        gameboy.rra();
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1110_1001);
        assert_eq!(carry_flag, false);
    }

    #[test]
    fn rrc_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::B;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(r1, 0b1101_0010);

        // Run test and compare output
        gameboy.rrc_r(r1);
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b0110_1001);
        assert_eq!(carry_flag, false);
    }

    #[test]
    fn rrc_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::C, true);
        gameboy.write_instruction(0x0, 0b1101_0010);

        // Run test and compare output
        gameboy.rrc_hl();
        let new_r1 = gameboy.read_instruction(0x0);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b0110_1001);
        assert_eq!(carry_flag, false);
    }

    #[test]
    fn rrca() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::A;

        // Set up gameboy state for test
        gameboy.cpu.register.write_u8(r1, 0b1101_0010);

        // Run test and compare output
        gameboy.rrca();
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b0110_1001);
        assert_eq!(carry_flag, false);
    }

    #[test]
    fn rr_r() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();
        let r1 = RegisterU8::B;

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::C, true);
        gameboy.cpu.register.write_u8(r1, 0b1101_0010);

        // Run test and compare output
        gameboy.rr_r(r1);
        let new_r1 = gameboy.cpu.register.read_u8(r1);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1110_1001);
        assert_eq!(carry_flag, false);
    }

    #[test]
    fn rr_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.cpu.flags.set_flag(Flag::C, true);
        gameboy.write_instruction(0x0, 0b1101_0010);

        // Run test and compare output
        gameboy.rr_hl();
        let new_r1 = gameboy.read_instruction(0x0);
        let carry_flag = gameboy.cpu.flags.get_flag(Flag::C);

        assert_eq!(new_r1, 0b1110_1001);
        assert_eq!(carry_flag, false);
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

    #[test]
    fn bit_hl() {
        // Create a gameboy for testing purposes
        let mut gameboy = Gameboy::new();

        // Set up gameboy state for test
        gameboy.write_instruction(0x0, 0x9F);

        // Run test and compare output
        gameboy.bit_hl(6);

        assert_eq!(gameboy.cpu.flags.get_flag(Flag::Z), true);
        assert_eq!(gameboy.cpu.flags.get_flag(Flag::N), false);
        assert_eq!(gameboy.cpu.flags.get_flag(Flag::H), true);
    }

}