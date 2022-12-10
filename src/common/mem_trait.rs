pub trait Mem {
    fn read(&mut self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, data: u8);

    fn read_u16(&mut self, loc: u16) -> u16 {
        let lo = self.read(loc) as u16;
        let hi = self.read(loc + 1) as u16;
        (hi << 8) | (lo as u16)
    }

    fn write_u16(&mut self, loc: u16, pck: u16) {
        let hi = (pck >> 8) as u8;
        let lo = (pck & 0xFF) as u8;
        self.write(loc, lo);
        self.write(loc + 1, hi);
    }
}
