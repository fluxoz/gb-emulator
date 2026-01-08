#[cfg(test)]
mod tests {
    use crate::flags::FlagsRegister;
    #[test]
    fn flag_zero() {
        let flags_register = 0b0000_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_16() {
        let flags_register = 0b0001_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_17() {
        let flags_register = 0b0010_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_18() {
        let flags_register = 0b0011_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_19() {
        let flags_register = 0b0100_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_20() {
        let flags_register = 0b0101_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_21() {
        let flags_register = 0b0110_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_22() {
        let flags_register = 0b0111_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_23() {
        let flags_register = 0b1000_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_24() {
        let flags_register = 0b1001_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_25() {
        let flags_register = 0b1010_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_26() {
        let flags_register = 0b1011_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_27() {
        let flags_register = 0b1100_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_28() {
        let flags_register = 0b1101_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_29() {
        let flags_register = 0b1110_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }

    #[test]
    fn flag_30() {
        let flags_register = 0b1111_0000;
        let fr_struct: FlagsRegister = flags_register.into();
        let fr_binary: u8 = fr_struct.clone().into();
        assert!(fr_binary == flags_register);
    }
}

#[cfg(test)]
mod cpu_tests {
    use crate::cpu::CPU;

    fn setup_cpu_with_rom(rom: Vec<u8>) -> CPU {
        let mut cpu = CPU::new();
        let mut full_rom = vec![0; 0x100];
        full_rom.extend(rom);
        cpu.load_rom(full_rom);
        cpu
    }

    #[test]
    fn test_nop() {
        let mut cpu = setup_cpu_with_rom(vec![0x00]); // NOP
        let cycles = cpu.step();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.get_pc(), 0x0101);
    }

    #[test]
    fn test_ld_immediate() {
        let mut cpu = setup_cpu_with_rom(vec![
            0x3E, 0x42, // LD A, $42
            0x06, 0x10, // LD B, $10
            0x0E, 0x20, // LD C, $20
        ]);
        
        cpu.step();
        cpu.step();
        cpu.step();
        
        assert_eq!(cpu.get_pc(), 0x0106);
    }

    #[test]
    fn test_inc_dec() {
        let mut cpu = setup_cpu_with_rom(vec![
            0x06, 0x10, // LD B, $10
            0x04,       // INC B
            0x05,       // DEC B
            0x05,       // DEC B
        ]);
        
        cpu.step(); // LD B, $10
        cpu.step(); // INC B (B becomes $11)
        cpu.step(); // DEC B (B becomes $10)
        cpu.step(); // DEC B (B becomes $0F)
        
        assert_eq!(cpu.get_pc(), 0x0105);
    }

    #[test]
    fn test_16bit_load() {
        let mut cpu = setup_cpu_with_rom(vec![
            0x01, 0x34, 0x12, // LD BC, $1234
            0x11, 0x78, 0x56, // LD DE, $5678
            0x21, 0xBC, 0x9A, // LD HL, $9ABC
        ]);
        
        cpu.step();
        cpu.step();
        cpu.step();
        
        assert_eq!(cpu.get_pc(), 0x0109);
    }

    #[test]
    fn test_halt() {
        let mut cpu = setup_cpu_with_rom(vec![
            0x76, // HALT
        ]);
        
        let cycles1 = cpu.step();
        assert_eq!(cycles1, 4);
        
        // After HALT, CPU should keep returning 4 cycles but not advance PC
        let cycles2 = cpu.step();
        assert_eq!(cycles2, 4);
        assert_eq!(cpu.get_pc(), 0x0101); // PC stays at next instruction
    }

    #[test]
    fn test_jump_relative() {
        let mut cpu = setup_cpu_with_rom(vec![
            0x18, 0x02, // JR +2
            0x00,       // NOP (skipped)
            0x00,       // NOP (skipped)
            0x3E, 0x42, // LD A, $42 (executed)
        ]);
        
        cpu.step(); // JR +2
        assert_eq!(cpu.get_pc(), 0x0104);
        
        cpu.step(); // LD A, $42
        assert_eq!(cpu.get_pc(), 0x0106);
    }

    #[test]
    fn test_stack_operations() {
        let mut cpu = setup_cpu_with_rom(vec![
            0x01, 0x34, 0x12, // LD BC, $1234
            0xC5,             // PUSH BC
            0x01, 0x00, 0x00, // LD BC, $0000
            0xC1,             // POP BC
        ]);
        
        cpu.step(); // LD BC, $1234
        cpu.step(); // PUSH BC
        cpu.step(); // LD BC, $0000
        cpu.step(); // POP BC - BC should be $1234 again
        
        assert_eq!(cpu.get_pc(), 0x0108);
    }

    #[test]
    fn test_timing_accuracy() {
        let mut cpu = setup_cpu_with_rom(vec![
            0x00,       // NOP - 4 cycles
            0x3E, 0x42, // LD A, $42 - 8 cycles
            0x04,       // INC B - 4 cycles
        ]);
        
        assert_eq!(cpu.get_ticks(), 0);
        
        cpu.step(); // NOP
        assert_eq!(cpu.get_ticks(), 4);
        
        cpu.step(); // LD A, $42
        assert_eq!(cpu.get_ticks(), 12);
        
        cpu.step(); // INC B
        assert_eq!(cpu.get_ticks(), 16);
    }

    #[test]
    fn test_cb_prefix_bit_operations() {
        let mut cpu = setup_cpu_with_rom(vec![
            0x3E, 0xAA,       // LD A, $AA (10101010)
            0xCB, 0x47,       // BIT 0, A
            0xCB, 0x07,       // RLC A
        ]);
        
        cpu.step(); // LD A, $AA
        cpu.step(); // BIT 0, A
        cpu.step(); // RLC A
        
        assert_eq!(cpu.get_pc(), 0x0106);
    }

    #[test]
    fn test_arithmetic_operations() {
        let mut cpu = setup_cpu_with_rom(vec![
            0x3E, 0x10, // LD A, $10
            0x06, 0x05, // LD B, $05
            0x80,       // ADD A, B (A = $15)
            0x90,       // SUB B (A = $10)
        ]);
        
        cpu.step(); // LD A, $10
        cpu.step(); // LD B, $05
        cpu.step(); // ADD A, B
        cpu.step(); // SUB B
        
        assert_eq!(cpu.get_pc(), 0x0106);
    }
}
