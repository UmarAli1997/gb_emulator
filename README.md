# Gameboy Emulator

This is my attempt at creating a Gameboy emulator in Rust.

The main goals of this project are to:
- Learn Rust
- Become comfortable with bitwise operations
- Learn to write unit tests
- Gain an understanding of how computer hardware functions
- And finally of course, be able to play a game on the emulator :D
    - Tetris first, then hopefully work towards Pokemon

## Status

CPU instructions are implemented for the DMG bootrom.
The next stage is to debug the instruction set to make sure the CPU is behaving as it should.
In parallel I will have to implement the PPU (Picture Processing Unit) as well as interrupts and timer most likely to be able to continue debugging.