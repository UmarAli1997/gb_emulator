pub struct Timer {
    pub div_clocksum: usize,
    pub timer_clocksum: usize,
    pub div_reg: u8,
    pub tac_reg: u8,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            div_clocksum: 0,
            timer_clocksum: 0,
            div_reg: 0,
            tac_reg: 0
        }
    }
}