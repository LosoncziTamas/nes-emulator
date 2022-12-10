
// TODO: remove this when merged with cartridge code
#[derive(Debug, PartialEq)]
pub enum Mirroring {
   Vertical,
   Horizontal,
   FourScreen
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
            internal_data_buf: 0
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
            0x3000..=0x3eff => panic!("addr space 0x3000..0x3eff is not expected to be used, requested = {} ", addr),
            0x3f00..=0x3fff => {
                self.palette_table[(addr - 0x3f00) as usize]
            }
            _ => panic!("unexpected access to mirrored space {}", addr),
        }
    }

    // Horizontal:
   //   [ A ] [ a ]
   //   [ B ] [ b ]
 
   // Vertical:
   //   [ A ] [ B ]
   //   [ a ] [ b ]

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

 const MIRROR_DOWN_ADDRESS_START: u16 = 0x3fff;


 pub struct AddrRegister {
    /// high byte first, low byte second
    value: (u8, u8), 
    hi_ptr: bool,
 }

 impl AddrRegister {
    pub fn new() -> Self {
        AddrRegister {
            value: (0, 0),
            hi_ptr: true,
        }
    }
    fn set(&mut self, data: u16) {
        self.value.0 = (data >> 8) as u8;
        self.value.1 = (data & 0xff) as u8;
    }
 
    pub fn update(&mut self, data: u8) {
        if self.hi_ptr {
            self.value.0 = data;
        } else {
            self.value.1 = data;
        }
 
        if self.get() > MIRROR_DOWN_ADDRESS_START {
            self.set(self.get() & 0b11_1111_1111_1111);
        }
        self.hi_ptr = !self.hi_ptr;
    }
 
    pub fn increment(&mut self, inc: u8) {
        let lo = self.value.1;
        self.value.1 = self.value.1.wrapping_add(inc);
        if lo > self.value.1 {
            self.value.0 = self.value.0.wrapping_add(1);
        }
        if self.get() > MIRROR_DOWN_ADDRESS_START {
            self.set(self.get() & 0b11_1111_1111_1111);
        }
    }
 
    pub fn reset_latch(&mut self) {
        self.hi_ptr = true;
    }
 
    pub fn get(&self) -> u16 {
        ((self.value.0 as u16) << 8) | (self.value.1 as u16)
    }
 }

 use bitflags::bitflags;
 bitflags! {

    // 7  bit  0
    // ---- ----
    // VPHB SINN
    // |||| ||||
    // |||| ||++- Base nametable address
    // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
    // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
    // |||| |     (0: add 1, going across; 1: add 32, going down)
    // |||| +---- Sprite pattern table address for 8x8 sprites
    // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
    // |||+------ Background pattern table address (0: $0000; 1: $1000)
    // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
    // |+-------- PPU master/slave select
    // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
    // +--------- Generate an NMI at the start of the
    //            vertical blanking interval (0: off; 1: on)
    pub struct ControlRegister: u8 {
        const NAMETABLE1              = 0b00000001;
        const NAMETABLE2              = 0b00000010;
        const VRAM_ADD_INCREMENT      = 0b00000100;
        const SPRITE_PATTERN_ADDR     = 0b00001000;
        const BACKROUND_PATTERN_ADDR  = 0b00010000;
        const SPRITE_SIZE             = 0b00100000;
        const MASTER_SLAVE_SELECT     = 0b01000000;
        const GENERATE_NMI            = 0b10000000;
    }
 }

 impl ControlRegister {
    pub fn new() -> Self {
        ControlRegister::from_bits_truncate(0b00000000)
    }
 
    pub fn vram_addr_increment(&self) -> u8 {
        if !self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            1
        } else {
            32
        }
    }
 
    pub fn update(&mut self, data: u8) {
        self.bits = data;
    }
 }