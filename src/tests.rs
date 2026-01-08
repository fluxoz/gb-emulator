#[cfg(test)]
mod tests {
    use crate::FlagsRegister;
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
