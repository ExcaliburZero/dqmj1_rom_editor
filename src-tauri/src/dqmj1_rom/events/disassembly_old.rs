/*use std::{
    collections::BTreeMap,
    io::{self, Write},
};

use serde::Deserialize;

use crate::dqmj1_rom::{
    events::binary::{Evt, InstructionOffset},
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
    Label(String),
    ValueLocation(ValueLocation),
}

#[derive(Debug, Clone)]
pub struct DecodedInstruction {
    pub label: Option<String>,
    pub opcode: String,
    pub args: Vec<Arg>,
}

impl DecodedInstruction {
    pub fn does_jump(&self) -> bool {
        self.args.len() > 0 && matches!(self.args.iter().next().unwrap(), Arg::Label(_))
    }
}

#[derive(Debug, Clone)]
pub struct EventScript {
    pub data: Vec<u8>,
    pub instructions: BTreeMap<InstructionOffset, DecodedInstruction>,
}

impl EventScript {
    pub fn from_evt(
        character_encoding: &CharacterEncoding,
        opcodes: &[Opcode],
        evt: &Evt,
    ) -> EventScript {
        let data = &evt.data;

        // TODO: find labels, implement as a separate pass?

        let mut instructions = BTreeMap::new();
        for (offset, instruction) in evt.get_instructions_by_offset() {
            let opcode = &opcodes[instruction.opcode as usize];

            let args =
                EventScript::parse_arguments(character_encoding, opcode, &instruction.arguments);

            instructions.insert(
                offset,
                DecodedInstruction {
                    label: None,
                    opcode: opcode.name.clone(),
                    args,
                },
            );
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
                    let label = format!("0x{:x}", value);

                    arguments.push(Arg::Label(label));
                    current += 4;
                }
            }
        }

        assert!(current == raw_arguments.len());

        arguments
    }

    /*pub fn write_dqmj1_script<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, ".data:")?;
        writeln!(writer, "    {}", EventScript::bytes_to_literal(&self.data))?;

        writeln!(writer, ".code:")?;
        for (_, instruction) in self.instructions.iter() {
            // TODO: labels
            write!(writer, "    {:<12}", instruction.opcode)?;

            for arg in instruction.args.iter() {
                let string = match arg {
                    Arg::Float(f) => EventScript::format_f32(*f),
                    Arg::Label(label) => label.to_string(), // TODO: map to correct location
                    Arg::ValueLocation(location) => location.to_string(), // TODO: map to correct location
                    Arg::StringLit(string) => format!("\"{}\"", string), // TODO: map to correct location
                    Arg::Bytes(bytes) => EventScript::bytes_to_literal(bytes),
                };

                write!(writer, " {}", string)?;
            }

            writer.write_all("\n".as_bytes())?;
        }

        Ok(())
    }*/

    fn bytes_to_literal(bytes: &[u8]) -> String {
        let mut parts = vec!["b\"".to_string()];
        for byte in bytes {
            parts.push(format!("\\x{:02x}", byte));
        }
        parts.push("\"\n".to_string());

        parts.join("")
    }

    fn format_f32(f: f32) -> String {
        if f.fract() == 0.0 && f.abs() < 1e10 {
            format!("{:.1}", f) // "1.0", "30.0", "124.0"
        } else {
            format!("{:e}", f) // "1.401298464324817e-45"
        }
    }
}

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
*/
