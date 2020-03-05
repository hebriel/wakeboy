use super::conditionals::*;

const Z_BIT_POS: u8 = 0;
const N_BIT_POS: u8 = 1;
const H_BIT_POS: u8 = 2;
const C_BIT_POS: u8 = 3;

#[derive(Default)]
pub struct RegFlags {
	z: bool,
	n: bool,
	h: bool,
	c: bool,
}

impl RegFlags {
	pub fn clear(&mut self) {
		*self = Default::default();
	}

	pub fn is_condition_met(&self, condition: &Conditional) -> bool {
		match *condition {
			Conditional::NotZero => !self.z,
			Conditional::Zero => self.z,
			Conditional::NotCarry => !self.c,
			Conditional::Carry => self.c,
			Conditional::Invalid => false,
		}
	}
}

impl std::convert::From<RegFlags> for u8  {
    fn from(flag: RegFlags) -> u8 {
        (if flag.z   { 1 } else { 0 }) << Z_BIT_POS |
        (if flag.n   { 1 } else { 0 }) << N_BIT_POS |
        (if flag.h   { 1 } else { 0 }) << H_BIT_POS |
        (if flag.c   { 1 } else { 0 }) << C_BIT_POS
    }
}

impl std::convert::From<u8> for RegFlags {
    fn from(byte: u8) -> Self {
        let z = ((byte >> Z_BIT_POS) & 0b1) != 0;
        let n = ((byte >> N_BIT_POS) & 0b1) != 0;
        let h = ((byte >> H_BIT_POS) & 0b1) != 0;
        let c = ((byte >> C_BIT_POS) & 0b1) != 0;

        RegFlags {
            z,
            n,
            h,
            c,
        }
    }
}