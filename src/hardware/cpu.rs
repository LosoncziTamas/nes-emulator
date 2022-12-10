#![allow(unused)]

use crate::nes_instructions::instruction_list::Codes;

fn load_accumulator(cpu: &mut CPU, op_param: u8) {
    cpu.program_counter += 1;
    cpu.register_a = op_param;

    if cpu.register_a == 0 {
        cpu.status = cpu.status | 0b0000_0010;
    } else {
        cpu.status = cpu.status & 0b1111_1101;
    }

    if cpu.register_a & 0b1000_000 != 0 {
        cpu.status = cpu.status | 0b1000_000;
    } else {
        cpu.status = cpu.status & 0b0111_111;
    }
}

fn make_code(u8op: u8) -> Codes {
    match u8op {
        0xA9 => Codes::Lda,
        0x00 => Codes::Brk,
        _ => panic!("Unknown code {}", u8op),
    }
}

pub struct CPU {
    pub register_a: u8,
    pub status: u8,
    pub program_counter: u16,

}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            status: 0,
            program_counter: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;
            let op = make_code(opcode);
            match op {
                Codes::Lda => {load_accumulator(self, opcode );},
                Codes::Brk => { return; },
                _ => todo!(),
            }
        }
    }
}



