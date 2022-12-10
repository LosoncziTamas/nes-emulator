#![allow(unused)]


mod cpu;
//use cpu::{CPU};
mod instru;
//use instru::{Instructions};

fn main() {
    println!("Hello, world!");
    let mut cpu = cpu::CPU::new();
    let mut prog = Vec::new();
    prog.push(instru::Instructions::Lda);
    cpu.interpret(prog);
}
