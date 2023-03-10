use std::{convert::From};

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

// Building CPU up
pub struct CPU {
    pub register: Registers,
    pub flags: FlagsRegister
}

// Initialising CPU with zero values
impl CPU {
    pub fn new() -> CPU {
        let cpu = CPU {
            register: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                f: 0,
                e: 0,
                h: 0,
                l: 0,
                pc: 0,
                sp: 0,
            },
            flags: FlagsRegister { z: false, n: false, h: false, c: false }
        };
        cpu
    }
}

pub struct FlagsRegister {
    // Zero flag
    z: bool,
    // Add/Subtract flag
    n: bool,
    // Half-carry flag
    h: bool,
    // Carry flag
    c: bool
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    pub pc: u16,
    pub sp: u16
}

#[derive(Copy, Clone)]
pub enum RegisterU8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L
}

#[derive(Copy, Clone)]
pub enum RegisterU16 {
    AF,
    BC,
    DE,
    HL,
    PC,
    SP
}

#[derive(Copy, Clone)]
pub enum Flag {
    Z,
    N,
    H,
    C
}

#[derive(Copy, Clone)]
pub enum FlagConds {
    NZ,
    Z,
    NC,
    C
}

// If the register needs to be accessed as a u8 this will convert the bool in the struct to a u8
impl From<FlagsRegister> for u8  {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.z { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.n { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.h { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.c { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl FlagsRegister {
    pub fn get_flag(&self, flag: Flag) -> bool {
        match flag {
            Flag::Z => self.z,
            Flag::N => self.n,
            Flag::H => self.h,
            Flag::C => self.c
        }
    }

    pub fn set_flag(&mut self, flag: Flag, val: bool) {
        match flag {
            Flag::Z => self.z = val,
            Flag::N => self.n = val,
            Flag::H => self.h = val,
            Flag::C => self.c = val
        }
    }

}

impl Registers {
    pub fn read_u8(&self, reg: RegisterU8) -> u8 {
        match reg {
            RegisterU8::A => self.a,
            RegisterU8::B => self.b,
            RegisterU8::C => self.c,
            RegisterU8::D => self.d,
            RegisterU8::E => self.e,
            RegisterU8::F => self.f,
            RegisterU8::H => self.h,
            RegisterU8::L => self.l,
        }
    }

    pub fn write_u8(&mut self, reg: RegisterU8, val: u8) {
        match reg {
            RegisterU8::A => self.a = val,
            RegisterU8::B => self.b = val,
            RegisterU8::C => self.c = val,
            RegisterU8::D => self.d = val,
            RegisterU8::E => self.e = val,
            RegisterU8::F => self.f = val, // This needs to be masked to the correct flag
            RegisterU8::H => self.h = val,
            RegisterU8::L => self.l = val,
        }
    }

    pub fn read_u16(&self, reg: RegisterU16) -> u16 {
        match reg {
            RegisterU16::AF => (self.a as u16) << 8 | self.f as u16,
            RegisterU16::BC => (self.b as u16) << 8 | self.c as u16,
            RegisterU16::DE => (self.d as u16) << 8 | self.e as u16,
            RegisterU16::HL => (self.h as u16) << 8 | self.l as u16,
            RegisterU16::PC => self.pc,
            RegisterU16::SP => self.sp,
        }
    }

    pub fn write_u16(&mut self, reg: RegisterU16, val: u16) {
        match reg {
            RegisterU16::AF => {
                let two_bytes = Registers::u16_to_u8(val);
                self.write_u8(RegisterU8::A, two_bytes[0]);
                self.write_u8(RegisterU8::F, two_bytes[1]);
            }

            RegisterU16::BC => {
                let two_bytes = Registers::u16_to_u8(val);
                self.write_u8(RegisterU8::B, two_bytes[0]);
                self.write_u8(RegisterU8::C, two_bytes[1]);
            }

            RegisterU16::DE => {
                let two_bytes = Registers::u16_to_u8(val);
                self.write_u8(RegisterU8::D, two_bytes[0]);
                self.write_u8(RegisterU8::E, two_bytes[1]);
            }

            RegisterU16::HL => {
                let two_bytes = Registers::u16_to_u8(val);
                self.write_u8(RegisterU8::H, two_bytes[0]);
                self.write_u8(RegisterU8::L, two_bytes[1]);
            }

            RegisterU16::PC => self.pc = val,
            RegisterU16::SP => self.sp = val,
        }
    }

    pub fn u16_to_u8(val: u16) -> [u8; 2] {
        let high_byte = (val >> 8) as u8;
        let low_byte = (val & 0xFF) as u8;
        return [high_byte, low_byte];
    }

}