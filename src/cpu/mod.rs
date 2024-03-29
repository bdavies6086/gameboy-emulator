mod operations;
mod register;
mod arithmetic;

use register::*;
use arithmetic::add;
use operations::Operation;

const MAX_CYCLES: i32 = 69905;

pub struct CPU {
    registers: Register,
    flags: u8,
    h_ram: [u8; 127]
}

impl CPU {

    pub fn new() -> Self {
        return Self {
            registers: Register {
                acc: 0,
                b: 0,
                c: 0,
                d: 0,
                e: 0,
                h: 0,
                l: 0
            },
            flags: 0,
            h_ram: [0; 127]
        }
    }

    pub fn tick(&mut self) {
        let mut cycle_count = 0;

        while cycle_count < MAX_CYCLES {
            cycle_count = cycle_count + 1;
        }

    }

}

fn noop() -> Operation {
    Operation { size: 1, num_cycles: 4 }
}

fn run_op(op: u8, register: &mut Register, flags: u8) -> Operation {
    return match op {
        0x00 => noop(),

        0x40 => ld(register, RegisterKeys::B, RegisterKeys::B),
        0x41 => ld(register, RegisterKeys::B, RegisterKeys::C),
        0x42 => ld(register, RegisterKeys::B, RegisterKeys::D),
        0x43 => ld(register, RegisterKeys::B, RegisterKeys::E),
        0x44 => ld(register, RegisterKeys::B, RegisterKeys::H),
        0x45 => ld(register, RegisterKeys::B, RegisterKeys::L),
        // 0x46
        0x47 => ld(register, RegisterKeys::B, RegisterKeys::ACC),
        0x48 => ld(register, RegisterKeys::C, RegisterKeys::B),
        0x49 => ld(register, RegisterKeys::C, RegisterKeys::C),
        0x4a => ld(register, RegisterKeys::C, RegisterKeys::D),
        0x4b => ld(register, RegisterKeys::C, RegisterKeys::E),
        0x4c => ld(register, RegisterKeys::C, RegisterKeys::H),
        0x4d => ld(register, RegisterKeys::C, RegisterKeys::L),
      //0x4e
        0x4f => ld(register, RegisterKeys::C, RegisterKeys::ACC),

        0x50 => ld(register, RegisterKeys::D, RegisterKeys::B),
        0x51 => ld(register, RegisterKeys::D, RegisterKeys::C),
        0x52 => ld(register, RegisterKeys::D, RegisterKeys::D),
        0x53 => ld(register, RegisterKeys::D, RegisterKeys::E),
        0x54 => ld(register, RegisterKeys::D, RegisterKeys::H),
        0x55 => ld(register, RegisterKeys::D, RegisterKeys::L),
     // 0x56
        0x57 => ld(register, RegisterKeys::D, RegisterKeys::ACC),
        0x58 => ld(register, RegisterKeys::E, RegisterKeys::B),
        0x59 => ld(register, RegisterKeys::E, RegisterKeys::C),
        0x5a => ld(register, RegisterKeys::E, RegisterKeys::D),
        0x5b => ld(register, RegisterKeys::E, RegisterKeys::E),
        0x5c => ld(register, RegisterKeys::E, RegisterKeys::H),
        0x5d => ld(register, RegisterKeys::E, RegisterKeys::L),
     // 0x5e
        0x5f => ld(register, RegisterKeys::E, RegisterKeys::ACC),

        0x60 => ld(register, RegisterKeys::H, RegisterKeys::B),
        0x61 => ld(register, RegisterKeys::H, RegisterKeys::C),
        0x62 => ld(register, RegisterKeys::H, RegisterKeys::D),
        0x63 => ld(register, RegisterKeys::H, RegisterKeys::E),
        0x64 => ld(register, RegisterKeys::H, RegisterKeys::H),
        0x65 => ld(register, RegisterKeys::H, RegisterKeys::L),
     // 0x66
        0x67 => ld(register, RegisterKeys::H, RegisterKeys::ACC),
        0x68 => ld(register, RegisterKeys::L, RegisterKeys::B),
        0x69 => ld(register, RegisterKeys::L, RegisterKeys::C),
        0x6a => ld(register, RegisterKeys::L, RegisterKeys::D),
        0x6b => ld(register, RegisterKeys::L, RegisterKeys::E),
        0x6c => ld(register, RegisterKeys::L, RegisterKeys::H),
        0x6d => ld(register, RegisterKeys::L, RegisterKeys::L),
     // 0x6e
        0x6f => ld(register, RegisterKeys::L, RegisterKeys::ACC),
        _=> panic!("unknown op")
    };
}