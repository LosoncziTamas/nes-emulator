#![allow(unused)]

use crate::common::mem_trait::Mem;
use crate::ram::bus::Bus;
use crate::nes_instructions::instruction_list::Codes;
use crate::nes_instructions::addressing_mode::AddressingMode;

const MEM_START: u16 = 0x8000;


fn load_accumulator(cpu: &mut CPU) {
    cpu.program_counter += 1;
    cpu.register_a = cpu.read(cpu.program_counter);
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
    pub register_y: u8,
    memory: [u8; 0xFFFF],
    pub bus: Bus,
}

impl Mem for CPU {
    fn read(&self, addr: u16) -> u8 {
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, data: u8) {
        self.bus.write(addr, data)
    }

    fn read_u16(&self, loc: u16) -> u16 {
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
            register_y: 0,
            memory: [0; 0xFFFF],
            bus: Bus::new(),
        }
    }

    fn select_op_mode(&self, mode: &AddressingMode) -> u16 {
        let base = self.read_u16(self.program_counter);
        let pos = self.read(self.program_counter);
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => pos as u16,
            AddressingMode::Absolute => self.read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let addr = pos.wrapping_add(self.register_x) as u16;
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }
            AddressingMode::Absolute_X => {
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }
            AddressingMode::Indirect_X => {
                let ptr: u8 = (pos as u8).wrapping_add(self.register_x);
                let lo = self.read(ptr as u16);
                let hi = self.read(ptr.wrapping_add(1) as u16);
                (hi as u16) << 8 | (lo as u16)
            },
            AddressingMode::Indirect_Y => {
                let lo = self.read(pos as u16);
                let hi = self.read((pos as u8).wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            AddressingMode::NoneAddressing => {
                panic!("Unknown addressing mode");
            }
        }
    }

    pub fn load_and_run(&mut self, mut program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.write_u16(0xFFC, MEM_START);
    }

    pub fn reset(&mut self) {
        self.status = 0;
        self.register_a = 0;
        self.register_x = 0;


        self.program_counter = self.read_u16(0xFFFC);
    }

    pub fn run(&mut self) {
         loop {
            //let mut data: u8 = 0;
            let opcode = self.read(self.program_counter);
            self.program_counter += 1;
            let counter = self.program_counter as usize;
            // if (program.len() > counter)
            // {
            //     data = program[counter];
            // }
            let op = make_code(opcode);
            match op {
                Codes::Brk => { return; }
                Codes::Inx => { increment_x_register(self); }
                Codes::Lda => { load_accumulator(self ); }
                Codes::Tax => { transfer_accumulator_to_x(self); }
                Codes::Zpg => { continue; }
                _ => todo!()
            }
        }
    }

    pub fn interpret(&mut self, mut program: Vec<u8>) {
        self.program_counter = 0;
        todo!("remove from main")
       
    }
}



