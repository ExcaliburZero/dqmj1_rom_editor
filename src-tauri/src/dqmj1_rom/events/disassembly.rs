use std::collections::BTreeMap;

use serde::Deserialize;

use crate::dqmj1_rom::{
    events::binary::{Evt, InstructionOffset, EVT_INSTRUCTIONS_BASE_OFFSET},
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

    pub fn to_string(&self) -> String {
        match self {
            ValueLocation::Pool0 => "Pool_0".to_string(),
            ValueLocation::Pool1 => "Pool_1".to_string(),
            ValueLocation::Constant => "Const".to_string(),
            ValueLocation::Pool3 => "Pool_3".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Arg {
    Float(f32),
    Bytes(Vec<u8>),
    StringLit(String),
    InstructionLocation(InstructionOffset),
    ValueLocation(ValueLocation),
}

#[derive(Debug)]
pub struct InstructionDestinations {
    pub normal: Option<InstructionOffset>,
    pub jump: Option<InstructionOffset>,
}

#[derive(Debug, Clone)]
pub struct DecodedInstruction<'a> {
    pub opcode: &'a Opcode,
    pub args: Vec<Arg>,
    pub size: usize,
}

impl DecodedInstruction<'_> {
    pub fn get_destinations(&self, offset: InstructionOffset) -> InstructionDestinations {
        if let Some(Arg::InstructionLocation(destination)) = self.args.first() {
            if self.opcode.id == 0x0C {
                InstructionDestinations {
                    normal: None,
                    jump: Some(*destination + EVT_INSTRUCTIONS_BASE_OFFSET),
                }
            } else {
                InstructionDestinations {
                    normal: Some(self.next_offset(offset)),
                    jump: Some(*destination + EVT_INSTRUCTIONS_BASE_OFFSET),
                }
            }
        } else {
            InstructionDestinations {
                normal: Some(self.next_offset(offset)),
                jump: None,
            }
        }
    }

    pub fn next_offset(&self, offset: InstructionOffset) -> InstructionOffset {
        offset + self.size
    }
}

#[derive(Debug)]
pub struct DisassembledEvt<'a> {
    pub data: [u8; 0x1000],
    pub instructions: BTreeMap<InstructionOffset, DecodedInstruction<'a>>,
}

impl DisassembledEvt<'_> {
    pub fn from_evt<'a>(
        evt: &Evt,
        character_encoding: &CharacterEncoding,
        opcodes: &'a [Opcode],
    ) -> DisassembledEvt<'a> {
        let mut instructions = BTreeMap::new();
        for (offset, instruction) in evt.get_instructions_by_offset() {
            let size = instruction.length as usize;
            let opcode = &opcodes[instruction.opcode as usize];

            let args = DisassembledEvt::parse_arguments(
                &instruction.arguments,
                character_encoding,
                opcode,
            );

            instructions.insert(offset, DecodedInstruction { opcode, args, size });
        }

        DisassembledEvt {
            data: evt.data,
            instructions,
        }
    }

    fn parse_arguments(
        raw_arguments: &[u8],
        character_encoding: &CharacterEncoding,
        opcode: &Opcode,
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

                    arguments.push(Arg::InstructionLocation(value as InstructionOffset));
                    current += 4;
                }
            }
        }

        assert!(current == raw_arguments.len());

        arguments
    }
}

#[derive(Debug)]
pub enum ArgumentKind {
    Bytes,
    U32,
    Dqmj1String,
    AsciiString,
    InstructionLocation,
    ValueLocation,
}

impl ArgumentKind {
    pub fn from_string(string: &str) -> ArgumentKind {
        match string {
            "Bytes" => ArgumentKind::Bytes,
            "U32" => ArgumentKind::U32,
            "String" => ArgumentKind::Dqmj1String,
            "AsciiString" => ArgumentKind::AsciiString,
            "InstructionLocation" => ArgumentKind::InstructionLocation,
            "ValueLocation" => ArgumentKind::ValueLocation,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
pub struct Opcode {
    pub id: u8,
    pub name: String,
    pub arguments: Vec<ArgumentKind>,
}

#[derive(Debug, Deserialize)]
struct OpcodeRecord {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Arguments")]
    pub arguments: String,
    #[serde(rename = "Notes")]
    pub _notes: Option<String>,
}

impl OpcodeRecord {
    pub fn get_id_u8(&self) -> u8 {
        u8::from_str_radix(&self.id[2..], 16).unwrap()
    }

    pub fn get_arguments(&self) -> Vec<ArgumentKind> {
        if self.arguments == "[]" {
            return vec![];
        }

        let parts = self.arguments[1..self.arguments.len() - 1].split(", ");
        parts.map(ArgumentKind::from_string).collect()
    }
}

impl Opcode {
    pub fn multiple_from_csv(filepath: &str) -> Vec<Opcode> {
        //let contents = std::fs::read_to_string(filepath)
        //    .unwrap_or_else(|_| panic!("opcodes file not found: {}", filepath));
        let contents = include_bytes!("opcodes.csv");

        let mut reader = csv::Reader::from_reader(&contents[..]);

        reader
            .deserialize::<OpcodeRecord>()
            .map(|record| record.unwrap())
            .map(|record| Opcode {
                id: record.get_id_u8(),
                name: record.name.clone(),
                arguments: record.get_arguments(),
            })
            .collect()
    }
}
