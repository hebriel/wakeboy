use super::direction::*;
use super::conditionals::*;
use super::registers::{Reg8, Reg16};
use super::*;

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
	ALUaR		(Reg8),
	ALUaN		(u8),
	POPrr		(Reg16),
	PUSHrr		(Reg16),
	RSTn		(u8), //todo: wtf is N >> 3 (in the doc) ??
	RETc		(Conditional),
	RET,
	RETI,
	JPcNN		(Conditional, u16),
	JPnn 		(u16),
	CALLcNN		(Conditional, u16),
	CALLnn 		(u16),
	ADDspN		(u8),
	LDhlSPpN	(u8),
	LDmzpPn		(u8),
	LDaMZPpN	(u8),
	LDmcA,
	LDaMC,
	LDmnA		(u16),
	LDaMN 		(u16),
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
}

impl Instruction {
	pub fn fetch(mem: &mut bus::MemoryBus, pc: &mut u16) -> Instruction {
		Instruction::NOP
	}
}