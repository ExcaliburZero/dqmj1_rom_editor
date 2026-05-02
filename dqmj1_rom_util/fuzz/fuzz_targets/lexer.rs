#![no_main]

use dqmj1_rom_util::events::assembly::lexer::lex_dqmj1_asm;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|contents: &str| {
    lex_dqmj1_asm(contents);
});
