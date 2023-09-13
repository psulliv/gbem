use crate::machine::MachineState;
use crate::sm83::{ConditionFlags, ProcessorState};
use std::fmt::Write;
use std::fs;
use std::io;
use std::io::prelude::*;

pub fn pause() {
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn debug_console_print(print_this: &String) {
    if cfg!(target_arch = "wasm32") {
        unsafe {
            web_sys::console::log_1(&print_this.into());
        }
    }
    if cfg!(target_arch = "x86_64") {
        println!("{}", &print_this);
    }
}

pub fn load_json_opcodes() -> serde_json::Value {
    let raw_json: String =
        String::from_utf8_lossy(&fs::read("sm83_opcodes.json").unwrap()).to_string();
    serde_json::from_str(&raw_json).unwrap()
}

pub fn opcode_printer(state: &MachineState) {
    // 1a35 INX    D   .sp..  A $00 B $af C $00 D $1b E $52 H $20 L $52 SP 23fe
    // print the address in hex, print the instruction and source/dest, flags, registers
    // A B C D E H L SP

    let (opcode_mnem, opcode_len, _) =
        debug_print_op_code(state.mem_map[state.processor_state.prog_counter]);
    let opcode_message = match opcode_len {
        1 => format!("{} ", opcode_mnem),
        2 => format!(
            "{} {:#04x} ",
            opcode_mnem,
            state.mem_map[state.processor_state.prog_counter + 1],
        ),
        3 => format!(
            "{} {:#04x} {:#04x} ",
            opcode_mnem,
            state.mem_map[state.processor_state.prog_counter + 1],
            state.mem_map[state.processor_state.prog_counter + 2],
        ),
        _ => panic!(),
    };

    // mnemonic, length, message
    let (opcode_mnem, _, _) =
        debug_print_op_code(state.mem_map[state.processor_state.prog_counter]);
    debug_console_print(&format!(
        "{:04x} {} ",
        state.processor_state.prog_counter, opcode_mnem,
    ))
}

// Todo: revamp for SM83
pub fn processor_state_printer(state: &MachineState) {
    let mut this_string = String::new();
    write!(this_string, "{} ", state.processor_state.instr_count);
    write!(
        this_string,
        "{}",
        if state.processor_state.flags.contains(ConditionFlags::Z) {
            "z"
        } else {
            "."
        }
    );
    write!(
        this_string,
        "{}",
        if state.processor_state.flags.contains(ConditionFlags::S) {
            "s"
        } else {
            "."
        }
    );
    write!(
        this_string,
        "{}",
        if state.processor_state.flags.contains(ConditionFlags::P) {
            "p"
        } else {
            "."
        }
    );
    write!(
        this_string,
        "{}",
        if state.processor_state.flags.contains(ConditionFlags::CY) {
            "c"
        } else {
            "."
        }
    );
    write!(
        this_string,
        "{}",
        if state.processor_state.flags.contains(ConditionFlags::AC) {
            "a"
        } else {
            "."
        }
    );

    write!(
        this_string,
        " A ${:02x} B ${:02x} C ${:02x} D ${:02x} E ${:02x} H ${:02x} L ${:02x} SP {:04x}",
        state.processor_state.reg_a,
        state.processor_state.reg_b,
        state.processor_state.reg_c,
        state.processor_state.reg_d,
        state.processor_state.reg_e,
        state.processor_state.reg_h,
        state.processor_state.reg_l,
        state.processor_state.stack_pointer,
    );
    debug_console_print(&this_string);
}

// Todo: load the opcode json file, return opcode's string and number of cycles
pub fn debug_print_op_code(opcode: u8) -> (String, u8, String) {
    //! Print out the current opcode and return the number of bytes it uses including itself
    panic!("Not implemented!");
    (
        String::from("Not Implemented! but should be opcode"),
        0,
        String::from("Not implemented! but should be a description"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sm83_opcode_list_basic() {
        assert!(True);
    }
}
