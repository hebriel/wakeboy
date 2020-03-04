use super::bus::*;
use super::registers::*;
use super::instructions::*;

pub struct CPU {
	pub memory: MemoryBus,
	pub registers: Registers,
}

impl CPU {
	pub fn run(&mut self) {
		loop {
			let instruction = Instruction::fetch(&mut self.memory, &mut self.registers.pc);
			self.registers.pc = self.execute(&instruction);
		}
	}

	pub fn execute(&mut self, instruction: &Instruction) -> u16 {

		match *instruction {
			Instruction::NOP => { println!("NOP") },
			Instruction::Unknown => { println!("Unknown") },
			_ => {},
		}

		self.registers.pc.wrapping_add(1)
	}
}

impl Default for CPU {
	fn default() -> Self {
		CPU {
			memory: Default::default(),
			registers: Default::default(),
		}
	}
}