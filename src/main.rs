#![allow(unused)]

mod hardware;
mod nes_instructions;

fn main() {
    println!("Hello, world!");
    let mut cpu = hardware::cpu::CPU::new();
    cpu.interpret(vec![0xA9, 0x05, 0x00]);
}
