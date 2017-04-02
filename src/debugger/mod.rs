mod command;

use std::io::{stdin, stdout};
use std::io::prelude::*;
use std::borrow::Cow;

use cpu::cpu::Cpu;
use cpu::cpu::GFX_HEIGHT;
use cpu::cpu::GFX_WIDTH;
use self::command::Command;

pub struct Debugger {
    cpu: Cpu,
    last_commad: Option<Command>,
}

impl Debugger {
    pub fn new(cpu: Cpu) -> Debugger {
        Debugger {
            cpu: cpu,
            last_commad: None,
        }
    }

    pub fn run(&mut self) {
        let mut cnt = 1;
        loop {
            print!("chip8:{}> ", cnt);
            stdout().flush().unwrap();

            let command = match (read_stdin().parse(), self.last_commad) {
                (Ok(Command::Repeat), Some(c)) => Ok(c),
                (Ok(Command::Repeat), None) => Err("No last command".into()),
                (Ok(c), _) => Ok(c),
                (Err(e), _) => Err(e),
            };

            match command {
                Ok(Command::Step(count)) => self.step(count),
                Ok(Command::Exit) => break,
                Ok(Command::Repeat) => unreachable!(),
                Err(ref e) => println!("{}", e),
            }

            self.last_commad = command.ok();
            cnt += 1;
        }
    }

    pub fn step(&mut self, count: usize) {
        for _ in 0..count {
            let curr_pc = self.cpu.curr_pc();
            let curr_index = self.cpu.curr_index();
            let curr_opcode = self.cpu.curr_opcode();
            let curr_regs = self.cpu.curr_regs();
            let curr_stack = self.cpu.curr_stack();
            let curr_sp = self.cpu.curr_sp();
            let curr_gfx = self.cpu.curr_gfx();
            let curr_dt = self.cpu.curr_dt();
            let curr_st = self.cpu.curr_st();

            print!("pc:{:04X},", curr_pc);
            print!("index:{:04X},", curr_index);
            print!("delay:{:02X},", curr_dt);
            print!("sound:{:02X},", curr_st);
            println!("opcode:{:04X}", curr_opcode);
            println!("=============================================");

            println!("stack:");
            print_stack(curr_stack, curr_sp);
            println!("=============================================");
            println!("registers:");
            print_regs(curr_regs);
            println!("=============================================");
            println!("screen:");
            print_gfx(curr_gfx);
            println!("=============================================");

            self.cpu.emulate_cycle();
        }
    }
}

fn print_gfx(gfx: Vec<u8>) {
    for _ in 0..GFX_WIDTH {
        print!("-");
    }
    println!("--");
    for i in 0..GFX_HEIGHT {
        print!("|");
        for j in 0..GFX_WIDTH {
            // print!("{}", gfx[(i * GFX_WIDTH + j) as usize]);
            if gfx[(i * GFX_WIDTH + j) as usize] == 1 {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!("|");
    }
    for _ in 0..GFX_WIDTH {
        print!("-");
    }
    println!("--");
}

fn print_regs(regs: Vec<u8>) {
    println!("| 0| 1| 2| 3| 4| 5| 6| 7| 8| 9| A| B| C| D| E| F|");
    for i in 0..regs.len() {
        print!("|{:02X}", regs[i as usize]);
    }
    println!("|");
}

fn print_stack(stack: Vec<usize>, sp: usize) {
    for i in 0..stack.len() {
        print!("|");
        if i == sp {
            print!("[");
        }
        print!("{:04X}", stack[i as usize]);
        if i == sp {
            print!("]");
        }
    }
    println!("|");
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}
