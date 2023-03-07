use std::path::Path;
use std::fs;

mod mmu;
mod cpu;
mod instructions;
mod gameboy;

use cpu::{CPU, Registers, RegisterU8};
use instructions::Instruction;
use mmu::MemoryBus;
use crate::gameboy::Gameboy;

fn main() {

    let mut gameboy = Gameboy {
        cpu: CPU::new(),
        memory: MemoryBus::new()
    };

    let boot_path = Path::new("./BOOT/dmg_boot.bin");
    let boot_rom = fs::read(boot_path).expect("File not found!");

    gameboy.memory.copy_to_ram(0, &boot_rom);

    // for val in boot_rom.iter() {
    //     //println!("{:#X}", val)
    //     memory.copy_to_ram(0, &boot_rom);
    // }

    // Checking if the data has been loaded in correctly
    // for i in 0..0x0100 {
    //     println!("{:#X}", gameboy.memory.read_byte(i));
    // }

    gameboy.fetch();

}
