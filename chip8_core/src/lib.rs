use rand::random;

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

/*
    ref - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#keyboard
    for example 1 is represented as: 

    * *  * []  * * * * - 0x20
    * * [] []  * * * * - 0x60
    * *  * []  * * * * - 0x20
    * *  * []  * * * * - 0x20
    * * [] [] [] * * * - 0x70
*/
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

    pub fn get_dislay(&self) -> &[bool]{
        return &self.screen;
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
        let digit4: u16 = op & 0xF000;

        match(digit1,digit2,digit3,digit4){

            //  Time for OP \\

            // 0000 - Nop \\
            (0,0,0,0) => return,

            // 00E0 - Clear Screen \\
            (0,0,0xE,0) => {
                self.screen = [false; SCREEN_WIDTH*SCREEN_HEIGHT];
            },

            // 00EE - Return from Subroutine \\
            (0,0,0xE,0xE) => {
                let ret_addr: u16 = self.pop();

                self.program_counter = ret_addr;
            },

            // 1NNN - Jump \\
            (1,_,_,_) => {
                let nnn: u16 = op & 0xFFF;
                self.program_counter = nnn;
            },

            // 2NNNN - Call Subroutine \\
            (2,_,_,_) => {
                let nnn: u16 = op & 0xFFF;
                self.push(self.program_counter);
                self.program_counter = nnn;
            },

            // 3XNN - Skip next if VX == NN \\
            (3,_,_,_) => {
                let x: usize = digit2 as usize;
                let nn: u8 = (op & 0xFF) as u8;
                if self.v_reg[x] == nn{
                    self.program_counter += 2;
                }
            },

            // 4XNN - Skip next if VX == NN \\
            (4,_,_,_) => {
                let x: usize = digit2 as usize;
                let nn: u8 = (op & 0xFF) as u8;
                if self.v_reg[x] != nn{
                    self.program_counter += 2;
                }
            },

            // 5XY0 - Skip next if VX == VY \\
            (5,_,_,0) => {
                let x: usize = digit2 as usize;
                let y: usize = digit2 as usize;
                if self.v_reg[x] == self.v_reg[y]{
                    self.program_counter += 2;
                }
            },

            // 6XNN - VX = NN \\
            (6,_,_,_) => {
                let x: usize = digit2 as usize;
                let nn: u8 = (op & 0xFF) as u8;
                self.v_reg[x] = nn;
            },

            // 7XNN - VX += NN \\
            (7,_,_,_) => {
                let x: usize = digit2 as usize;
                let nn: u8 = (op & 0xFF) as u8;
                self.v_reg[x] = self.v_reg[x].wrapping_add(nn);
            },

            // 8XY0 - Skip next if VX == VY \\
            (8,_,_,0) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;
                self.v_reg[x] = self.v_reg[y]
            },
            // BITWISE \\
            // 8XY1 - Bitwise OR VX |= VY \\
            (8,_,_,1) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;
                self.v_reg[x] |= self.v_reg[y]
            },

            // 8XY2 - Bitwise AND VX &= VY \\
            (8,_,_,2) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;
                self.v_reg[x] &= self.v_reg[y]
            },

            // 8XY3 - Bitwise XOR VX ^= VY \\
            (8,_,_,3) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;
                self.v_reg[x] ^= self.v_reg[y]
            },

            // 8XY4 - VX += VY \\
            (8,_,_,4) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;
                
                let (new_vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let new_vf = if carry{1}else{0}; // returns if overflow while operation is 1/0
                // uses VF - Flag register
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },

            // 8XY5 - VX -= VY \\
            (8,_,_,5) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;
                
                let (new_vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let new_vf = if borrow{0}else{1}; // returns if underflow while operation is 0/1
                // uses VF - Flag register
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            },

            // 8XY6 - VX >>= 1 \\
            (8,_,_,6) => {
                let x: usize = digit2 as usize;
                let lsb: u8 = self.v_reg[x] & 1; // least sig bit

                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            },

           // 8XY7 - VX = VY - VX \\
            (8,_,_,7) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;
                
                let (new_vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let new_vf = if borrow{0}else{1}; // returns if underflow while operation is 0/1
                // uses VF - Flag register
                self.v_reg[x] = new_vx;
                self.v_reg[0xF] = new_vf;
            }, 

            // 8XYE - VX <<= 1 \\
            (8,_,_,0xE) => {
                let x: usize = digit2 as usize;
                let msb: u8 = self.v_reg[x] & 1; // most sig bit

                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            },

            // 9XY0 - Skip if VX != VY \\
            (9,_,_,0) => {
                let x: usize = digit2 as usize;
                let y: usize = digit3 as usize;

                if self.v_reg[x] != self.v_reg[y]{
                    self.program_counter += 2;
                }
            },

            // ANNN - I = NNN \\
            (0xA,_,_,_) => {
                let nnn: u16 = op & 0xFFF;
                self.i_reg = nnn;
            },

            // BNNN - Jump to V0 + NNN \\
            (0xB,_,_,_) => {
                let nnn: u16 = op & 0xFFF;
                self.program_counter = (self.v_reg[0] as u16) + nnn;
            },

            // CXNN - VX = rand() & NN \\
            (0xC,_,_,_) => {
                let x: usize = digit2 as usize;
                let nn: u8 = (op & 0xFF) as u8;

                let rng: u8 = random();
                self.v_reg[x] = rng & nn;
            },

            // DXYV - Draw Sprite \\
            (0xD,_,_,_) => {
                // at (x,y) co - ords
                let x_cord: u16 = self.v_reg[digit2 as usize] as u16;
                let y_cord: u16 = self.v_reg[digit3 as usize] as u16;

                let num_rows: u16 = digit4;
                
                let mut flipped: bool = false; // to track flipped pixels

                for y_line in 0..num_rows{
                    let addr: u16 = self.i_reg + y_line as u16;
                    let pixels: u8 = self.ram[addr as usize];
                    
                    for x_line in 0..8{
                        if (pixels & (0b1000_0000 >> x_line)) != 0{
                            let x: usize = (x_cord as usize + x_line) % SCREEN_WIDTH;
                            let y: usize = (y_cord as usize + y_line as usize) % SCREEN_HEIGHT;

                            let idx = x + SCREEN_WIDTH*y;
                            flipped |= self.screen[idx];
                            self.screen[idx] ^= true;
                        }
                    }
                }

                if flipped{
                    self.v_reg[0xF] = 1;
                }
                else{
                    self.v_reg[0xF] = 0;
                }
            },

            // EX9E - Skip if Key Pressed \\
            (0xE,_,9,0xE) => {
                let x: usize = digit2 as usize;
                let vx = self.v_reg[x];
                let key = self.keys[vx as usize];
                if key{
                    self.program_counter += 2;
                }
            },

            // EXA1 - Skip if Key Not Pressed \\
            (0xE,0xA,_,1) => {
                let x: usize = digit3 as usize;
                let vx = self.v_reg[x];
                let key: bool = self.keys[vx as usize];
                if !key{
                    self.program_counter += 2;
                }
            },

            // FX07 - VX = DT \\
            (0xF,_,0,7) => {
                let x: usize = digit2 as usize;
                self.v_reg[x] = self.dt;
            },

            // FX0A - Wait for a Key Press \\
            (0xF,_,0,0xA) => {
                let x: usize = digit2 as usize;
                let mut key_pressed: bool = false;
                for i in 0..self.keys.len(){
                    if self.keys[i]{
                        self.v_reg[x] = i as u8;
                        key_pressed = true;
                        break;
                    }
                }
                if !key_pressed{
                    self.program_counter -= 2; // re run the operation
                }
            },

            // FX15 - DT = VX \\
            (0xF,_,1,5) => {
                let x: usize = digit2 as usize;
                self.dt = self.v_reg[x];
            },

            // FX18 - ST = VX \\
            (0xF,_,1,8) => {
                let x: usize = digit2 as usize;
                self.st = self.v_reg[x];
            },

            // FX1E - I += VX \\
            (0xF,_,1,0xE) => {
                let x: usize = digit2 as usize;
                let vx: u16 = self.v_reg[x] as u16;
                self.i_reg = self.i_reg.wrapping_add(vx);
            },

            // FX29 - Set I to Font Addr \\
            (0xF,_,2,9) => {
                let x: usize = digit2 as usize;
                let c: u16 = self.v_reg[x] as u16;
                self.i_reg = c*5;
            },

            // FX33 - I = BCD of VX
            (0xF,_,3,3) => {
                let x: usize = digit2 as usize;
                let vx: u16 = self.v_reg[x] as u16; 
                // BCD - Binary Coded Decimal
                let hundreds: u8 = (vx/100) as u8;
                let tens: u8 = ((vx/10)%10) as u8;
                let ones: u8 = (vx%10) as u8;

                self.ram[self.i_reg as usize] = hundreds;
                self.ram[(self.i_reg + 1) as usize] = tens;
                self.ram[(self.i_reg + 2) as usize] = ones;
            },

            // FX55 - Store V0 - VX into I \\
            (0xF,_,5,5) => {
                let x: usize = digit2 as usize;
                let i: usize = self.i_reg as usize;
                for idx in 0..x{
                    self.ram[i+idx] = self.v_reg[idx];
                }
            },
            
            // FX65 - Load I into V0-VX
            (0xF,_,6,5) => {
                let x: usize = digit2 as usize;
                let i: usize = self.i_reg as usize;
                for idx in 0..x{
                    self.v_reg[idx] = self.ram[i+idx];
                }
            },

            (_,_,_,_) => unimplemented!("Error: Unimplimented opcode: {}", op),
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
                // Bada Bing Bada Boom
            }
            self.st -= 1;
        }
    }
}






