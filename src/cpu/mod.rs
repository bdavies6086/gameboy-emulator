mod operations;
mod register;
mod arithmetic;

use register::*;
use arithmetic::add;

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
                e: 0
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

fn run_op(op: u8, register: &mut Register, flags: &mut u8) -> (u8, u8) {
    return match op {
        0x00 => noop(),
        _=> panic!("unknown op")
    };
}