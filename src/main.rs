#[macro_use]
extern crate nom;
extern crate rand;

use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

mod cpu;
mod debugger;

use debugger::Debugger;

fn main() {
    let rom_name: String;
    let debug: String;

    let file = env::args().nth(2);
    let flag = env::args().nth(1);

    match file {
        None => {
            match flag {
                Some(_) => {
                    rom_name = flag.unwrap();
                    debug = String::new();
                },
                None => panic!("Where the args at? Do need a file atleast.")
            }
        },
        Some(_) => {
            rom_name = file.unwrap();
            debug = flag.unwrap();
        }
    }
    let rom = read_rom(rom_name);
    //TODO: setup graphics
    //TODO: setup input

    let mut cpu = cpu::cpu::init();
    cpu.load_rom(rom);

    match debug.as_ref() {
        "-d" | "--debug" => {
            // TODO: run with debugger
            let mut debug = Debugger::new(cpu);
            debug.run();
        }
        _ => {
            loop {
                cpu.emulate_cycle();
            }
        }
    }
}

fn read_rom<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
