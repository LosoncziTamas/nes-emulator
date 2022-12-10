
use crate::hardware::cpu::CPU;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0xa9_lda() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
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
}
