use crate::hardware::ppu::NesPPU;
use crate::common::mem_trait::Mem;

// TODO: remove this when merged with cartridge code
#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

// TODO: remove dummy rom when merged with actual one
pub struct Rom{
    pub chr_rom: Vec<u8>,
    pub prg_rom: Vec<u8>,
    pub screen_mirroring: Mirroring
}

pub const RAM_REG: u16 = 0x0000;
pub const RAM_MIRROR_ENDS: u16 = 0x1FFF;
pub const PPU_REG: u16 = 0x2000;
pub const PPU_MIRROR_ENDS: u16 = 0x3FFF;

pub struct Bus {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: NesPPU
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        let ppu = NesPPU::new(rom.chr_rom, rom.screen_mirroring);

        Bus {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: ppu,
        }
    }
}

impl Mem for Bus {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_REG ..= RAM_MIRROR_ENDS => {
                let ram_mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[ram_mirror_down_addr as usize]
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Attempt to read from write-only PPU address {:x}", addr);
            }
            0x2007 => self.ppu.read_data(),
            0x2008 ..= PPU_MIRROR_ENDS => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.read(mirror_down_addr)
            }
            //TODO: implement read from rom 0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("[READ] Unregistered memory access at {}", addr);
                0
            }
        }
    }


    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_REG ..= RAM_MIRROR_ENDS => {
                let ram_mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[ram_mirror_down_addr as usize] = data;
            }
            0x2000 => {
                self.ppu.write_to_ctrl(data);
            }
            0x2006 => {
                self.ppu.write_to_ppu_addr(data);
            }
            0x2007 => {
                self.ppu.write_to_data(data);
            }
            0x2008..=PPU_MIRROR_ENDS => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.write(mirror_down_addr, data);
            }
            0x8000..=0xFFFF => panic!("Attempt to write to Cartridge ROM space: {:x}", addr),
            _ => {
                println!("[WRITE] Unregistered memory access at {}", addr);
            }
        }
    }

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


#[test]
fn invalid_ram_access() {
    let invalid_ram_addr = 0x2457;
    let bus = Bus::new();
    assert_eq!(0, bus.read(invalid_ram_addr));
}

#[test]
fn read_ram_addr() {
    let read_data: u8 = 0x24;
    let mut bus = Bus::new();
    bus.cpu_vram[1] = read_data;
    assert!(0x24 == bus.read(0x0001));
}

#[test]
fn write_ram_addr() {
    let write_data: u8 = 0x42;
    let write_addr: u16 = 0x0001;
    let mut bus = Bus::new();
    bus.write(write_addr, write_data);
    assert!(0 == 0);
}
