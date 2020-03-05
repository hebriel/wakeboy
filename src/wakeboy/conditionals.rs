use super::core::*;

#[derive(Copy, Clone)]
pub enum Conditional {
	NotZero,
	Zero,
	NotCarry,
	Carry,
	Invalid,
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
				Conditional::Invalid
			}
		}
	}
}

impl std::fmt::Display for Conditional {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ret = match *self {
			Conditional::NotZero => "NZ",
			Conditional::Zero => "Z",
			Conditional::NotCarry => "NC",
			Conditional::Carry => "C",
			Conditional::Invalid => "Invalid Conditional",
		};
		write!(f, "{}", ret)
    }
}