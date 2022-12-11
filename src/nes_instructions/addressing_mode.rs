use std::collections::HashMap;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
}

pub fn count_address_bytes(address_mo: &AddressingMode) -> i8 {
    match address_mo {
        AddressingMode::Immediate | AddressingMode::ZeroPage | AddressingMode::ZeroPage_X | AddressingMode::ZeroPage_Y => 2,
        AddressingMode::Absolute | AddressingMode::Absolute_X | AddressingMode::Absolute_Y | AddressingMode::Indirect_X | AddressingMode::Indirect_Y => 3,
        AddressingMode::NoneAddressing => -2,
        _ => panic!("Can't find Addressing mode to validate")
    }
}


