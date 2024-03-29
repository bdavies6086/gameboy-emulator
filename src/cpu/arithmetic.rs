use bitflags::bitflags;

pub struct Flags {
    flags: u8
}

bitflags! {
    pub struct FlagOps: u32 {
        const Zero = 0x01;
        const Subtract = 0x02;
        const HalfCarry = 0x04;
        const Carry = 0x08;
        const ForceZeroOn = 0x10;
        const ForceZeroOff = 0x20;
        const ForceSubtractOn = 0x40;
        const ForceSubtractOff = 0x80;
        const ForceHalfCarryOn = 0x100;
        const ForceHalfCarryOff = 0x200;
        const ForceCarryOn = 0x400;
        const ForceCarryOff = 0x800;
    }
}

const ADD_8_BIT_FLAGS: FlagOps = FlagOps::Zero | FlagOps::ForceSubtractOff | FlagOps::HalfCarry | FlagOps::Carry;

impl Flags {
    pub fn update_flags(self: &mut Self, result: Option<u8>, updates: FlagOps) {
        
         
        if (updates.contains(FlagOps::Zero)) {

        }
        if (updates.contains(FlagOps::Subtract)){}
        if (updates.contains(FlagOps::HalfCarry)){}
        if (updates.contains(FlagOps::Carry)){}
        if (updates.contains(FlagOps::ForceZeroOn)){
            self.flags = self.flags | 0x80;
        }
        if (updates.contains(FlagOps::ForceZeroOff)){
            self.flags = self.flags & 0x70;
        }
        if (updates.contains(FlagOps::ForceSubtractOn)){
            self.flags = self.flags | 0x40;
        }
        if (updates.contains(FlagOps::ForceSubtractOff)){
            self.flags = self.flags & 0xb0;
        }
        if (updates.contains(FlagOps::ForceHalfCarryOn)){
            self.flags = self.flags | 0x20;
        }
        if (updates.contains(FlagOps::ForceHalfCarryOff)){
            self.flags = self.flags & 0xd0;
        }
        if (updates.contains(FlagOps::ForceCarryOn)){
            self.flags = self.flags | 0x10;
        }
        if (updates.contains(FlagOps::ForceCarryOff)){
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

pub fn add(a: u8, b: u8, flags: &mut Flags) -> Option<u8> {

    let res = a.checked_add(b);

    flags.update_flags(res, ADD_8_BIT_FLAGS);

    return res;
}

pub fn sub(a: u8, b: u8) -> Option<u8> {
    return a.checked_sub(b);
}