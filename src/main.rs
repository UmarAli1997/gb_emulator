
mod memoryBus;
mod cpu;

pub struct CPU {
    register: cpu::Registers,
    pc: u16,
    bus: memoryBus::MemoryBus
}



fn main() {
    println!("Hello, world!");
}
