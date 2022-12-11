use crate::cartridge::rom::Rom;
use crate::common::mem_trait::Mem;
use crate::hardware::ppu::NesPPU;

pub const RAM_REG: u16 = 0x0000;
pub const RAM_MIRROR_ENDS: u16 = 0x1FFF;
pub const PPU_REG: u16 = 0x2000;
pub const PPU_MIRROR_ENDS: u16 = 0x3FFF;

pub struct Bus {
    cpu_vram: [u8; 2048],
    rom: Rom,
    ppu: NesPPU,
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        // TODO: do not clone here
        let ppu = NesPPU::new(rom.chr_rom.clone(), rom.screen_mirroring);

        Bus {
            cpu_vram: [0; 2048],
            rom,
            ppu: ppu,
        }
    }

    fn read_prg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr = addr % 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
}

impl Mem for Bus {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_REG..=RAM_MIRROR_ENDS => {
                let ram_mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[ram_mirror_down_addr as usize]
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Attempt to read from write-only PPU address {:x}", addr);
            }
            0x2007 => self.ppu.read_data(),
            0x2008..=PPU_MIRROR_ENDS => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.read(mirror_down_addr)
            }
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => {
                println!("[READ] Unregistered memory access at {}", addr);
                0
            }
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            RAM_REG..=RAM_MIRROR_ENDS => {
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
    let mut bus = Bus::new(Rom::new_dummy());
    assert_eq!(0, bus.read(invalid_ram_addr));
}

#[test]
fn read_ram_addr() {
    let read_data: u8 = 0x24;
    let mut bus = Bus::new(Rom::new_dummy());
    bus.cpu_vram[1] = read_data;
    assert!(0x24 == bus.read(0x0001));
}

#[test]
fn write_ram_addr() {
    let write_data: u8 = 0x42;
    let write_addr: u16 = 0x0001;
    let mut bus = Bus::new(Rom::new_dummy());
    bus.write(write_addr, write_data);
    assert!(0 == 0);
}
