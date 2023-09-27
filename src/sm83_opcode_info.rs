use std::fs;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct OpcodeInfo {
    mnemonic: String,
    bytes: u32,
    cycles: Vec<u32>,
    operands: Vec<serde_json::Value>,
    immediate: bool,
    flags: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SM83OpcodesJson {
    cbprefixed: HashMap<String, OpcodeInfo>,
    unprefixed: HashMap<String, OpcodeInfo>,
}

pub fn load_json_opcodes() -> SM83OpcodesJson {
    let raw_json: String =
        String::from_utf8_lossy(&fs::read("src/sm83_opcodes.json").unwrap()).to_string();
    serde_json::from_str(&raw_json).unwrap()
}

pub fn get_unprefixed_opcode_cycle_counts(opcodes: &SM83OpcodesJson) -> HashMap<String, u32> {
    // duration of conditional call/jmp insns is different if the branch is not taken
    // To accurately count the cycles we would want to check the processor state at execution
    // time for the given insn
    // Todo: accurately track the cycle counts
    opcodes
        .unprefixed
        .iter()
        .map(|(k, v)| (k.clone(), v.cycles[0]))
        .collect()
}

pub fn get_cbprefixed_opcode_cycle_counts(opcodes: &SM83OpcodesJson) -> HashMap<String, u32> {
    // CB prefixed insns will have the same cycle counts thankfully
    opcodes
        .cbprefixed
        .iter()
        .map(|(k, v)| (k.clone(), v.cycles[0]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sm83_opcode_list_basic() {
        let opcode_info = load_json_opcodes();
        assert!(opcode_info.cbprefixed.contains_key("0x00"));
    }

    #[test]
    fn test_sm83_opcode_list_0x00_contents() {
        let opcode_info = load_json_opcodes();

        let zero_x_00 = opcode_info.unprefixed.get("0x00").unwrap();
        assert!(zero_x_00.mnemonic == "NOP");
    }

    #[test]
    fn test_sm83_opcode_cb_cycle_count() {
        let opcode_info = load_json_opcodes();
        let bit_2_hl_opcode = opcode_info.cbprefixed["0x56"].clone();
        let cycle_counts = get_cbprefixed_opcode_cycle_counts(&opcode_info);
        assert!(cycle_counts["0x56"] == bit_2_hl_opcode.cycles[0])
    }
}
