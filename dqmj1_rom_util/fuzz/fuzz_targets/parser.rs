#![no_main]

use dqmj1_rom_util::events::{assembly::parser::parse_dqmj1_asm, disassembly::Opcode};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|contents: &str| {
    let opcodes = Opcode::get();
    parse_dqmj1_asm(contents, &opcodes);
});
