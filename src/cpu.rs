use std::convert::From;
use crate::Gameboy;
use crate::mmu::MemoryBus;
use crate::instructions::Instruction;

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

// Building CPU up
pub struct CPU {
    pub register: Registers,
    pub pc: u16,
    pub sp: u16,
    flags: FlagsRegister
}

// Initialising CPU with zero values
impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            register: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
                f: 0,
                e: 0,
                h: 0,
                l: 0,
            },
            pc: 0,
            sp: 0,
            flags: FlagsRegister { z: false, n: false, h: false, c: false }
        };
        cpu
    }

}

struct FlagsRegister {
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
}

pub enum RegisterU8 {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    L
}

enum RegisterU16 {
    AF,
    BC,
    DE,
    HL
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
            _ => panic!("Invalid read_u8")
        }
    }

    pub fn write_u8(&mut self, reg: RegisterU8, val: u8) {
        match reg {
            A => self.a = val,
            B => self.b = val,
            C => self.c = val,
            D => self.d = val,
            E => self.e = val,
            F => self.f = val, // This needs to be masked to the correct flag
            H => self.h = val,
            L => self.l = val,
            _ => panic!("Invalid write_u8")
        }
    }

    pub fn read_u16(&self, reg: RegisterU16) -> u16 {
        match reg {
            AF => (self.a as u16) << 8 | self.f as u16,
            BC => (self.b as u16) << 8 | self.c as u16,
            DE => (self.d as u16) << 8 | self.e as u16,
            HL => (self.h as u16) << 8 | self.l as u16,
            _ => panic!("Invalid read_u16")
        }
    }

    pub fn read_af(&self) -> u16 {
        // Left shift register a by 8 bits and do a bitwise OR operation with register f
        return (self.a as u16) << 8 | self.f as u16;
    }

    pub fn set_af(&mut self, value: u16) {
        // Bitwise AND the MSB then right shift the result into the LSB to store into the register
        self.a = ((value & 0xFF00) >> 8) as u8;
        // No need to right shift the result here as the LSB is already in the correct position
        self.f = (value & 0xFF) as u8;
    }

    pub fn read_bc(&self) -> u16 {
        return (self.b as u16) << 8 | self.c as u16;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn read_de(&self) -> u16 {
        return (self.d as u16) << 8 | self.e as u16;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn read_hl(&self) -> u16 {
        return (self.h as u16) << 8 | self.l as u16;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}