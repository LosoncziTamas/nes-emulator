#![allow(unused)]

use crate::nes_instructions::instruction_list::Codes;

fn load_accumulator(cpu: &mut CPU) {
    todo!();
}

fn map_instructions_to_number() {
    todo!();
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

    pub fn interpret(&mut self, program: Vec<Codes>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                Codes::Lda => {load_accumulator(self);},
                _ => todo!(),
            }
        }
    }
}



