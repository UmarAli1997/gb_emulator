use super::*;
use crate::gameboy::Gameboy;

pub fn init_gameboy() -> Gameboy {
    let mut gameboy = Gameboy {
        cpu = CPU {
            register: Registers {
                a: 0,
                b: 1,
                c: 0,
                d: 0,
                f: 0,
                e: 0,
                h: 0,
                l: 0,
            },
            pc: 0,
            sp: 0,
            flags: FlagsRegister { z: false, n: false, h: false, c: false }
        },
        memory: MemoryBus::new(),
    };

    return gameboy;
}

#[cfg(tests)]
mod tests {

    #[test]
    fn test_ld_r_r() {
        let mut gameboy = init_gameboy();
        let r1 = RegisterU8::A;
        let r2 = RegisterU8::B;

        gameboy.ld_r_r(r1, r2);
        let new_r1 = gameboy.cpu.register.read_u8(r1);

        assert_eq!(new_r1, 1);
    }
}


