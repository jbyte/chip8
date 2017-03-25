const INTERPRETER_START: u32 = 0x000;
const INTERPRETER_END: u32 = 0x1FF;
const INTERPRETER_LENGTH: u32 = INTERPRETER_END - INTERPRETER_START;

const FONT_SET_START: usize = 0x050;
const FONT_SET_END: usize = 0x0A0;
const FONT_SET_LENGTH: usize = FONT_SET_END - FONT_SET_START;

const DATA_START: usize = 0x200;
const DATA_END: usize = 0xFFF;
const DATA_LENGTH: usize = DATA_END - DATA_START;

const MEM_LENGTH: usize = 4096;
const STACK_LEVEL: usize = 16;
const REG_NUM: usize = 16;
const GFX_WIDTH: usize = 64;
const GFX_HEIGHT: usize = 32;

const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub fn init() -> Cpu {
    let mut ret = Cpu {
        pc: 0x200,
        opcode: 0,
        memory: vec![0; MEM_LENGTH],
        reg: vec![0; REG_NUM],
        index: 0,
        sp: 0,
        stack: vec![0; STACK_LEVEL],
        // gfx: vec![vec![0; GFX_HEIGHT]; GFX_WIDTH],
        gfx: vec![0; GFX_WIDTH * GFX_HEIGHT],
        delay_timer: 0,
        sound_timer: 0,
        key: vec![0; STACK_LEVEL],
    };
    ret.load_fontset();
    ret
}

#[derive(Debug)]
pub struct Cpu {
    pc: usize,
    opcode: u16,
    memory: Vec<u8>,
    reg: Vec<u8>,
    index: usize,
    // gfx: Vec<Vec<u8>>,
    gfx: Vec<u8>,
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<usize>,
    sp: usize,
    key: Vec<u8>,
}

impl Cpu {
    fn load_fontset(&mut self) {
        for i in 0..FONT_SET_LENGTH {
            self.memory[FONT_SET_START + i] = FONTSET[i];
        }
    }

    pub fn load_rom(&mut self, rom: Box<[u8]>) {
        for i in 0..rom.len() {
            self.memory[DATA_START + i] = rom[i];
        }
    }

    pub fn emulate_cycle(&mut self) {
        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc+1] as u16);

        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x0F00 {
                    0x0000 => {
                        match self.opcode & 0x00FF {
                            0x00EE => {
                                self.sp -= 1;
                                self.pc = self.stack[self.sp];
                            },
                            _ => panic!("Unknown opcode or not implemented yet: {:X}", self.opcode)
                        }
                    },
                    _ => panic!("Unknown opcode or not implemented yet: {:X}", self.opcode)
                }
            },
            0x2000 => { // 2NNN: Calls subroutine at NNN
                let addr = self.opcode & 0x0FFF;
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = addr as usize;
            },
            0x6000 => { // 6XNN: Sets VX to NN
                let index = (self.opcode & 0x0F00) >> 8;
                let val = self.opcode & 0x00FF;
                self.reg[index as usize] = val as u8;
                self.pc += 2;
            },
            0x7000 => { // 7XNN: Adds NN to VX
                let x = (self.opcode & 0x0F00) >> 8;
                let nn = self.opcode & 0x00FF;
                self.reg[x as usize] += nn as u8;
                self.pc += 2;
            },
            0xA000 => { // ANNN: Sets I to the address NNN
                let val = self.opcode & 0x0FFF;
                self.index = val as usize;
                self.pc += 2;
            },
            0xD000 => { // DXYN: Draws sprite at cordinate VX, VY with height N
                let x = (self.opcode & 0x0F00) >> 8;
                let y = (self.opcode & 0x00F0) >> 4;
                let n = self.opcode & 0x000F;
                let mut pix: u8;

                let tmp = self.index as usize;
                let sprite = &self.memory[tmp..(tmp + (n as usize))];

                self.reg[0xF] = 0;
                for i in 0..n {
                    pix = (&sprite)[i as usize];
                    for j in 0..8 {
                        if (pix & (0x80 >> j)) != 0 {
                            if self.gfx[(x + j + ((y + i) * 64)) as usize] == 1 {
                                self.reg[0xF] = 1;
                            }
                            self.gfx[(x + j + ((y + i) * 64)) as usize] ^= 1;
                        }
                    }
                }
                self.pc += 2;
            },
            0xF000 => {
                match self.opcode & 0x00FF {
                    0x0029 => { // FX29: set I to character sprite in VX
                        let x = (self.opcode & 0x0F00) >> 8;
                        let tmp = FONT_SET_START + ((self.reg[x as usize] * 5) as usize);
                        self.index = self.memory[tmp] as usize;
                        self.pc += 2;
                    },
                    0x0033 => { // FX33: set memory at I:I+2 to VX
                        let x = (self.opcode & 0x0F00) >> 8;
                        let val = self.reg[x as usize];
                        self.memory[self.index] = val / 100;
                        self.memory[self.index + 1] = (val / 10) % 10;
                        self.memory[self.index + 2] = val % 10;
                        self.pc += 2;
                    },
                    0x0065 => { // FX65: fill V0 to VX with values at I:I+X
                        let x = (self.opcode & 0x0F00) >> 8;
                        for i in 0..x {
                            self.reg[i as usize] = self.memory[self.index + (i as usize)];
                        }
                        self.pc += 2;
                    },
                    _ => panic!("Unknown opcode or not implemented yet: {:X}", self.opcode)
                }
            },
            _ => panic!("Unknown opcode or not implemented yet: {:X}", self.opcode)
        }
    }
}
