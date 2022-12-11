#![allow(unused)]

use crate::cartridge::rom::Rom;
use crate::common::mem_trait::Mem;
use crate::ram::bus::Bus;
use crate::nes_instructions::instruction_list::Codes;

fn load_accumulator(cpu: &mut CPU, data: u8) {
    cpu.program_counter += 1;
    cpu.register_a = data;
    update_zero_and_negative_flags(cpu, cpu.register_a);
}

fn transfer_accumulator_to_x(cpu: &mut CPU) {
    cpu.register_x = cpu.register_a;
    update_zero_and_negative_flags(cpu, cpu.register_x);
}

fn increment_x_register(cpu: &mut CPU) {
    cpu.register_x += 1;
    update_zero_and_negative_flags(cpu, cpu.register_x);
}

fn update_zero_and_negative_flags(cpu: &mut CPU, result: u8){
    if result == 0 {
        cpu.status = cpu.status | 0b0000_0010;
    } else {
        cpu.status = cpu.status & 0b1111_1101;
    }

    if result & 0b1000_000 != 0 {
        cpu.status = cpu.status | 0b1000_000;
    } else {
        cpu.status = cpu.status & 0b0111_1111;
    }
}

fn make_code(u8op: u8) -> Codes {
    match u8op {
        0x00 => Codes::Brk,
        0xE8 => Codes::Inx,
        0xA9 => Codes::Lda,
        0xAA => Codes::Tax,
        0x05 => Codes::Zpg,
        _ => panic!("Ran out of valid Codes!!!")
    }
}

pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,
    pub register_x: u8,
    memory: [u8; 0xFFFF],
    pub bus: Bus,
}

impl Mem for CPU {
    fn read(&mut self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data)
    }

    fn read_u16(&mut self, loc: u16) -> u16 {
        self.bus.read_u16(loc)
    }

    fn write_u16(&mut self, loc: u16, pck: u16) {
        self.bus.write_u16(loc, pck)
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
            register_x: 0,
            memory: [0; 0xFFFF],
            bus: Bus::new(Rom::new_dummy()),
        }
    }

    pub fn interpret(&mut self, mut program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let mut data: u8 = 0;
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;
            let counter = self.program_counter as usize;
            if (program.len() > counter)
            {
                data = program[counter];
            }

            let op = make_code(opcode);
            match op {
                Codes::Brk => { return; }
                Codes::Inx => { increment_x_register(self); }
                Codes::Lda => { load_accumulator(self, data ); }
                Codes::Tax => { transfer_accumulator_to_x(self); }
                Codes::Zpg => { continue; }
                _ => todo!()
            }
        }
    }
}



