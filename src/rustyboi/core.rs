use std::fs::File;
use std::io::prelude::*;

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