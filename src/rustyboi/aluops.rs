use super::core::*;

#[derive(Copy, Clone)]
pub enum Operation {
	ADD,
	ADC,
	SUB,
	SBC,
	AND,
	XOR,
	OR,
	CP,
	Invalid,
}

impl std::convert::From<u8> for Operation {
	fn from(bits: u8) -> Self {
		match bits {
			0b000 => Operation::ADD,
			0b001 => Operation::ADC,
			0b010 => Operation::SUB,
			0b011 => Operation::SBC,
			0b100 => Operation::AND,
			0b101 => Operation::XOR,
			0b110 => Operation::OR,
			0b111 => Operation::CP,
			_ => Operation::Invalid,
		}
	}
}

impl std::fmt::Display for Operation {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let ret = match *self {
			Operation::ADD => "ADD",
			Operation::ADC => "ADC",
			Operation::SUB => "SUB",
			Operation::SBC => "SBC",
			Operation::AND => "AND",
			Operation::XOR => "XOR",
			Operation::OR => "OR",
			Operation::CP => "CP",
			Operation::Invalid => "Invalid ALU Operation",
		};
		write!(f, "{}", ret)
	}
}