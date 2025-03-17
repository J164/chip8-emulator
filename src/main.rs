use chip8::Chip8;

mod chip8;

fn main() {
    let mut processor = Chip8::new();
    processor.load(&[]);

    loop {
        processor.emulate_cycle();
    }
}
