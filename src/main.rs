#![allow(unused)]

mod hardware;
mod nes_instructions;

fn main() {
    println!("Hello, world!");
    let mut cpu = hardware::cpu::CPU::new();
    let mut prog = Vec::new();
    prog.push(nes_instructions::instruction_list::Codes::Lda);
    cpu.interpret(prog);
}
