use crate::flags::FlagOps;
use serde::Deserialize;
use std::error::Error;

#[derive(Clone, Debug, Deserialize)]
pub struct OpCodeRaw {
    mnemonic: String,
    length: u8,
    cycles: Vec<u8>,
    flags: [FlagOps; 4],
    addr: String,
    group: String,
    operand1: Option<String>,
    operand2: Option<String>
}

#[derive(Clone, Debug)]
pub struct OpCode {
    pub prefixed: bool,
    pub mnemonic: String,
    pub length: u8,
    pub cycles: (Option<u8>, Option<u8>),
    pub flags: [FlagOps; 4],
    pub addr: u16,
    pub group: String,
    pub operand1: Option<String>,
    pub operand2: Option<String>
}

fn parse_hex_string_u16(s: &str) -> Result<u16, Box<dyn Error>> {
    let raw = s;
    let without_prefix = raw.trim_start_matches("0x");
    let x = u16::from_str_radix(without_prefix, 16)?;
    Ok(x)
}

impl From<(OpCodeRaw, bool)> for OpCode {
    fn from(value: (OpCodeRaw, bool)) -> Self {
        let cycle1 = value.0.cycles.first();
        let cycle2 = value.0.cycles.get(1);
        Self {
            prefixed: value.1,
            mnemonic: value.0.mnemonic,
            length: value.0.length,
            cycles: (cycle1.cloned(), cycle2.cloned()),
            flags: value.0.flags,
            addr: parse_hex_string_u16(&value.0.addr).unwrap(),
            group: value.0.group,
            operand1: value.0.operand1,
            operand2: value.0.operand2,
        }
    }
}

pub fn load_opcodes() -> Result<(Vec<OpCode>, Vec<OpCode>), Box<dyn Error>> {
    let unprefixed = include_str!("./unprefixed.json");
    let cbprefixed = include_str!("./cbprefixed.json");
    // println!("UNPREFIXED: {:?}", unprefixed);
    // println!("CBPREFIXED: {:?}", cbprefixed);
    let unprefixed_opcodes_raw: Vec<OpCodeRaw> = serde_json::from_str(unprefixed).unwrap();
    let cbprefixed_opcodes_raw: Vec<OpCodeRaw> = serde_json::from_str(cbprefixed).unwrap();

    let unprefixed_opcodes: Vec<OpCode> = unprefixed_opcodes_raw.into_iter().map(|x| (x, false).into()).collect();
    let cbprefixed_opcodes: Vec<OpCode> = cbprefixed_opcodes_raw.into_iter().map(|x| (x, true).into()).collect();

    Ok((unprefixed_opcodes, cbprefixed_opcodes))
}
