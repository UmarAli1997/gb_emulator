use std::path::Path;
use std::fs;

use crate::cpu::CPU;
use crate::memory_bus::MemoryBus;


mod memory_bus;
mod cpu;


fn main() {

    let mut cpu = CPU::new();
    let mut memory = MemoryBus::new();

    let boot_path = Path::new("./BOOT/dmg_boot.bin");
    let boot_rom = fs::read(boot_path).expect("File not found!");

    for val in boot_rom.iter() {
        //println!("{:#X}", val)
        memory.copy_rom(0, &boot_rom);
    }

//     for i in 0..0x0102 {
//         println!("{:#X}", memory.read_byte(i));
//     }
}
