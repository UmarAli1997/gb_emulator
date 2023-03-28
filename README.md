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

CPU instructions have been debugged for the DMG boot rom and I can confidently say that the CPU executes the instructions correctly.
The next stage is to implement the rest of the CPU instructions and then use the blargg test rom set to validate my CPU.
Once the CPU has been debugged and validated I will work on the other aspects of the emulator such as the PPU, timer and interrupts.