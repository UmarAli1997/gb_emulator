pub enum Instruction {
    // Load to the 8-bit register r, the data from the 8-bit register r'
    LD_r_r{ r1: u8, r2: u8 },

    // Load to the 8-bit register r, the immediate data n
    LD_r_n{ r1: u8 },

    // Load to the 8-bit register r, data from the absolute address specified by the 16-bit register HL
    LD_r_HL{ r1: u8, hl: u16 },

    // Load to the absolute address specified by the 16-bit register HL, data from the 8-bit register r
    LD_HL_r { hl: u16, r1: u8 },

    // Load to the absolute address specified by the 16-bit register HL, the immediate data n
    LD_HL_n { hl: u16 },

    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register BC
    LD_A_BC { a: u8, bc: u16 },

    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register DE
    LD_A_DE { a:u8, de: u16 },

    // Load to the absolute address specified by the 16-bit register BC, data from the 8-bit A register
    LD_BC_A { bc: u16, a: u8 },

    // Load to the absolute address specified by the 16-bit register DE, data from the 8-bit A register
    LD_DE_A { de: u16, a: u8 },

    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit operand nn
    LD_A_nn { a: u8 },

    // Load to the absolute address specified by the 16-bit operand nn, data from the 8-bit A register
    LD_nn_A { a: u8 },

    // Load to the 8-bit A register, data from the address specified by the 8-bit C register. The full 16-bit absolute
    // address is obtained by setting the most significant byte to 0xFF and the least significant byte to the value of C,
    // so the possible range is 0xFF00-0xFFFF
    LDH_A_C { a: u8, c: u8 },

    // Load to the address specified by the 8-bit C register, data from the 8-bit A register. The full 16-bit absolute
    // address is obtained by setting the most significant byte to 0xFF and the least significant byte to the value of C,
    // so the possible range is 0xFF00-0xFFFF
    LDH_C_A { c: u8, a: u8 },

    // Load to the 8-bit A register, data from the address specified by the 8-bit immediate data n. The full 16-bit
    // absolute address is obtained by setting the most significant byte to 0xFF and the least significant byte to the
    // value of n, so the possible range is 0xFF00-0xFFFF.
    LDH_A_n { a: u8 },

    // Load to the address specified by the 8-bit immediate data n, data from the 8-bit A register. The full 16-bit
    // absolute address is obtained by setting the most significant byte to 0xFF and the least significant byte to the
    // value of n, so the possible range is 0xFF00-0xFFFF
    LDH_n_A { a: u8 },

    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register HL. The value of
    // HL is decremented after the memory read
    LD_A_decHL { a: u8, hl: u16 },

    // Load to the absolute address specified by the 16-bit register HL, data from the 8-bit A register. The value of
    // HL is decremented after the memory write
    LD_decHL_A { hl: u16, a: u8 },

    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register HL. The value of
    // HL is incremented after the memory read
    LD_A_incHL { a: u8, hl: u16 },

    // Load to the absolute address specified by the 16-bit register HL, data from the 8-bit A register. The value of
    // HL is incremented after the memory write
    LD_incHL_A { hl: u16, a: u8 }

}

impl Instructions {
    todo!();
}