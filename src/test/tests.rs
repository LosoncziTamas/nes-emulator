
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
    }
}