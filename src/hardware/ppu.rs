
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
        }
    }
 }

 pub struct AddrRegister {
    value: (u8, u8),
    hi_ptr: bool,
 }

 