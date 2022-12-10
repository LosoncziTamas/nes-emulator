use crate::hardware::ppu::NesPPU;

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
    pub screen_mirroring: Mirroring
}

const RAM_REG: u16 = 0x0000;
const RAM_MIRROR_ENDS: u16 = 0x1FFF;
const PPU_REG: u16 = 0x2000;
const PPU_MIRROR_ENDS: u16 = 0x3FFF;

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
    fn read(&self, addr: u16) {
        match addr {
            RAM_REG ..= RAM_MIRROR_ENDS => {
                let ram_mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[ram_mirror_down_addr as usize];
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Attempt to read from write-only PPU address {:x}", addr);
            }
            0x2007 => self.ppu.read_data(),
            0x2008 ..= PPU_MIRROR_ENDS => {
                let mirror_down_addr = addr & 0b00100000_00000111;
                self.mem_read(mirror_down_addr)
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
                self.mem_write(mirror_down_addr, data);
            }
            0x8000..=0xFFFF => panic!("Attempt to write to Cartridge ROM space: {:x}", addr),
            _ => {
                println!("[WRITE] Unregistered memory access at {}", addr);
                0
            }
        }
    }
}
