#![allow(unused)]


#[macro_use]
extern crate lazy_static;

mod common;
mod hardware;
mod nes_instructions;
mod ram;
mod test;
mod cartridge;
mod platform;

use platform::sdl;

const USE_SDL: bool = true;

fn main() {
    if (USE_SDL) {
        sdl::sdl_entry();
    } else {
        println!("Iron world!");
        let mut cpu = hardware::cpu::CPU::new();
        let mut program = vec![0xE8, 0x05, 0x00];
        cpu.interpret(program);
    }
}
