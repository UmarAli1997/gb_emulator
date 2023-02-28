use crate::cpu;

// pub struct Instruction {
//     cpu_state: cpu::CPU,
//     opcode: u8,
//     cycles: u8,
//     length: u8
// }

pub enum Instruction {
    LD_r_r
}

impl Instruction {
    pub fn LD_r_r() {
        println!("LD_r_r")
    }
}