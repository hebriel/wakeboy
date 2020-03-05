use super::bus::*;
use super::registers::*;
use super::instructions::*;
use super::core::*;

pub struct CPU {
	pub memory: MemoryBus,
	pub registers: Registers,
}

impl CPU {
	pub fn run(&mut self) {
		loop {
			let old_pc = self.registers.pc;
			let (instruction, name) = Instruction::fetch(&mut self.memory, &mut self.registers.pc);

			match instruction {
				Instruction::Invalid => warn_or_crash(String::from("Invalid instruction")),
				Instruction::Unknown => warn_or_crash(String::from("Unknown instruction? That's not supposed to happen")),
				_ => println!("{} [{:#06x}]", name, old_pc),
			}

			let (new_pc, did_overflow) = self.execute(&instruction);

			if did_overflow {
				warn_or_crash("Program counter overflowed".to_owned());
			}
			self.registers.pc = new_pc;
		}
	}

	pub fn execute(&mut self, instruction: &Instruction) -> (u16, bool) {
		self.registers.pc.overflowing_add(1)
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