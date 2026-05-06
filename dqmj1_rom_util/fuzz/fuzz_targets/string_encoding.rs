#![no_main]

use dqmj1_rom_util::{regions::Region, strings::encoding::CharacterEncoding};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|string: &str| {
    let na = CharacterEncoding::get(Region::NorthAmerica);
    let jp = CharacterEncoding::get(Region::Japan);

    na.encode_string(string);
    jp.encode_string(string);
});
