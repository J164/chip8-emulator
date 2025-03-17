const REGISTER_COUNT: usize = 16;
const MEMORY_SIZE: usize = 4096;
const MAX_STACK_SIZE: usize = 12;
const KEYPAD_SIZE: usize = 16;

pub struct Chip8 {
    registers: [u8; REGISTER_COUNT],
    index_register: usize,
    program_counter: usize,
    stack_pointer: usize,
    memory: [u8; MEMORY_SIZE],
    stack: [u16; MAX_STACK_SIZE],
    sound_timer: u8,
    delay_timer: u8,
    keypad: [bool; KEYPAD_SIZE],
}

type OpcodeDigits = (u8, u8, u8, u8);

trait Opcode {
    fn new(opcode: u16) -> Self;
    fn nnn(&self) -> usize;
    fn nn(&self) -> u8;
    fn n(&self) -> u8;
    fn x(&self) -> usize;
    fn y(&self) -> usize;
}

impl Opcode for OpcodeDigits {
    fn new(opcode: u16) -> Self {
        (
            (opcode & 0xF000) as u8,
            (opcode & 0x0F00) as u8,
            (opcode & 0x00F0) as u8,
            (opcode & 0x000F) as u8,
        )
    }

    fn nnn(&self) -> usize {
        ((self.1 as usize) << 8) | ((self.2 as usize) << 4) | (self.3 as usize)
    }

    fn nn(&self) -> u8 {
        (self.2 << 4) | self.3
    }

    fn n(&self) -> u8 {
        self.3
    }

    fn x(&self) -> usize {
        self.3 as usize
    }

    fn y(&self) -> usize {
        self.2 as usize
    }
}

impl Chip8 {
    pub fn new() -> Self {
        // TODO: load font into memory

        Chip8 {
            registers: [0; REGISTER_COUNT],
            index_register: 0,
            program_counter: 0,
            stack_pointer: 0,
            memory: [0; MEMORY_SIZE],
            stack: [0; MAX_STACK_SIZE],
            sound_timer: 0,
            delay_timer: 0,
            keypad: [false; KEYPAD_SIZE],
        }
    }

    /// Loads bytes of data into memory starting from address 0x200
    pub fn load(&mut self, data: &[u8]) {
        todo!();
    }

    /// Emulates one tick of the processor
    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch_opcode();
        self.execute(opcode);

        todo!();
    }

    /// Fetches the 16 bit opcode pointed to by the program counter
    fn fetch_opcode(&self) -> u16 {
        ((self.memory[self.program_counter] as u16) << 8)
            | (self.memory[self.program_counter + 1] as u16)
    }

    /// Decodes and executes instruction corresponding to the opcode
    fn execute(&mut self, opcode: u16) {
        let opcode_digits = OpcodeDigits::new(opcode);

        match opcode_digits {
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
            (0x00, _, _, _) => self.op_0nnn(opcode_digits.nnn()),
            (0x01, _, _, _) => self.op_1nnn(opcode_digits.nnn()),
            (0x02, _, _, _) => self.op_2nnn(opcode_digits.nnn()),
            (0x03, _, _, _) => self.op_3xnn(opcode_digits.x(), opcode_digits.nn()),
            (0x04, _, _, _) => self.op_4xnn(opcode_digits.x(), opcode_digits.nn()),
            (0x05, _, _, 0x00) => self.op_5xy0(opcode_digits.x(), opcode_digits.y()),
            (0x06, _, _, _) => self.op_6xnn(opcode_digits.x(), opcode_digits.nn()),
            (0x07, _, _, _) => self.op_7xnn(opcode_digits.x(), opcode_digits.nn()),
            _ => unimplemented!(),
        }
    }

    /// Opcode: 0NNN
    /// Calls the subroutine at address `nnn`
    fn op_0nnn(&mut self, nnn: usize) {
        todo!();
    }

    /// Opcode: 00E0
    /// Clears the screen
    fn op_00e0(&mut self) {
        todo!();
    }

    /// Opcode: 00EE
    /// Returns from the current subroutine
    fn op_00ee(&mut self) {
        todo!();
    }

    /// Opcode: 1NNN
    /// Sets the program counter to address `nnn`
    fn op_1nnn(&mut self, nnn: usize) {
        todo!();
    }

    /// Opcode: 2NNN
    /// Calls the subroutine at address `nnn`
    fn op_2nnn(&mut self, nnn: usize) {
        todo!();
    }

    /// Opcode: 3XNN
    /// Skips the following instruction if `registers[x] = nn`
    fn op_3xnn(&mut self, x: usize, nn: u8) {
        todo!();
    }

    /// Opcode: 4XNN
    /// Skips the following instruction if `registers[x] != nn`
    fn op_4xnn(&mut self, x: usize, nn: u8) {
        todo!();
    }

    /// Opcode: 5XY0
    /// Skips the following instruction if `registers[x] = registers[y]`
    fn op_5xy0(&mut self, x: usize, y: usize) {
        todo!();
    }

    /// Opcode: 6XNN
    /// Stores `nn` in register `x`
    fn op_6xnn(&mut self, x: usize, nn: u8) {
        self.registers[x] = nn;
    }

    /// Opcode: 7XNN
    /// Adds `nn` to register `x`
    fn op_7xnn(&mut self, x: usize, nn: u8) {
        self.registers[x] += nn;
    }
}
