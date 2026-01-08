use serde::{Deserialize, de::Visitor};

#[derive(Clone, Debug)]
pub enum FlagOps {
    AlwaysSet,
    AlwaysReset,
    Dependent,
    DoNothing,
}

struct FlagOpsVisitor;

impl<'de > Visitor<'de> for FlagOpsVisitor {
    type Value = FlagOps;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expecting a &str or String of a single char")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        match v {
            "Z" => Ok(FlagOps::Dependent),
            "H" => Ok(FlagOps::Dependent),
            "C" => Ok(FlagOps::Dependent),
            "N" => Ok(FlagOps::Dependent),
            "0" => Ok(FlagOps::AlwaysReset),
            "1" => Ok(FlagOps::AlwaysSet),
            "-" => Ok(FlagOps::DoNothing),
            _ => Err(E::custom(format!("Bad flag op encountered! {}", v)))
        }
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.visit_str(&v)
    }
}

impl<'de> Deserialize<'de> for FlagOps {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_any(FlagOpsVisitor)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FlagsRegister {
    pub zero: bool,
    pub negative: bool, 
    pub half_carry: bool,
    pub carry: bool,
}

impl FlagsRegister {
    pub fn init() -> Self {
        Self {
            zero: false,
            negative: false,
            half_carry: false,
            carry: false,
        }
    }
}

impl From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> Self {
        let mut flag_int = 0x00;
        flag_int |= match flag.zero {
            true => 1 << 7,
            false => 0 << 7,
        };
        flag_int |= match flag.negative {
            true => 1 << 6,
            false => 0 << 6,
        };
        flag_int |= match flag.half_carry {
            true => 1 << 5,
            false => 0 << 5,
        };
        flag_int |= match flag.carry {
            true => 1 << 4,
            false => 0 << 4,
        };
        flag_int
    }
}

impl From<u8> for FlagsRegister {
    fn from(value: u8) -> Self {
       Self {
            zero: (value & 0x80) == 0x80,
            negative: (value & 0x40) == 0x40,
            half_carry: (value & 0x20) == 0x20,
            carry: (value & 0x10) == 0x10,
       } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
