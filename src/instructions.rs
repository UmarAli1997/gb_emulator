pub enum Instructions{
    // Load to the 8-bit register r, the data from the 8-bit register r'
    LD_r_r(u8, u8),
    // Load to the 8-bit register r, the immediate data n
    LD_r_n(u8),
    // Load to the 8-bit register r, data from the absolute address specified by the 16-bit register HL
    LD_r_HL(u8, u16)

}

impl Instructions {
    todo!();
}