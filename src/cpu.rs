use std::convert::From;

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

// Building CPU up
pub struct CPU {
    pub register: Registers,
    pub flags: FlagsRegister,
    pub ime: bool
}

// Initialising CPU with zero values
impl CPU {
    pub fn new() -> CPU {
        let cpu = CPU {
            register: Registers {
                a: 0x0,
                b: 0x0,
                c: 0x0,
                d: 0x0,
                f: 0x0,
                e: 0x0,
                h: 0x0,
                l: 0x0,
                pc: 0x0,
                sp: 0x0,
            },
            flags: FlagsRegister { z: false, n: false, h: false, c: false },
            ime: false,
        };
        cpu
    }
}

// // Initialising CPU with zero values
// impl CPU {
//     pub fn new() -> CPU {
//         let cpu = CPU {
//             register: Registers {
//                 a: 0x01,
//                 b: 0x0,
//                 c: 0x13,
//                 d: 0x0,
//                 f: 0xB0,
//                 e: 0xD8,
//                 h: 0x01,
//                 l: 0x4D,
//                 pc: 0x100,
//                 sp: 0xFFFE,
//             },
//             flags: FlagsRegister { z: false, n: false, h: false, c: false },
//             ime: false,
//         };
//         cpu
//     }
// }

#[derive(Copy, Clone)]
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

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
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

#[derive(Copy, Clone)]
pub enum InterruptConds {
    Enabled,
    Disabled
}

impl CPU {
    pub fn get_ime_state(&self) -> bool {
        return self.ime;
    }

    pub fn set_ime_state(&mut self, interrupt_condition: InterruptConds) {
        match interrupt_condition {
            InterruptConds::Enabled => self.ime = true,
            InterruptConds::Disabled => self.ime = false
        }
    }
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

    pub fn get_f_reg(&self, reg: FlagsRegister) -> u8 {
        return reg.into();
    }

    pub fn update_f_reg(&mut self, val: FlagsRegister) { 
        self.write_u8(RegisterU8::F, val.into());
    }

    pub fn write_u8(&mut self, reg: RegisterU8, val: u8) {
        match reg {
            RegisterU8::A => self.a = val,
            RegisterU8::B => self.b = val,
            RegisterU8::C => self.c = val,
            RegisterU8::D => self.d = val,
            RegisterU8::E => self.e = val,
            RegisterU8::F => self.f = val,
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
                let [lsb, msb] = val.to_le_bytes();
                self.write_u8(RegisterU8::A, msb);
                self.write_u8(RegisterU8::F, lsb);
            }

            RegisterU16::BC => {
                let [lsb, msb] = val.to_le_bytes();
                self.write_u8(RegisterU8::B, msb);
                self.write_u8(RegisterU8::C, lsb);
            }

            RegisterU16::DE => {
                let [lsb, msb] = val.to_le_bytes();
                self.write_u8(RegisterU8::D, msb);
                self.write_u8(RegisterU8::E, lsb);
            }

            RegisterU16::HL => {
                let [lsb, msb] = val.to_le_bytes();
                self.write_u8(RegisterU8::H, msb);
                self.write_u8(RegisterU8::L, lsb);
            }

            RegisterU16::PC => self.pc = val,
            RegisterU16::SP => self.sp = val,
        }
    }
}