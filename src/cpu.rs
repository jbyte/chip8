const INTERPRETER_START: u32 = 0x000;
const INTERPRETER_END: u32 = 0x1FF;
const INTERPRETER_LENGTH: u32 = INTERPRETER_END - INTERPRETER_START;

const FONT_SET_START: u32 = 0x050;
const FONT_SET_END: u32 = 0x0A0;
const FONT_SET_LENGTH: u32 = FONT_SET_END - FONT_SET_START;

const DATA_START: u32 = 0x200;
const DATA_END: u32 = 0xFFF;
const DATA_LENGTH: u32 = DATA_END - DATA_START;

const MEM_LENGTH: usize = 4096;
const STACK_LEVEL: usize = 16;
const REG_NUM: usize = 16;
const GFX_WIDTH: usize = 64;
const GFX_HEIGHT: usize = 32;

pub fn init() -> Cpu {
    Cpu {
        pc: 0x200,
        opcode: 0,
        memory: vec![0; MEM_LENGTH],
        reg: vec![0; REG_NUM],
        index: 0,
        sp: 0,
        stack: vec![0; STACK_LEVEL],
        gfx: vec![0; GFX_WIDTH * GFX_HEIGHT],
        delay_timer: 0,
        sound_timer: 0,
        key: vec![0; STACK_LEVEL],
    }
}

pub struct Cpu {
    pc: u16,
    opcode: u16,
    memory: Vec<u8>,
    reg: Vec<u8>,
    index: u16,
    gfx: Vec<u8>,
    delay_timer: u8,
    sound_timer: u8,
    stack: Vec<u8>,
    sp: u8,
    key: Vec<u8>,
}
