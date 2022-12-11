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

    pub fn update(&mut self, data: u8) {
        if self.high_byte_latch {
            self.high_byte = data;
        } else {
            self.low_byte = data;
        }

        if self.get() > MIRROR_DOWN_ADDRESS_START {
            self.set(self.get() & MIRROR_MASK);
        }
        self.high_byte_latch = !self.high_byte_latch;
    }

    pub fn increment(&mut self, value: u8) {
        let low = self.low_byte;
        self.low_byte = self.low_byte.wrapping_add(value);
        if low > self.low_byte {
            self.high_byte = self.high_byte.wrapping_add(1);
        }
        if self.get() > MIRROR_DOWN_ADDRESS_START {
            self.set(self.get() & MIRROR_MASK);
        }
    }

    pub fn reset_latch(&mut self) {
        self.high_byte_latch = true;
    }

    pub fn get(&self) -> u16 {
        ((self.high_byte as u16) << 8) | (self.low_byte as u16)
    }
}
