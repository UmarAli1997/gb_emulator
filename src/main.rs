
mod memory_bus;
mod cpu;

pub struct CPU {
    register: cpu::Registers,
    pc: u16,
    bus: memory_bus::MemoryBus
}



fn main() {
    println!("Hello, world!");
}
