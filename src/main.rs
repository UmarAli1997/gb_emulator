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

    let cart_path = Path::new("./BOOT/Tetris (World) (Rev 1).gb");
    let cart = fs::read(cart_path).expect("File not found!");

    let test_path = Path::new("./TEST/09-op r,r.gb");
    let test = fs::read(test_path).expect("File not found!");

    //gameboy.memory.copy_to_ram(0, &cart);
    //gameboy.memory.copy_to_ram(0, &boot_rom);
    gameboy.memory.copy_to_ram(0, &test);
    //gameboy.write_instruction(0xFF44, 0x90);

    // for val in boot_rom.iter() {
    //     //println!("{:#X}", val)
    //     memory.copy_to_ram(0, &boot_rom);
    // }

    // //Checking if the data has been loaded in correctly
    // for (i, item) in gameboy.memory.ram.iter().enumerate() {
    //     println!("{:#X}: {}", i, item);
    // }

    let mut counter = 1;
    loop {
        println!("{}", counter);
        gameboy.fetch();
        counter += 1;
        // if counter > 47932 {
        //     gameboy.memory.copy_to_ram(0, &cart);
        // }
    }

}
