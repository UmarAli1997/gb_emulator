pub enum Instruction {
    // Load to the 8-bit register r, the data from the 8-bit register r'
    LD_r_r{ r1: u8, r2: u8 },

    // Load to the 8-bit register r, the immediate data n
    LD_r_n{ r1: u8 },

    // Load to the 8-bit register r, data from the absolute address specified by the 16-bit register HL
    LD_r_HL{ r1: u8, r2: u16 },

    // Load to the absolute address specified by the 16-bit register HL, data from the 8-bit register r
    LD_HL_r { r1: u16, r2: u8 },

    // Load to the absolute address specified by the 16-bit register HL, the immediate data n
    LD_HL_n { r1: u16, r2: u8 },

    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register BC
    LD_A_BC { r1: u8, r2: u16 },

    // Load to the 8-bit A register, data from the absolute address specified by the 16-bit register DE
    LD_A_DE { r1:u8, r2: u16 },

    // Load to the absolute address specified by the 16-bit register BC, data from the 8-bit A register
    LD_BC_A { r1: u16, r2: u8 }

}

impl Instructions {
    todo!();
}