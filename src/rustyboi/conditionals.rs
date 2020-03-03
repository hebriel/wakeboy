use super::core::*;

pub enum Conditional {
	NotZero,
	Zero,
	NotCarry,
	Carry,
	Unknown,
}

impl std::convert::From<u8> for Conditional {
	fn from(bits: u8) -> Self {
		match bits {
			0b00 => Conditional::NotZero,
			0b01 => Conditional::Zero,
			0b10 => Conditional::NotCarry,
			0b11 => Conditional::Carry,
			_ 	 => {
				warn_or_crash(format!("Unknown Conditional ({:#b})", bits));
				Conditional::Unknown
			}
		}
	}
}