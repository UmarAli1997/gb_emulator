use std::path::Path;
use std::fs;

mod mmu;
mod cpu;
mod gameboy;

use gameboy::Gameboy;

fn main() {

    let mut gameboy = Gameboy::new();

    let boot_path = Path::new("./BOOT/dmg_boot.bin");
    let boot_rom = fs::read(boot_path).expect("File not found!");

    gameboy.memory.copy_to_ram(0, &boot_rom);

    // for val in boot_rom.iter() {
    //     //println!("{:#X}", val)
    //     memory.copy_to_ram(0, &boot_rom);
    // }

    // //Checking if the data has been loaded in correctly
    // for (i, item) in gameboy.memory.ram.iter().enumerate() {
    //     println!("{:#X}: {}", i, item);
    // }

    gameboy.fetch();

}
