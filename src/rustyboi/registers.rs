pub enum Reg8 {
	B, C, D,
	E, H, L,
	MHL, A,
}

pub enum Reg16 {
	BC, DE, HL,
	SP, AF,
}

#[derive(Default)]
pub struct Registers {
	pub a: u8,
	pub b: u8,
	pub c: u8,
	pub d: u8,
	pub e: u8,
	pub f: u8,
	pub sp: u16,
	pub pc: u16,
}