use bitflags::bitflags;

pub struct Flags {
    flags: u8
}

bitflags! {
    pub struct FlagOps: u32 {
        const Zero = 0x01;
        const HalfCarry = 0x02;
        const Carry = 0x04;
        const ForceZeroOn = 0x8;
        const ForceZeroOff = 0x10;
        const ForceSubtractOn = 0x20;
        const ForceSubtractOff = 0x40;
        const ForceHalfCarryOn = 0x80;
        const ForceHalfCarryOff = 0x100;
        const ForceCarryOn = 0x200;
        const ForceCarryOff = 0x400;
    }
}

const ADD_8_BIT_FLAGS: FlagOps = FlagOps::Zero | FlagOps::ForceSubtractOff | FlagOps::HalfCarry | FlagOps::Carry;

impl Flags {
    pub fn update_flags(self: &mut Self, overflowed: bool, updates: FlagOps, a: u8, b: u8, result: u8) {
        
        // Todo this feels messy with nested ifs and duplication, clean up.
        if (updates.intersects(FlagOps::Zero)) {
            if(result == 0) {
                self.flags = self.flags | 0x80;
            }
            else {
                self.flags = self.flags & 0x70;
            }
        }
        // Todo come back to this, I'm hoping it only affects the DAA for now
        if (updates.intersects(FlagOps::HalfCarry)){

        }
        if (updates.intersects(FlagOps::Carry)){
            match overflowed {
                true => { self.flags = self.flags | 0x10 },
                false => { self.flags = self.flags & 0xe0 }
            }
        }
        if (updates.intersects(FlagOps::ForceZeroOn)){
            self.flags = self.flags | 0x80;
        }
        if (updates.intersects(FlagOps::ForceZeroOff)){
            self.flags = self.flags & 0x70;
        }
        if (updates.intersects(FlagOps::ForceSubtractOn)){
            self.flags = self.flags | 0x40;
        }
        if (updates.intersects(FlagOps::ForceSubtractOff)){
            self.flags = self.flags & 0xb0;
        }
        if (updates.intersects(FlagOps::ForceHalfCarryOn)){
            self.flags = self.flags | 0x20;
        }
        if (updates.intersects(FlagOps::ForceHalfCarryOff)){
            self.flags = self.flags & 0xd0;
        }
        if (updates.intersects(FlagOps::ForceCarryOn)){
            self.flags = self.flags | 0x10;
        }
        if (updates.intersects(FlagOps::ForceCarryOff)){
            self.flags = self.flags & 0xe0;
        }
    }
}

fn set_zero(res: Option<u8>) -> bool {
    match res {
        None => { false },
        Some(result) => { result == 0 }
    }
}

fn set_carry(res: Option<u8>) -> bool {
    match res {
        None => { true },
        Some(_) => { false }
    }
}

pub fn add(a: u8, b: u8, flags: &mut Flags) -> u8 {

    let res = a.overflowing_add(b);

    flags.update_flags(res.1, ADD_8_BIT_FLAGS, a, b, res.0);

    return res.0;
}

pub fn sub(a: u8, b: u8) -> Option<u8> {
    return a.checked_sub(b);
}