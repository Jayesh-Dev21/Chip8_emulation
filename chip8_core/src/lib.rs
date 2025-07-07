// constants
const RAM_SIZE: usize = 4096;

// display
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 64;

// 16 registers for chip 8
const NUM_REGS: usize = 16;

//stack
const STACK_SIZE: usize = 16;

//addrers to start loading the ram -> 512 bytes   
const START_ADDR: u16 = 0x200; // let because of the old architecture of chip8, to do required calculations, to run 

const NUM_KEYS: usize = 16;

// Defined font set

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0x90, // for 0
    0x20, 0x60, 0x20, 0x20, 0x70, // for 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // for 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // for 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // for 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // for 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // for 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // for 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // for 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // for 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // for A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // for B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // for C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // for D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // for E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // for F
];
// Main object, for emulation class
pub struct EMU{
    program_counter: u16, // program counter
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH*SCREEN_HEIGHT],
    v_reg: [u8; NUM_REGS], // value/variable registers
    i_reg: u16, // index reg
    sp: u16, //stack pointer
    stack: [u16; STACK_SIZE],
    keys: [bool; NUM_KEYS],
    st: u8, //sound timer
    dt: u8, //delay timer
}


impl EMU{
    pub fn new() -> Self{
        let mut new_emu = Self{
            program_counter: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH*SCREEN_HEIGHT],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            st: 0,
            dt: 0,
        };

        new_emu.ram[0..(FONTSET_SIZE as usize)].copy_from_slice(&FONTSET);

        new_emu

        // ? DONE with the bascis ? //
    }

    pub fn reset(&mut self){
        self.program_counter= START_ADDR;
        self.ram = [0; RAM_SIZE ];
        self.screen = [false; SCREEN_WIDTH*SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.st = 0;
        self.dt = 0;
        self.ram[0..(FONTSET_SIZE as usize)].copy_from_slice(&FONTSET);
    }

    //Basis of LIFO
    // Implemeted push and pop
    fn push(&mut self, val: u16){
        self.stack[self.sp as usize] = val; // converts *sp* to type usize from u16
        self.sp += 1;
    }

    fn pop(&mut self) -> u16{
        self.sp -= 1;
        self.stack[self.sp as usize] // scope for underflow panic, try later
    }

    pub fn tick(&mut self){
        //fetch
        let op: u16 = self.fetch(); // opcode
        // Decode & Execute
        self.execute(op);
    }

    fn execute(&mut self, op: u16){
        let digit1: u16 = (op & 0xF000) >> 12;
        let digit2: u16 = (op & 0xF000) >> 8;
        let digit3: u16 = (op & 0xF000) >> 4;
        let digit4: u16 = (op & 0xF000);

        match(digit1,digit2,digit3,digit4){

            //  Time for OP

            // 0000 - Nop
            (0,0,0,0) => return,

            // 00E0 - Clear Screen
            (0,0,0xE,0) => {
                self.screen = [false; SCREEN_WIDTH*SCREEN_HEIGHT];
            },

            // 00EE - Return from Subroutine

            (_,_,_,_) => unimplemented!("Unimplimented opcode: {}", op),
        }

    }

    fn fetch(&mut self) -> u16{
        let higher_byte: u16 = self.ram[self.program_counter as usize] as u16;
        let lower_byte: u16 = self.ram[(self.program_counter + 1) as usize] as u16;
        let op: u16 = (higher_byte << 8) | lower_byte;
        self.program_counter += 2;
        op
        /* Example
        higher_byte = 0xA2 as u16 = 0x00A2
        lower_byte  = 0xF0 as u16 = 0x00F0
        op          = (0xA2 << 8) | 0xF0
                    = 0xA200 | 0x00F0
                    = 0xA2F0

        */
    }

    pub fn tick_timers(&mut self){
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if 1 == self.st { // yoda style
                // Audio work here
            }
            self.st -= 1;
        }
    }
}






