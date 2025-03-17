use rand::random;

const REGISTER_COUNT: usize = 16;
const MEMORY_SIZE: usize = 4096;
const MAX_STACK_SIZE: usize = 12;
const KEYPAD_SIZE: usize = 16;
const INSTRUCTION_SIZE: usize = 2;
/// The logical height of the screen in pixels
const SCREEN_HEIGHT: usize = 32;

pub struct Chip8 {
    registers: [u8; REGISTER_COUNT],
    index_register: usize,
    program_counter: usize,
    stack_pointer: usize,
    memory: [u8; MEMORY_SIZE],
    stack: [usize; MAX_STACK_SIZE],
    sound_timer: u8,
    delay_timer: u8,
    keypad: [bool; KEYPAD_SIZE],
    screen: [u64; SCREEN_HEIGHT],
    draw_flag: bool
}

type Opcode = u16;

/// Allows the type to be parsed as a CHIP8 opcode
trait Chip8Opcode {
    /// Splits the opcode into hexadecimal digits
    fn nibbles(&self) -> (u8, u8, u8, u8);
    /// Gets the address
    fn nnn(&self) -> usize;
    /// Gets the 8-bit constant
    fn nn(&self) -> u8;
    /// Gets the 4-bit constant
    fn n(&self) -> u8;
    /// Gets the first register address
    fn x(&self) -> usize;
    /// Gets the second register address
    fn y(&self) -> usize;
}

impl Chip8Opcode for Opcode {
    fn nibbles(&self) -> (u8, u8, u8, u8) {
        (
            ((self & 0xF000) >> 12) as u8,
            ((self & 0x0F00) >> 8) as u8,
            ((self & 0x00F0) >> 4) as u8,
            (self & 0x000F) as u8,
        )
    }

    fn nnn(&self) -> usize {
        (self & 0x0FFF) as usize
    }

    fn nn(&self) -> u8 {
        (self & 0x00FF) as u8
    }

    fn n(&self) -> u8 {
        (self & 0x000F) as u8
    }

    fn x(&self) -> usize {
        ((self & 0x0F00) >> 8) as usize
    }

    fn y(&self) -> usize {
        ((self & 0x00F0) >> 4) as usize
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
            screen: [0; SCREEN_HEIGHT],
            draw_flag: false
        }
    }

    /// Loads bytes of data into memory starting from address 0x200
    pub fn load(&mut self, data: &[u8]) {
        let start_address = 0x200;
        let max_length = MEMORY_SIZE - start_address;
        for (i, &byte) in data.iter().take(max_length).enumerate() {
            self.memory[start_address + i] = byte;
        }
    }

    /// Emulates one tick of the processor
    pub fn emulate_cycle(&mut self) {
        let opcode = self.fetch_opcode();

        self.program_counter = match opcode.nibbles() {
            (0x00, 0x00, 0x0E, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0E, 0x0E) => self.op_00ee(),
            (0x01, _, _, _) => self.op_1nnn(opcode.nnn()),
            (0x02, _, _, _) => self.op_2nnn(opcode.nnn()),
            (0x03, _, _, _) => self.op_3xnn(opcode.x(), opcode.nn()),
            (0x04, _, _, _) => self.op_4xnn(opcode.x(), opcode.nn()),
            (0x05, _, _, 0x00) => self.op_5xy0(opcode.x(), opcode.y()),
            (0x06, _, _, _) => self.op_6xnn(opcode.x(), opcode.nn()),
            (0x07, _, _, _) => self.op_7xnn(opcode.x(), opcode.nn()),
            (0x08, _, _, 0x00) => self.op_8xy0(opcode.x(), opcode.y()),
            (0x08, _, _, 0x01) => self.op_8xy1(opcode.x(), opcode.y()),
            (0x08, _, _, 0x02) => self.op_8xy2(opcode.x(), opcode.y()),
            (0x08, _, _, 0x03) => self.op_8xy3(opcode.x(), opcode.y()),
            (0x08, _, _, 0x04) => self.op_8xy4(opcode.x(), opcode.y()),
            (0x08, _, _, 0x05) => self.op_8xy5(opcode.x(), opcode.y()),
            (0x08, _, _, 0x06) => self.op_8xy6(opcode.x(), opcode.y()),
            (0x08, _, _, 0x07) => self.op_8xy7(opcode.x(), opcode.y()),
            (0x08, _, _, 0x0E) => self.op_8xye(opcode.x(), opcode.y()),
            (0x09, _, _, 0x00) => self.op_9xy0(opcode.x(), opcode.y()),
            (0x0A, _, _, _) => self.op_annn(opcode.nnn()),
            (0x0B, _, _, _) => self.op_bnnn(opcode.nnn()),
            (0x0C, _, _, _) => self.op_cxnn(opcode.x(),opcode.nn()),
            (0x0D, _, _, _) => self.op_dxyn(opcode.x(), opcode.y(), opcode.n()),
            _ => unimplemented!(),
        };

        todo!();
    }

    /// Fetches the 16 bit opcode pointed to by the program counter
    fn fetch_opcode(&self) -> Opcode {
        ((self.memory[self.program_counter] as u16) << 8)
            | (self.memory[self.program_counter + 1] as u16)
    }

    /// Opcode: 00E0
    /// Clears the screen
    fn op_00e0(&mut self) -> usize {
        self.screen = [0; SCREEN_HEIGHT];
        self.draw_flag = true;
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 00EE
    /// Returns from the current subroutine
    fn op_00ee(&mut self) -> usize {
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer] + INSTRUCTION_SIZE
    }

    /// Opcode: 1NNN
    /// Sets the program counter to address `nnn`
    fn op_1nnn(&self, nnn: usize) -> usize {
        nnn
    }

    /// Opcode: 2NNN
    /// Calls the subroutine at address `nnn`
    fn op_2nnn(&mut self, nnn: usize) -> usize {
        self.stack[self.stack_pointer] = self.program_counter;
        self.stack_pointer += 1;
        nnn
    }

    /// Opcode: 3XNN
    /// Skips the following instruction if `registers[x] = nn`
    fn op_3xnn(&mut self, x: usize, nn: u8) -> usize {
        if self.registers[x] == nn {
            self.program_counter + INSTRUCTION_SIZE * 2
        } else {
            self.program_counter + INSTRUCTION_SIZE
        }
    }

    /// Opcode: 4XNN
    /// Skips the following instruction if `registers[x] != nn`
    fn op_4xnn(&mut self, x: usize, nn: u8) -> usize {
        if self.registers[x] != nn {
            self.program_counter + INSTRUCTION_SIZE * 2
        } else {
            self.program_counter + INSTRUCTION_SIZE
        }
    }

    /// Opcode: 5XY0
    /// Skips the following instruction if `registers[x] = registers[y]`
    fn op_5xy0(&mut self, x: usize, y: usize) -> usize {
        if self.registers[x] == self.registers[y] {
            self.program_counter + INSTRUCTION_SIZE * 2
        } else {
            self.program_counter + INSTRUCTION_SIZE
        }
    }

    /// Opcode: 6XNN
    /// Stores `nn` in `registers[x]`
    fn op_6xnn(&mut self, x: usize, nn: u8) -> usize {
        self.registers[x] = nn;
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 7XNN
    /// Adds `nn` to `registers[x]`
    fn op_7xnn(&mut self, x: usize, nn: u8) -> usize {
        self.registers[x] += nn;
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 8XY0
    /// Stores `registers[y]` in `registers[x]`
    fn op_8xy0(&mut self, x: usize, y: usize) -> usize {
        self.registers[x] = self.registers[y];
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 8XY1
    /// ORs `registers[y]` with `registers[x]` and stores in `registers[x]`
    fn op_8xy1(&mut self, x: usize, y: usize) -> usize {
        self.registers[x] |= self.registers[y];
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 8XY2
    /// ANDs `registers[y]` with `registers[x]` and stores in `registers[x]`
    fn op_8xy2(&mut self, x: usize, y: usize) -> usize {
        self.registers[x] &= self.registers[y];
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 8XY3
    /// XORs `registers[y]` with `registers[x]` and stores in `registers[x]`
    fn op_8xy3(&mut self, x: usize, y: usize) -> usize {
        self.registers[x] ^= self.registers[y];
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 8XY4
    /// Adds `registers[y]` to `registers[x]` and stores in `registers[x]`
    /// Sets `registers[0xF]` to `0x01` if overflow occurs and `0x00` otherwise
    fn op_8xy4(&mut self, x: usize, y: usize) -> usize {
        let (result, overflow) = self.registers[x].overflowing_add(self.registers[y]);
        self.registers[x] = result;
        self.registers[0xF] = if overflow { 0x01 } else { 0x00 };
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 8XY5
    /// Subtracts `registers[y]` from `registers[x]` and stores in `registers[x]`
    /// Sets `registers[0xF]` to `0x01` if overflow occurs and `0x00` otherwise
    fn op_8xy5(&mut self, x: usize, y: usize) -> usize {
        let (result, overflow) = self.registers[x].overflowing_sub(self.registers[y]);
        self.registers[x] = result;
        self.registers[0xF] = if overflow { 0x01 } else { 0x00 };
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 8XY6
    /// Stores `registers[y]` shifted right one bit in `registers[x]`
    /// Sets `registers[0xF]` to the least significant bit of `registers[y]`
    fn op_8xy6(&mut self, x: usize, y: usize) -> usize {
        self.registers[x] = self.registers[y] >> 1;
        self.registers[0xF] = self.registers[y] & 0x01;
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 8XY7
    /// Subtracts `registers[x]` from `registers[y]` and stores in `registers[x]`
    /// Sets `registers[0xF]` to `0x01` if overflow occurs and `0x00` otherwise
    fn op_8xy7(&mut self, x: usize, y: usize) -> usize {
        let (result, overflow) = self.registers[y].overflowing_sub(self.registers[x]);
        self.registers[x] = result;
        self.registers[0xF] = if overflow { 0x01 } else { 0x00 };
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 8XYE
    /// Stores `registers[y]` shifted left one bit in `registers[x]`
    /// Sets `registers[0xF]` to the most significant bit of `registers[y]`
    fn op_8xye(&mut self, x: usize, y: usize) -> usize {
        self.registers[x] = self.registers[y] << 1;
        self.registers[0xF] = self.registers[y] >> 7;
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: 9XY0
    /// Skips the following instruction if `registers[x] != registers[y]`
    fn op_9xy0(&mut self, x: usize, y: usize) -> usize {
        if self.registers[x] != self.registers[y] {
            self.program_counter + INSTRUCTION_SIZE * 2
        } else {
            self.program_counter + INSTRUCTION_SIZE
        }
    }

    /// Opcode: ANNN
    /// Store memory address `nnn` in `index_register`
    fn op_annn(&mut self, nnn: usize) -> usize {
        self.index_register = nnn;
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: BNNN
    /// Sets the program counter to `nnn + registers[0x0]`
    fn op_bnnn(&self, nnn: usize) -> usize {
        nnn + (self.registers[0x0] as usize)
    }

    /// Opcode: CXNN
    /// Sets `registers[x]` to a random number with a mask of `nn`
    fn op_cxnn(&mut self, x: usize, nn: u8) -> usize {
        self.registers[x] = random::<u8>() & nn;
        self.program_counter + INSTRUCTION_SIZE
    }

    /// Opcode: DXYN
    /// Draws a sprite from memory starting at the address `index_register` at position `registers[x]`, `registers[y]`
    /// The sprite has a width of 8 pixels and a height of `n` pixels
    /// Sets `registers[0xF]` to `0x01` if any previously set pixels are unset and `0x00` otherwise
    fn op_dxyn(&mut self, x: usize, y: usize, n: u8) -> usize {
        self.registers[0xF] = 0x00;
        
        let left = self.registers[x] as usize;
        let top = self.registers[y] as usize;

        for row_idx in 0..(n as usize) {
            let row = (self.memory[self.index_register + row_idx] as u64) << ((64 - 8) - left);
            let collision_mask = self.screen[top + row_idx] | row;
            self.screen[top + row_idx] ^= row;

            if self.screen[top + row_idx] != collision_mask {
                self.registers[0xF] = 0x01;
            }
        }

        self.program_counter + INSTRUCTION_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let mut chip8 = Chip8::new();
        let buffer = [1, 2, 3, 4, 5];

        chip8.load(&buffer);
        assert_eq!(chip8.memory[0x200], 1);
        assert_eq!(chip8.memory[0x201], 2);
        assert_eq!(chip8.memory[0x202], 3);
        assert_eq!(chip8.memory[0x203], 4);
        assert_eq!(chip8.memory[0x204], 5);
    }

    #[test]
    fn test_op_00e0() {
        let mut chip8 = Chip8::new();
        chip8.screen = [0xFFFFFFFFFFFFFFFF; SCREEN_HEIGHT];
        chip8.program_counter = 0x200;
        
        let result = chip8.op_00e0();
        assert_eq!(chip8.screen, [0; SCREEN_HEIGHT]);
        assert!(chip8.draw_flag);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_00ee() {
        let mut chip8 = Chip8::new();
        chip8.stack_pointer = 0x01;
        chip8.stack[0] = 0x300;
        
        let result = chip8.op_00ee();
        assert_eq!(result, 0x300 + INSTRUCTION_SIZE);
        assert_eq!(chip8.stack_pointer, 0);
    }

    #[test]
    fn test_op_1nnn() {
        let chip8 = Chip8::new();
        assert_eq!(chip8.op_1nnn(0x400), 0x400);
    }

    #[test]
    fn test_op_2nnn() {
        let mut chip8 = Chip8::new();
        chip8.program_counter = 0x200;
        chip8.stack_pointer = 0x00;

        let result = chip8.op_2nnn(0x600);
        assert_eq!(result, 0x600);
        assert_eq!(chip8.stack[0], 0x200);
        assert_eq!(chip8.stack_pointer, 1);
    }

    #[test]
    fn test_op_3xnn() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x42;
        chip8.program_counter = 0x200;

        let result = chip8.op_3xnn(0x1, 0x42);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE * 2);

        let result = chip8.op_3xnn(0x1, 0x43);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_4xnn() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x42;
        chip8.program_counter = 0x200;

        let result = chip8.op_4xnn(0x1, 0x43);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE * 2);

        let result = chip8.op_4xnn(0x1, 0x42);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_5xy0() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x50;
        chip8.registers[0x2] = 0x50;
        chip8.registers[0x3] = 0x51;
        chip8.program_counter = 0x200;

        let result = chip8.op_5xy0(0x1, 0x2);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE * 2);

        let result = chip8.op_5xy0(0x1, 0x3);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_6xnn() {
        let mut chip8 = Chip8::new();
        chip8.program_counter = 0x200;

        let result = chip8.op_6xnn(0x1, 0x99);
        assert_eq!(chip8.registers[0x1], 0x99);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_7xnn() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x05;
        chip8.program_counter = 0x200;

        let result = chip8.op_7xnn(0x1, 0x10);
        assert_eq!(chip8.registers[0x1], 0x15);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_8xy0() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x05;
        chip8.registers[0x2] = 0x10;
        chip8.program_counter = 0x200;

        let result = chip8.op_8xy0(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0x10);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_8xy1() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x07;
        chip8.registers[0x2] = 0x0A;
        chip8.program_counter = 0x200;

        let result = chip8.op_8xy1(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0x0F);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_8xy2() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x07;
        chip8.registers[0x2] = 0x0A;
        chip8.program_counter = 0x200;

        let result = chip8.op_8xy2(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0x02);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_8xy3() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x07;
        chip8.registers[0x2] = 0x0A;
        chip8.program_counter = 0x200;

        let result = chip8.op_8xy3(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0x0D);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_8xy4() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x07;
        chip8.registers[0x2] = 0xFF;
        chip8.registers[0x3] = 0x0A;
        chip8.program_counter = 0x200;

        let result = chip8.op_8xy4(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0x06);
        assert_eq!(chip8.registers[0xF], 0x01);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);

        let result = chip8.op_8xy4(0x1, 0x3);
        assert_eq!(chip8.registers[0x1], 0x10);
        assert_eq!(chip8.registers[0xF], 0x00);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_8xy5() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x07;
        chip8.registers[0x2] = 0xFF;
        chip8.registers[0x3] = 0x03;
        chip8.program_counter = 0x200;

        let result = chip8.op_8xy5(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0x08);
        assert_eq!(chip8.registers[0xF], 0x01);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);

        let result = chip8.op_8xy5(0x1, 0x3);
        assert_eq!(chip8.registers[0x1], 0x05);
        assert_eq!(chip8.registers[0xF], 0x00);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_8xy6() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x07;
        chip8.registers[0x2] = 0xFF;
        chip8.registers[0x3] = 0x06;
        chip8.program_counter = 0x200;

        let result = chip8.op_8xy6(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0x7F);
        assert_eq!(chip8.registers[0xF], 0x01);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);

        let result = chip8.op_8xy6(0x1, 0x3);
        assert_eq!(chip8.registers[0x1], 0x03);
        assert_eq!(chip8.registers[0xF], 0x00);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_8xy7() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x0F;
        chip8.registers[0x2] = 0x07;
        chip8.registers[0x3] = 0x08;
        chip8.program_counter = 0x200;

        let result = chip8.op_8xy7(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0xF8);
        assert_eq!(chip8.registers[0xF], 0x01);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);

        let result = chip8.op_8xy7(0x2, 0x3);
        assert_eq!(chip8.registers[0x2], 0x01);
        assert_eq!(chip8.registers[0xF], 0x00);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_8xye() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x07;
        chip8.registers[0x2] = 0xFF;
        chip8.registers[0x3] = 0x06;
        chip8.program_counter = 0x200;

        let result = chip8.op_8xye(0x1, 0x2);
        assert_eq!(chip8.registers[0x1], 0xFE);
        assert_eq!(chip8.registers[0xF], 0x01);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);

        let result = chip8.op_8xye(0x1, 0x3);
        assert_eq!(chip8.registers[0x1], 0x0C);
        assert_eq!(chip8.registers[0xF], 0x00);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_9xy0() {
        let mut chip8 = Chip8::new();
        chip8.registers[0x1] = 0x50;
        chip8.registers[0x2] = 0x50;
        chip8.registers[0x3] = 0x51;
        chip8.program_counter = 0x200;

        let result = chip8.op_9xy0(0x1, 0x2);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);

        let result = chip8.op_9xy0(0x1, 0x3);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE * 2);
    }

    #[test]
    fn test_op_annn() {
        let mut chip8 = Chip8::new();
        chip8.program_counter = 0x200;

        let result = chip8.op_annn( 0x99A);
        assert_eq!(chip8.index_register, 0x99A);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_bnnn() {
        let mut chip8 = Chip8::new();
        chip8.program_counter = 0x200;
        chip8.registers[0x0] = 0x04;

        let result = chip8.op_bnnn( 0x203);
        assert_eq!(result, 0x207);
    }

    #[test]
    fn test_op_cxnn() {
        let mut chip8 = Chip8::new();
        chip8.program_counter = 0x200;

        let result = chip8.op_cxnn( 0x0, 0xF0);
        assert_eq!(chip8.registers[0x0] & !0xF0, 0);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }

    #[test]
    fn test_op_dxyn() {
        let mut chip8 = Chip8::new();
        chip8.program_counter = 0x200;
        chip8.registers[0x0] = 0x05;
        chip8.registers[0x1] = 0x01;
        chip8.index_register = 0x300;
        chip8.memory[0x300] = 0b11111111;
        chip8.memory[0x301] = 0b10000001;
        chip8.memory[0x302] = 0b10000001;
        chip8.memory[0x303] = 0b11111111;

        let result = chip8.op_dxyn(0x0, 0x01, 0x4);
        assert_eq!(chip8.screen, [
            0x0000000000000000,
            0x07F8000000000000,
            0x0408000000000000,
            0x0408000000000000,
            0x07F8000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
        ]);
        assert_eq!(chip8.registers[0xF], 0x00);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);

        chip8.registers[0x3] = 0x02;
        let result = chip8.op_dxyn(0x0, 0x3, 0x1);
        assert_eq!(chip8.screen, [
            0x0000000000000000,
            0x07F8000000000000,
            0x03F0000000000000,
            0x0408000000000000,
            0x07F8000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
            0x0000000000000000,
        ]);
        assert_eq!(chip8.registers[0xF], 0x01);
        assert_eq!(result, 0x200 + INSTRUCTION_SIZE);
    }
}
