use std::fs::File;
use std::io::prelude::*;
use colored::*;

pub static mut GLOBAL_FLAGS: GlobalFlags = GlobalFlags {
	is_strict: false,
};

pub struct GlobalFlags {
	pub is_strict: bool,
}

pub fn warn_or_crash(msg: String) {
	unsafe {
		if GLOBAL_FLAGS.is_strict {
			println!("Error: {}", msg.to_owned().red());
			std::process::exit(-1);
		} else {
			println!("Warning: {}", msg.to_owned().yellow());
		}
	}
}

pub fn read_rom(path: &String) -> Option<Vec<u8>> {
    let mut file = match File::open(path) {
        Err(_) => {
            return None
        },
        Ok(file) => file,
    };
    let mut contents = Vec::new();

    match file.read_to_end(&mut contents) {
        Ok(size) => {
            if size == 0 {
                return None
            }
        },
        Err(_) => {
            return None
        }
    }

    Some(contents)
}

#[inline(always)]
pub fn combine_bytes(b1: u8, b2: u8) -> u16 {
	((b2 as u16) << 8) | b1 as u16
}