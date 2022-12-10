#![allow(unused)]

mod common;
mod hardware;
mod nes_instructions;
mod ram;
mod test;
mod cartridge;

fn main() {
    println!("Iron world!");
    let mut cpu = hardware::cpu::CPU::new();
    let mut program = vec![0xE8, 0x05, 0x00];
    cpu.interpret(program);
}
