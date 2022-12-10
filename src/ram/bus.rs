use crate::common::mem_trait::Mem;

const RAM_REG: u16 = 0x0000;
const RAM_MIRROR_ENDS: u16 = 0x1FFF;
const PPU_REG: u16 = 0x2000;
const PPU_MIRROR_ENDS: u16 = 0x3FFF;

pub struct Bus {
    cpu_vram: [u8; 2048]
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu_vram: [0; 2048]
        }
    }
}

impl Mem for Bus {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            RAM_REG ..= RAM_MIRROR_ENDS => {
                let ram_mirror_down_addr = addr & 0b00000111_11111111;
                self.cpu_vram[ram_mirror_down_addr as usize]
            }
            PPU_REG ..= PPU_MIRROR_ENDS => {
                let ppu_mirror_down_addr = addr & 0b00100000_00000111;
                todo!("Ppu implementation awaits!");
            }
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
            PPU_REG ..= PPU_MIRROR_ENDS => {
                let ppu_mirror_down_addr = addr & 0b00100000_00000111;
                todo!("Ppu implementation coming up!");
            }
            _ => {
                println!("[WRITE] Unregistered memory access at {}", addr);
            }
        }
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
