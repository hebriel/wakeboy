mod rustyboi;

use structopt::StructOpt;
use std::path::PathBuf;
use rustyboi::core::*;
use rustyboi::cpu::*;
use colored::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustyboi-i")]
struct Opt {
    /// Rom file to execute
    #[structopt(short, long, parse(from_os_str), default_value = "__none")]
    input: PathBuf,
    
    /// Enable debug tools [NOT IMPLEMENTED]
    #[structopt(short, long)]
    debug: bool,

    /// Turn warnings into crashes
    #[structopt(short, long)]
    strict: bool,

    /// Path to an alternative boot rom (should be 256 bytes long)
    #[structopt(short, long, default_value = "__none")]
    boot_rom: String,
}

fn get_path() -> std::io::Result<PathBuf> {
    let exe = std::env::current_exe()?;
    let dir = exe.parent().expect("Executable must be in a directory");
    let dir = dir.join("resources");
    Ok(dir)
}

fn main() {
    let mut opt = Opt::from_args();

    unsafe {
        GLOBAL_FLAGS.is_strict = opt.strict;
    }

    if opt.boot_rom == "__none" {
        let res_path = get_path().unwrap();
        opt.boot_rom = format!("{}/{}", res_path.to_str().unwrap(), "boot_rom/dmg_boot.bin");
    }

    if opt.input.to_str().unwrap() == "__none" {
        let res_path = get_path().unwrap();
        opt.input = PathBuf::from(  format!("{}/{}",
                                    res_path.to_str().unwrap(),
                                    "default_rom/infinite_loop.bin"));
    }

    let boot_rom: Vec<u8> = match read_rom(&opt.boot_rom) {
        Some(b) => b,
        None => {
            println!("{} ({}) {}", "Error: Boot rom file".red(), opt.boot_rom, "wasn't found or is empty".red());
            std::process::exit(0);
        }
    };

    let len = boot_rom.len();
    if len < 256 {
        warn_or_crash(format!("Boot rom is smaller than 256 bytes\n{} bytes will be filled with 0's", 256 - len));
    } else if len > 256 {
        warn_or_crash(format!("Boot rom is larger than 256 bytes and will so be truncated\n{} bytes will be discarded", len - 256));
    }

    let rom: Vec<u8> = match opt.input.to_str() {
        Some(a) => {
            match read_rom(&String::from(a)) {
                Some(b) => b,
                None => {
                    println!("{}", "Error: Rom file wasn't found or is empty".red());
                    std::process::exit(0);
                }
            }
        }
        None => panic!("Invalid input"),
    };

    let mut cpu: CPU = Default::default();
    cpu.memory.load_boot_rom(&boot_rom[..256]);

    cpu.run();
}
