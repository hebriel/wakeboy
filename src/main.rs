mod rustyboi;

use structopt::StructOpt;
use std::path::PathBuf;
use colored::*;
use rustyboi::core::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "rustyboi-i")]
struct Opt {
    /// Rom file to execute
    #[structopt(short, long, parse(from_os_str), default_value = "resources/default_rom/infinite_loop.bin")]
    input: PathBuf,
    
    /// Enable debug tools [NOT IMPLEMENTED]
    #[structopt(short, long)]
    debug: bool,

    /// Path to an alternative boot rom (should be 256 bytes long)
    #[structopt(short, long, default_value = "resources/boot_rom/dmg_boot.bin")]
    boot_rom: String,
}

fn main() {
    let opt = Opt::from_args();

    let boot_rom: Vec<u8> = match read_rom(&opt.boot_rom) {
        Some(b) => b,
        None => {
            println!("{}", "Error: Boot rom file wasn't found or is empty".red());
            std::process::exit(0);
        }
    };

    let len = boot_rom.len();
    if len < 256 {
        println!("{}\n{} {}", "Warning: Boot rom is smaller than 256 bytes".yellow(), 256 - len, "bytes will be filled with 0's".yellow());
    } else if len > 256 {
        println!("{}\n{} {}", "Warning: Boot rom is larger than 256 bytes and will so be truncated ".yellow(),len - 256, "bytes will be discarded".yellow());
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




}
