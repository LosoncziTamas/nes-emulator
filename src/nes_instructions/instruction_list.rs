#[derive(Copy, Clone)]
pub enum Codes {
    Brk = 0x00,
    Inx = 0xE8,
    Lda = 0xA9,
    Tax = 0xAA,
    Zpg = 0x05,
}
