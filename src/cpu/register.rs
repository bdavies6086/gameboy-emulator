use std::ops::{Index, IndexMut};

use super::operations::Operation;

pub enum RegisterKeys {
    ACC,
    B,
    C,
    D,
    E,
    H,
    L
}

pub struct Register {
    pub acc: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8
}

impl Index<RegisterKeys> for Register {

    type Output = u8;
    fn index(&self, index: RegisterKeys) -> &Self::Output {
        match index {
            RegisterKeys::ACC => &self.acc,
            RegisterKeys::B => &self.b,
            RegisterKeys::C => &self.c,
            RegisterKeys::D => &self.d,
            RegisterKeys::E => &self.e,
            RegisterKeys::H => &self.h,
            RegisterKeys::L => &self.l,

        } 
    }
}

impl IndexMut<RegisterKeys> for Register {

    fn index_mut(&mut self, index: RegisterKeys) -> &mut Self::Output {
        match index {
            RegisterKeys::ACC => &mut self.acc,
            RegisterKeys::B => &mut self.b,
            RegisterKeys::C => &mut self.c,
            RegisterKeys::D => &mut self.d,
            RegisterKeys::E => &mut self.e,
            RegisterKeys::H => &mut self.h,
            RegisterKeys::L => &mut self.l
        } 
    }
}

pub fn ld(register: &mut Register, key1: RegisterKeys, key2: RegisterKeys) -> Operation {
   *register.index_mut(key1) = *register.index(key2);

   return Operation { size: 1, num_cycles: 4 };
}