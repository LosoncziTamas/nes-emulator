const MIRROR_DOWN_ADDRESS_START: u16 = 0x3fff;
const MIRROR_MASK: u16 = 0b0011_1111_1111_1111;

pub struct AddrRegister {
    high_byte: u8,
    low_byte: u8,
    high_byte_latch: bool,
}

impl AddrRegister {
    pub fn new() -> Self {
        AddrRegister {
            high_byte: 0,
            low_byte: 0,
            high_byte_latch: true,
        }
    }

    fn set(&mut self, data: u16) {
        self.high_byte = (data >> 8) as u8;
        self.low_byte = (data & 0xff) as u8;
    }

    pub fn get(&self) -> u16 {
        ((self.high_byte as u16) << 8) | (self.low_byte as u16)
    }

    pub fn write(&mut self, data: u8) {
        if self.high_byte_latch {
            self.high_byte = data;
        } else {
            self.low_byte = data;
        }
        self.handle_mirroring_down();
        self.high_byte_latch = !self.high_byte_latch;
    }

    pub fn increment(&mut self, value: u8) {
        let old_low = self.low_byte;
        self.low_byte = self.low_byte.wrapping_add(value);
        let is_wrapping = old_low > self.low_byte;
        if is_wrapping {
            self.high_byte = self.high_byte.wrapping_add(1);
        }
        self.handle_mirroring_down()
    }

    fn handle_mirroring_down(&mut self) {
        if self.get() > MIRROR_DOWN_ADDRESS_START {
            self.set(self.get() & MIRROR_MASK);
        }
    }

    pub fn reset_latch(&mut self) {
        self.high_byte_latch = true;
    }
}
