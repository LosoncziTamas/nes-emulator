use super::ppu_addr_register::AddrRegister;
use super::ppu_cntrl_register::ControlRegister;

// TODO: remove this when merged with cartridge code
#[derive(Debug, PartialEq)]
pub enum Mirroring {
    Vertical,
    Horizontal,
    FourScreen,
}

const VRAM_MAX_SIZE: usize = 2048;
const OAM_DATA_MAX_SIZE: usize = 256;
const PALETTE_TABLE_MAX_SIZE: usize = 32;

pub struct NesPPU {
    /// visuals of a game stored on a cartridge
    pub chr_rom: Vec<u8>,
    /// internal memory to keep palette tables used by a screen
    pub palette_table: [u8; PALETTE_TABLE_MAX_SIZE],
    /// 2 KiB banks of space to hold background information
    pub vram: [u8; VRAM_MAX_SIZE],
    ///  internal memory to keep state of sprites
    pub oam_data: [u8; OAM_DATA_MAX_SIZE],
    /// address register (0x2006) providing access to the memory map available for PPU
    pub addr: AddrRegister,
    /// control register (0x2000) instructs PPU on general logic flow
    pub ctrl: ControlRegister,
    /// internal buffer filled during the previous load operation
    pub internal_data_buf: u8,

    pub mirroring: Mirroring,
}

impl NesPPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        NesPPU {
            chr_rom: chr_rom,
            mirroring: mirroring,
            vram: [0; VRAM_MAX_SIZE],
            oam_data: [0; OAM_DATA_MAX_SIZE],
            palette_table: [0; PALETTE_TABLE_MAX_SIZE],
            addr: AddrRegister::new(),
            ctrl: ControlRegister::new(),
            internal_data_buf: 0,
        }
    }

    fn write_to_ppu_addr(&mut self, value: u8) {
        self.addr.update(value);
    }

    fn write_to_ctrl(&mut self, value: u8) {
        self.ctrl.update(value);
    }

    fn increment_vram_addr(&mut self) {
        self.addr.increment(self.ctrl.vram_addr_increment());
    }

    /// read or write access to 0x2007 increments the PPU Address (0x2006)
    fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr {
            0..=0x1fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                result
            }
            0x2000..=0x2fff => {
                let result = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                result
            }
            0x3000..=0x3eff => panic!(
                "addr space 0x3000..0x3eff is not expected to be used, requested = {} ",
                addr
            ),
            0x3f00..=0x3fff => self.palette_table[(addr - 0x3f00) as usize],
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    fn write_data(&mut self, value: u8) {
        todo!("");
    }

    /// Horizontal:
    ///   [ A ] [ a ]
    ///   [ B ] [ b ]

    /// Vertical:
    ///   [ A ] [ B ]
    ///   [ a ] [ b ]
    pub fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10_1111_1111_1111; // mirror down 0x3000-0x3eff to 0x2000 - 0x2eff
        let vram_index = mirrored_vram - 0x2000; // to vram vector
        let name_table = vram_index / 0x400; // to the name table index
        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }
}