use std::collections::BTreeMap;

use binrw::binrw;

pub const EVT_INSTRUCTIONS_BASE_OFFSET: usize = 0x1000 + 4;

#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct RawInstruction {
    pub opcode: u32,
    pub length: u32,

    #[br(count = length - 8)]
    pub arguments: Vec<u8>,
}

pub type InstructionOffset = usize;

#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Evt {
    pub magic: u32,
    pub data: [u8; 0x1000],

    #[br(parse_with = binrw::helpers::until_eof)]
    pub instructions: Vec<RawInstruction>,
}

impl Evt {
    pub fn get_instructions_by_offset(&self) -> BTreeMap<InstructionOffset, RawInstruction> {
        let mut map = BTreeMap::<InstructionOffset, RawInstruction>::new();
        let mut offset = EVT_INSTRUCTIONS_BASE_OFFSET;
        for instruction in self.instructions.iter() {
            map.insert(offset, instruction.clone());
            offset += instruction.length as usize;
        }

        map
    }
}
