use std::path::Path;
use std::fs;

use cpu::{CPU, Registers, RegisterU8};
use instructions::Instruction;
use mmu::MemoryBus;

mod mmu;
mod cpu;
mod instructions;

fn main() {

    let mut cpu = CPU::new();
    let mut memory = MemoryBus::new();

    let boot_path = Path::new("./BOOT/dmg_boot.bin");
    let boot_rom = fs::read(boot_path).expect("File not found!");

    memory.copy_to_ram(0, &boot_rom);

    // for val in boot_rom.iter() {
    //     //println!("{:#X}", val)
    //     memory.copy_to_ram(0, &boot_rom);
    // }

    // Checking if the data has been loaded in correctly
    // for i in 0..0x0100 {
    //     println!("{:#X}", memory.read_byte(i));
    // }

    cpu.execute();

}
