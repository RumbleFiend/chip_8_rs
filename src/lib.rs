pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096; //memory size
const NUM_REGS: usize = 16;  // number of v registers
const STACK_SIZE: usize = 16; // stack size
const NUM_KEYS: usize = 16; // num key size
const START_ADDR: u16 = 0x200;
const FONT_SIZE: usize = 80; // font size

const FONTS:[u8;FONT_SIZE] = [
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

pub struct Emu{   // Emulation program
    ram : [u8;RAM_SIZE] ,// Memory
    display: [bool; SCREEN_WIDTH * SCREEN_HEIGHT], //screen display
    pc: u16,  // program counter
    i_reg: u16, // I register
    stack: [u16;STACK_SIZE], // stack
    sp:u16, // stack pointer
    dt:u16, // delay timer
    st:u16, // sound timer
    keys: [bool;NUM_KEYS],
    v_reg: [u8; NUM_REGS], // V Registers
    
}

impl Emu { // initialisation
    pub fn new() -> Self{
        Self {
            ram: [0;RAM_SIZE],
            display: [false;SCREEN_WIDTH*SCREEN_HEIGHT] ,
            pc: START_ADDR,
            i_reg: 0,
            stack: [0; STACK_SIZE],
            sp: 0,
            dt: 0,
            st: 0,
            keys: [false;NUM_KEYS],
            v_reg: [0;NUM_REGS] }
    }
}