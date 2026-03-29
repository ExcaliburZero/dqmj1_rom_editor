use crate::dqmj1_rom::{
    events::binary::{ArgumentKind, Evt, Opcode},
    strings::encoding::CharacterEncoding,
};

#[derive(Debug, Clone, Copy)]
pub enum ValueLocation {
    Pool0,
    Pool1,
    Constant,
    Pool3,
}

impl ValueLocation {
    pub fn from_u32(value: u32) -> ValueLocation {
        match value {
            0 => ValueLocation::Pool0,
            1 => ValueLocation::Pool1,
            2 => ValueLocation::Constant,
            3 => ValueLocation::Pool3,
            _ => panic!("Unrecognized value location id: {}", value),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Arg {
    Float(f32),
    Bytes(Vec<u8>),
    StringLit(String),
    Label(String),
    ValueLocation(ValueLocation),
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub label: Option<String>,
    pub opcode: String,
    pub args: Vec<Arg>,
}

#[derive(Debug, Clone)]
pub struct EventScript {
    pub data: Vec<u8>,
    pub instructions: Vec<Instruction>,
}

impl EventScript {
    pub fn from_evt(
        character_encoding: &CharacterEncoding,
        opcodes: &[Opcode],
        evt: &Evt,
    ) -> EventScript {
        let data = &evt.data;

        // TODO: find labels, implement as a separate pass?

        let mut instructions = vec![];
        for instruction in evt.instructions.iter() {
            let opcode = &opcodes[instruction.opcode as usize];

            let args =
                EventScript::parse_arguments(character_encoding, opcode, &instruction.arguments);

            instructions.push(Instruction {
                label: None,
                opcode: opcode.name.clone(),
                args,
            });
        }

        EventScript {
            data: data.to_vec(),
            instructions,
        }
    }

    pub fn parse_arguments(
        character_encoding: &CharacterEncoding,
        opcode: &Opcode,
        raw_arguments: &[u8],
    ) -> Vec<Arg> {
        let mut current = 0;
        let mut arguments = vec![];
        for arg_kind in opcode.arguments.iter() {
            match arg_kind {
                ArgumentKind::Bytes => {
                    arguments.push(Arg::Bytes(raw_arguments[current..].to_vec()));
                    current += raw_arguments.len() - current; // Note: assumes no further args
                }
                ArgumentKind::Dqmj1String => {
                    let string = character_encoding.read_string(&raw_arguments[current..]);

                    arguments.push(Arg::StringLit(string));
                    current += raw_arguments.len() - current; // Note: assumes no further args
                }
                ArgumentKind::AsciiString => {
                    let string = std::str::from_utf8(&raw_arguments[current..]).unwrap();

                    arguments.push(Arg::StringLit(string.to_string()));
                    current += raw_arguments.len() - current; // Note: assumes no further args
                }
                ArgumentKind::U32 => {
                    let value = f32::from_le_bytes(
                        raw_arguments[current..(current + 4)].try_into().unwrap(),
                    );

                    arguments.push(Arg::Float(value));
                    current += 4;
                }
                ArgumentKind::ValueLocation => {
                    let value = u32::from_le_bytes(
                        raw_arguments[current..(current + 4)].try_into().unwrap(),
                    );

                    arguments.push(Arg::ValueLocation(ValueLocation::from_u32(value)));
                    current += 4;
                }
                ArgumentKind::InstructionLocation => {
                    let value = u32::from_le_bytes(
                        raw_arguments[current..(current + 4)].try_into().unwrap(),
                    );
                    let label = format!("0x:{:x}", value);

                    arguments.push(Arg::Label(label));
                    current += 4;
                }
            }
        }

        assert!(current == raw_arguments.len());

        arguments
    }
}
