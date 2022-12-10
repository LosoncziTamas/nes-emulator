
use crate::hardware::cpu::CPU;
use crate::ram::bus::{Bus, Mem};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xA9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xa9_tax() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xAA, 0x00]);
        assert_eq!(cpu.register_x, 10);
    }

    #[test]
    fn test_0x_inx() {
        let mut cpu = CPU::new();
        cpu.register_x = 1;
        cpu.interpret(vec![0xE8, 0x00]);
        assert_eq!(cpu.register_x, 2);
    }

    #[test]
    fn invalid_ram_access() {
        let invalid_ram_addr = 0x2457;
        assert_eq!(0, Mem::read(invalid_ram_addr));
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
        assert!(0x42 == bus.write(write_addr, write_data));
    }
}
