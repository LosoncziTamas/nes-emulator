#![allow(unused)]

mod cpu;
use cpu::{CPU};
//use cpu::CPU;

fn main() {
    println!("Hello, world!");
    let mut cpu = CPU::new();
    let mut prog = Vec::new();
    prog.push(5);
    cpu.interpret(prog);
}
