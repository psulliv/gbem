use std::fs;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct OpcodeInfo {
    mnemonic: String,
    bytes: u32,
    cycles: Vec<u32>,
    operands: Vec<serde_json::Value>,
    immediate: bool,
    flags: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct SM83OpcodesJson {
    cbprefixed: HashMap<String, OpcodeInfo>,
    unprefixed: HashMap<String, OpcodeInfo>,
}

pub fn load_json_opcodes() -> SM83OpcodesJson {
    let raw_json: String =
        String::from_utf8_lossy(&fs::read("sm83_opcodes.json").unwrap()).to_string();
    serde_json::from_str(&raw_json).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sm83_opcode_list_basic() {
        assert!(true);
    }
}
