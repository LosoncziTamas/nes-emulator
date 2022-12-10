#![allow(unused)]

mod hardware;
mod nes_instructions;
mod test;

fn main() {
    println!("Hello, world!");
    let mut cpu = hardware::cpu::CPU::new();
    let mut program = vec![0xA9, 0x05, 0x00];
    cpu.interpret(program);
}
