#![allow(unused)]

#[macro_use]
extern crate lazy_static;

mod cartridge;
mod common;
mod hardware;
mod nes_instructions;
mod platform;
mod ram;
mod test;

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
