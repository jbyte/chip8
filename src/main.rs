use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

pub mod cpu;

fn main() {
    let rom_name = env::args().nth(1).unwrap();
    let rom = read_rom(rom_name);
    //TODO: setup graphics
    //TODO: setup input

    let mut cpu = cpu::init();
    cpu.load_rom(rom);

    loop {
        cpu.emulate_cycle();
    }
}

fn read_rom<P: AsRef<Path>>(path: P) -> Box<[u8]> {
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
