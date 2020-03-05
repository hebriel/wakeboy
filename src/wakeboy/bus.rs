use super::core::*;

const ROM_SPACE_BEGIN:		usize = 0x0000;
const ROM_SPACE_END:		usize = 0x8000-1;
const VRAM_BEGIN:			usize = 0x8000;
const VRAM_END:				usize = 0xA000-1;
const EXTERNAL_RAM_BEGIN: 	usize = 0xA000;
const EXTERNAL_RAM_END:		usize = 0xC000-1;
const RAM_BEGIN: 			usize = 0xC000;
const RAM_END: 				usize = 0xE000-1;
const OAM_RAM_BEGIN:		usize = 0xFE00;
const OAM_RAM_END:			usize = 0xFEA0-1;
const IO_RAM_BEGIN:			usize = 0xFF00;
const IO_RAM_END: 			usize = 0xFF80-1;
const HRAM_BEGIN: 			usize = 0xFF80;
const HRAM_END: 			usize = 0xFFFF;

pub struct MemoryBus {
	rom_mem:	[u8; ROM_SPACE_END - ROM_SPACE_BEGIN + 1],
	vram_mem:	[u8; VRAM_END - VRAM_BEGIN + 1],
	extern_mem:	[u8; EXTERNAL_RAM_END - EXTERNAL_RAM_BEGIN + 1],
	ram_mem: 	[u8; RAM_END - RAM_BEGIN + 1],
	oam_mem: 	[u8; OAM_RAM_END - OAM_RAM_BEGIN + 1],
	io_ram_mem:	[u8; IO_RAM_END - IO_RAM_BEGIN + 1],
	hram_mem: 	[u8; HRAM_END - HRAM_BEGIN + 1],
}

impl MemoryBus {

	pub fn get_imm8(&self, pc: &mut u16) -> u8 {
		*pc += 1;
		let ret = self.get_byte(*pc as usize);
		ret.unwrap()
	}

	pub fn get_imm16(&self, pc: &mut u16) -> u16 {
		*pc += 1;
		let ret = self.get_2bytes(*pc as usize);
		*pc += 1;
		ret.unwrap()
	}

	pub fn get_byte(&self, address: usize) -> Option<u8> {
		return match address {
			ROM_SPACE_BEGIN ..= ROM_SPACE_END => {
				Some(self.rom_mem[address])
			},
			VRAM_BEGIN ..= VRAM_END => {
				Some(self.vram_mem[address - VRAM_BEGIN])
			},
			EXTERNAL_RAM_BEGIN ..= EXTERNAL_RAM_END => {
				Some(self.extern_mem[address - EXTERNAL_RAM_BEGIN])
			},
			RAM_BEGIN ..= RAM_END => {
				Some(self.ram_mem[address - RAM_BEGIN])
			},
			OAM_RAM_BEGIN ..= OAM_RAM_END => {
				Some(self.oam_mem[address - OAM_RAM_BEGIN])
			},
			IO_RAM_BEGIN ..= IO_RAM_END => {
				Some(self.io_ram_mem[address - IO_RAM_BEGIN])
			},
			HRAM_BEGIN ..= HRAM_END => {
				Some(self.hram_mem[address - HRAM_BEGIN])
			},
			_ => {
				warn_or_crash(String::from("CPU tried to access unassigned part of memory"));
				None
			}
		}
	}

	pub fn get_2bytes(&self, address: usize) -> Option<u16> {
		return match address {
			ROM_SPACE_BEGIN ..= ROM_SPACE_END => {
				Some(combine_bytes( self.rom_mem[address], self.rom_mem[address + 1]))
			},
			VRAM_BEGIN ..= VRAM_END => {
				Some(combine_bytes( self.vram_mem[address - VRAM_BEGIN],
									self.vram_mem[address - VRAM_BEGIN + 1]))
			},
			EXTERNAL_RAM_BEGIN ..= EXTERNAL_RAM_END => {
				Some(combine_bytes( self.extern_mem[address - EXTERNAL_RAM_BEGIN],
									self.extern_mem[address - EXTERNAL_RAM_BEGIN + 1]))
			},
			RAM_BEGIN ..= RAM_END => {
				Some(combine_bytes(	self.ram_mem[address - RAM_BEGIN],
									self.ram_mem[address - RAM_BEGIN + 1]))
			},
			OAM_RAM_BEGIN ..= OAM_RAM_END => {
				Some(combine_bytes( self.oam_mem[address - OAM_RAM_BEGIN],
									self.oam_mem[address - OAM_RAM_BEGIN + 1]))
			},
			IO_RAM_BEGIN ..= IO_RAM_END => {
				Some(combine_bytes( self.io_ram_mem[address - IO_RAM_BEGIN],
									self.io_ram_mem[address - IO_RAM_BEGIN + 1]))
			},
			HRAM_BEGIN ..= HRAM_END => {
				Some(combine_bytes( self.hram_mem[address - HRAM_BEGIN],
									self.hram_mem[address - HRAM_BEGIN + 1]))
			},
			_ => {
				warn_or_crash(String::from("CPU tried to access unassigned part of memory"));
				None
			}
		}
	}

	pub fn write_byte(&mut self, address: usize, data: u8) {
		match address {
			ROM_SPACE_BEGIN ..= ROM_SPACE_END => {
				self.rom_mem[address] = data;
			},
			VRAM_BEGIN ..= VRAM_END => {
				self.vram_mem[address - VRAM_BEGIN] = data;
			},
			EXTERNAL_RAM_BEGIN ..= EXTERNAL_RAM_END => {
				self.extern_mem[address - EXTERNAL_RAM_BEGIN] = data;
			},
			RAM_BEGIN ..= RAM_END => {
				self.ram_mem[address - RAM_BEGIN] = data;
			},
			OAM_RAM_BEGIN ..= OAM_RAM_END => {
				self.oam_mem[address - OAM_RAM_BEGIN] = data;
			},
			IO_RAM_BEGIN ..= IO_RAM_END => {
				self.io_ram_mem[address - IO_RAM_BEGIN] = data;
			},
			HRAM_BEGIN ..= HRAM_END => {
				self.hram_mem[address - HRAM_BEGIN] = data;
			},
			_ => {
				warn_or_crash(String::from("CPU tried to write on an unassigned part of memory"));
			}
		}
	}

	pub fn write_2bytes(&mut self, address: usize, data: u16) {
		match address {
			ROM_SPACE_BEGIN ..= ROM_SPACE_END => {
				self.rom_mem[address] = (data << 8) as u8;
				self.rom_mem[address + 1] = (data & 0xFF) as u8;
			},
			VRAM_BEGIN ..= VRAM_END => {
				self.vram_mem[address - VRAM_BEGIN] = (data << 8) as u8;
				self.vram_mem[address - VRAM_BEGIN + 1] = (data & 0xFF) as u8;
			},
			EXTERNAL_RAM_BEGIN ..= EXTERNAL_RAM_END => {
				self.extern_mem[address - EXTERNAL_RAM_BEGIN] = (data << 8) as u8;
				self.extern_mem[address - EXTERNAL_RAM_BEGIN + 1] = (data & 0xFF) as u8;
			},
			RAM_BEGIN ..= RAM_END => {
				self.ram_mem[address - RAM_BEGIN] = (data << 8) as u8;
				self.ram_mem[address - RAM_BEGIN + 1] = (data & 0xFF) as u8;
			},
			OAM_RAM_BEGIN ..= OAM_RAM_END => {
				self.oam_mem[address - OAM_RAM_BEGIN] = (data << 8) as u8;
				self.oam_mem[address - OAM_RAM_BEGIN + 1] = (data & 0xFF) as u8;
			},
			IO_RAM_BEGIN ..= IO_RAM_END => {
				self.io_ram_mem[address - IO_RAM_BEGIN] = (data << 8) as u8;
				self.io_ram_mem[address - IO_RAM_BEGIN + 1] = (data & 0xFF) as u8;
			},
			HRAM_BEGIN ..= HRAM_END => {
				self.hram_mem[address - HRAM_BEGIN] = (data << 8) as u8;
				self.hram_mem[address - HRAM_BEGIN + 1] = (data & 0xFF) as u8;
			},
			_ => {
				warn_or_crash(String::from("CPU tried to write on an unassigned part of memory"));
			},
		}
	}

	pub fn load_boot_rom(&mut self, boot_rom: &[u8]) {
		&self.rom_mem[..256].clone_from_slice(boot_rom);
	}
}

impl Default for MemoryBus {
	fn default() -> Self {
		MemoryBus {
			rom_mem:	[0; ROM_SPACE_END - ROM_SPACE_BEGIN + 1],
			vram_mem:	[0; VRAM_END - VRAM_BEGIN + 1],
			extern_mem:	[0; EXTERNAL_RAM_END - EXTERNAL_RAM_BEGIN + 1],
			ram_mem: 	[0; RAM_END - RAM_BEGIN + 1],
			oam_mem: 	[0; OAM_RAM_END - OAM_RAM_BEGIN + 1],
			io_ram_mem:	[0; IO_RAM_END - IO_RAM_BEGIN + 1],
			hram_mem: 	[0; HRAM_END - HRAM_BEGIN + 1],
		}
	}
}