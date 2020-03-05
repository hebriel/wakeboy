use super::direction::*;
use super::conditionals::*;
use super::registers::{Reg8, Reg16};
use super::bus::*;
use super::aluops::*;

pub enum Instruction {
	// Data -> http://goldencrystal.free.fr/GBZ80Opcodes.pdf
	NOP,
	LDmnnSP 	(u16),
	LDrrNN 		(Reg16, u16),
	ADDhlRR		(Reg16),
	LDmrrA		(Reg16),
	LDaMRR		(Reg16),
	INCrr		(Reg16),
	DECrr		(Reg16),
	INCr 		(Reg8),
	DECr		(Reg8),
	LDrN 		(Reg8, u8),
	RdCA		(Direction),
	RdA 		(Direction),
	STOP,
	JRe			(u8),
	JRcE 		(Conditional, u8),
	LDImhlA,
	LDIaMHL,
	LDDmhlA,
	LDDaMHL,
	DAA,
	CPL,
	SCF,
	CCF,
	LDrR 		(Reg8, Reg8),
	HALT,
	ALUaR		(Operation, Reg8),
	ALUaN		(Operation, u8),
	POPrr		(Reg16),
	PUSHrr		(Reg16),
	RSTn		(u8),
	RETc		(Conditional),
	RET,
	RETI,
	JPcNN		(Conditional, u16),
	JPnn 		(u16),
	CALLcNN		(Conditional, u16),
	CALLnn 		(u16),
	ADDspN		(u8),
	LDhlSPpN	(u8),
	LDmzpPnA	(u8),
	LDaMZPpN	(u8),
	LDmcA,
	LDaMC,
	LDmnnA		(u16),
	LDaMNN 		(u16),
	JPhl,
	LDspHL,
	DI,
	EI,
	RdCr		(Direction, Reg8),
	RdR 		(Direction, Reg8),
	SdAr		(Direction, Reg8),
	SWAPr		(Reg8),
	SRLr		(Reg8),
	BITnR		(u8, Reg8),
	RESnR		(u8, Reg8),
	SETnR		(u8, Reg8),
	Unknown,
	Invalid,
	Unassigned,
}

impl Instruction {

	pub fn fetch(mem: &mut MemoryBus, pc: &mut u16) -> (Instruction, String) {

		let opcode = match mem.get_byte(*pc as usize) {
			None => return (Instruction::Unassigned, "Unassigned".to_owned()),
			Some(o) => o,
		};
		
		if opcode == 0x00 {
			return (Instruction::NOP, "NOP".to_owned())
		}

		//Prefixed opcodes and opcodes without internal parameters
		match opcode {
			0xCB => {
				let opcode = mem.get_imm8(pc);
				let header = (opcode & 0b11000000) >> 6;
				match header {
					0b00 => {
						let header = (opcode & 0b00110000) >> 4;
						match header {
							0b00 => {
								let direction = Direction::from((opcode & 0b00001000) >> 3);
								let reg = Reg8::from(opcode & 0b111);
								return (Instruction::RdCr(direction, reg), format!("RdCr {}, {}", direction, reg))
							},
							0b01 => {
								let direction = Direction::from((opcode & 0b00001000) >> 3);
								let reg = Reg8::from(opcode & 0b111);
								return (Instruction::RdR(Direction::from((header & 0b00001000) >> 3), Reg8::from(opcode & 0b111)),
								format!("RdR {}, {}", direction, reg))
							},
							0b10 => {
								let direction = Direction::from((opcode & 0b00001000) >> 3);
								let reg = Reg8::from(opcode & 0b111);
								return (Instruction::SdAr(Direction::from((opcode & 0b00001000) >> 3), Reg8::from(opcode & 0b111)),
								format!("SdAr {}, {}", direction, reg))
							},
							0b11 => {
								let reg = Reg8::from(opcode & 0b111);
								if (opcode & 0b1000) >> 3 == 0 {
									return (Instruction::SWAPr(Reg8::from(opcode & 0b111)),
									format!("SWAP {}", reg))
								}
								return (Instruction::SRLr(Reg8::from(opcode & 0b111)),
									format!("SRL {}", reg))
							}
							_ => return (Instruction::Invalid, "Invalid".to_owned()),
						}
					},
					0b01 => {
						let n = (opcode & 0b00111000) >> 3;
						let reg = Reg8::from(opcode & 0b111);
						return (Instruction::BITnR(n, reg), format!("BIT {:#04x}, {}", n, reg))
					},
					0b10 => {
						let n = (opcode & 0b00111000) >> 3;
						let reg = Reg8::from(opcode & 0b111);
						return (Instruction::RESnR(n, reg), format!("RES {:#04x}, {}", n, reg))
					},
					0b11 => {
						let n = (opcode & 0b00111000) >> 3;
						let reg = Reg8::from(opcode & 0b111);
						return (Instruction::SETnR(n, reg), format!("SET {:#04x}, {}", n, reg))
					},
					_ => {}
				}
			},
			0x08 => {
				let n = mem.get_imm16(pc);
				return (Instruction::LDmnnSP(n), format!("LD ({:#06x}), SP", n))
			},
			0x10 => return (Instruction::STOP, "STOP".to_owned()),
			0x18 => {
				let n = mem.get_imm8(pc);
				return (Instruction::JRe(n), format!("JR pc({:#06x})+{:#04x}", *pc, n))
			},
			0x22 => return (Instruction::LDImhlA, "LDI (HL), A".to_owned()),
			0x2A => return (Instruction::LDIaMHL, "LDI A, (HL)".to_owned()),
			0x32 => return (Instruction::LDDmhlA, "LDD (HL), A".to_owned()),
			0x3A => return (Instruction::LDDaMHL, "LDD A, (HL)".to_owned()),
			0x27 => return (Instruction::DAA, "DAA".to_owned()),
			0x2F => return (Instruction::CPL, "CPL".to_owned()),
			0x37 => return (Instruction::SCF, "SCF".to_owned()),
			0x3F => return (Instruction::CCF, "CCF".to_owned()),
			0xC9 => return (Instruction::RET, "RET".to_owned()),
			0xD9 => return (Instruction::RETI, "RETI".to_owned()),
			0xC3 => {
				let n = mem.get_imm16(pc);
				return (Instruction::JPnn(n), format!("JP {:#06x}", n))
			},
			0xCD => {
				let n = mem.get_imm16(pc);
				return (Instruction::CALLnn(n), format!("CALL {:#06x}", n))
			},
			0xE8 => {
				let n = mem.get_imm8(pc);
				return (Instruction::ADDspN(n), format!("ADD SP, {:#04x}", n))
			},
			0xF8 => {
				let n = mem.get_imm8(pc);
				return (Instruction::LDhlSPpN(n), format!("LD HL, SP+{:#04x}", n))
			},
			0xE0 => {
				let n = mem.get_imm8(pc);
				return (Instruction::LDmzpPnA(n), format!("LD (0xFF00+{:#04x}), A", n))
			},
			0xF0 => {
				let n = mem.get_imm8(pc);
				return (Instruction::LDaMZPpN(n), format!("LD A, (FF00+{:#04x})", n))
			},
			0xE2 => return (Instruction::LDmcA, "LD (0xFF00+C), A".to_owned()),
			0xF2 => return (Instruction::LDaMC, "LD A, (0xFF00+C)".to_owned()),
			0xEA => {
				let n = mem.get_imm16(pc);
				return (Instruction::LDmnnA(n), format!("LD ({:#06x}), A", n))
			},
			0xFA => {
				let n = mem.get_imm16(pc);
				return (Instruction::LDaMNN(n), format!("LD A, ({:#06x})", n))
			},
			0xE9 => return (Instruction::JPhl, "JP HL".to_owned()),
			0xF9 => return (Instruction::LDspHL, "LD SP, HL".to_owned()),
			0xF3 => return (Instruction::DI, "DI".to_owned()),
			0xFB => return (Instruction::EI, "EI".to_owned()),
			_ => {}
		}

		let header = (opcode & 0b11000000) >> 6;
		let six_bit_rem = opcode & 0b00111111;

		match header {
			0b00 => {
				let half = six_bit_rem & 0b1111;
				match half {
					0b0001 => {
						let reg = Reg16::from(six_bit_rem >> 4, true);
						let n = mem.get_imm16(pc);
						return (Instruction::LDrrNN(reg, n), format!("LD {}, {:#06x}", reg, n))
					},
					0b1001 => {
						let reg = Reg16::from(six_bit_rem >> 4, true);
						return (Instruction::ADDhlRR(reg), format!("ADD HL, {}", reg))
					},
					0b0010 => {
						let reg = Reg16::from(six_bit_rem >> 4, false);
						return (Instruction::LDmrrA(reg), format!("LD {}, A", reg))
					},
					0b1010 => {
						let reg = Reg16::from(six_bit_rem >> 4, false);
						return (Instruction::LDaMRR(reg), format!("LD A {}", reg))
					},
					0b0011 => {
						let reg = Reg16::from(six_bit_rem >> 4, false);
						return (Instruction::INCrr(reg), format!("INC {}", reg))
					},
					0b1011 => {
						let reg = Reg16::from(six_bit_rem >> 4, false);
						return (Instruction::DECrr(reg), format!("DEC {}", reg))
					},
					_ => {}
				}

				let half = half & 0b111;
				match half {
					0b100 => {
						let reg = Reg8::from(six_bit_rem >> 3);
						return (Instruction::INCr(reg), format!("INC {}", reg))
					},
					0b101 => {
						let reg = Reg8::from(six_bit_rem >> 3);
						return (Instruction::DECr(reg), format!("DEC {}", reg))
					},
					0b110 => {
						let reg = Reg8::from(six_bit_rem >> 3);
						let n = mem.get_imm8(pc);
						return (Instruction::LDrN(reg, n), format!("LD {}, {:#04x}", reg, n))
					},
					0b111 => {
						match six_bit_rem >> 4 {
							0 => return (Instruction::RdCA(Direction::from((six_bit_rem & 0b1000) >> 3)),
										 format!("RdCA")),
							1 => return (Instruction::RdA(Direction::from((six_bit_rem & 0b1000) >> 3)),
										format!("RdA")),
							_ => return (Instruction::Invalid, "Invalid".to_owned())
						}
					},
					0b000 => {
						let conditional = Conditional::from((six_bit_rem & 0b011000) >> 3);
						let n = mem.get_imm8(pc);
						return (Instruction::JRcE(conditional, n), format!("JR {}, {:#04x}", conditional, n))
					},
					_ => return (Instruction::Invalid, "Invalid".to_owned())
				}
			},
			0b01 => {
				let reg1 = Reg8::from(six_bit_rem >> 3);
				let reg2 = Reg8::from(six_bit_rem & 0b111);

				if (reg1 == Reg8::MHL) && (reg2 == Reg8::MHL) {
					return (Instruction::HALT, "HALT".to_owned())
				}
				return (Instruction::LDrR(reg1, reg2), format!("LD {}, {}", reg1, reg2))
			},
			0b10 => {
				let operation = Operation::from((six_bit_rem & 0b111000) >> 3);
				let reg = Reg8::from(six_bit_rem & 0b000111);
				return (Instruction::ALUaR(operation, reg), format!("{} A, {}", operation, reg))
			},
			0b11 => {
				let header = six_bit_rem & 0b111;
				match header {
					0b110 => {
						let operation = Operation::from((six_bit_rem & 0b111000) >> 3);
						let n = mem.get_imm8(pc);
						return (Instruction::ALUaN(operation, n), format!("{} A, {:#04x}", operation, n))
					},
					0b001 => {
						let reg = Reg16::from(six_bit_rem >> 4, false);
						return (Instruction::POPrr(reg), format!("POP {}", reg))
					},
					0b101 => {
						let reg = Reg16::from(six_bit_rem >> 4, false);
						return (Instruction::PUSHrr(reg), format!("PUSH {}", reg))
					}
					0b111 => {
						let n = six_bit_rem & 0b111000;
						return (Instruction::RSTn(n), format!("RST {:#04}", n))
					},
					0b000 => {
						let conditional = Conditional::from((six_bit_rem & 0b011000) >> 3);
						return (Instruction::RETc(conditional), format!("RET {}", conditional))
					},
					0b010 => {
						let conditional = Conditional::from((six_bit_rem & 0b011000) >> 3);
						let n = mem.get_imm16(pc);
						return (Instruction::JPcNN(conditional, n), format!("JP {}, {:#06x}", conditional, n))
					},
					0b100 => {
						let conditional = Conditional::from((six_bit_rem & 0b011000) >> 3);
						let n = mem.get_imm16(pc);
						return (Instruction::CALLcNN(conditional, n), format!("CALL {}, {:#06x}", conditional, n))
					},
					_ => return (Instruction::Invalid, "Invalid".to_owned())
				}
			},
			_ => {}
		}

		(Instruction::Unknown, "Unknown".to_owned())
	}
}