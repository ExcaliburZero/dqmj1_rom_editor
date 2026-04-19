use std::{
    collections::{BTreeMap, BTreeSet},
    io::{self, Write},
};

use serde::Deserialize;

use crate::dqmj1_rom::{
    events::binary::{Evt, RawInstruction, EVT_INSTRUCTIONS_BASE_OFFSET, EVT_MAGIC},
    strings::encoding::CharacterEncoding,
};

const EXIT: u8 = 0x02;
const START_EVENT: u8 = 0x0A;
const JUMP: u8 = 0x0C;

pub type Label = String;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

    pub fn to_u32(&self) -> u32 {
        match self {
            ValueLocation::Pool0 => 0,
            ValueLocation::Pool1 => 1,
            ValueLocation::Constant => 2,
            ValueLocation::Pool3 => 3,
        }
    }

    pub fn to_asm_string(&self) -> String {
        match self {
            ValueLocation::Pool0 => "Pool_0".to_string(),
            ValueLocation::Pool1 => "Pool_1".to_string(),
            ValueLocation::Constant => "Const".to_string(),
            ValueLocation::Pool3 => "Pool_3".to_string(),
        }
    }

    pub fn from_asm_string(string: &str) -> Option<ValueLocation> {
        match string {
            "Pool_0" => Some(ValueLocation::Pool0),
            "Pool_1" => Some(ValueLocation::Pool1),
            "Const" => Some(ValueLocation::Constant),
            "Pool_3" => Some(ValueLocation::Pool3),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Float(f32),
    Bytes(Vec<u8>),
    StringLit(String),
    JumpDestination(Label),
    ValueLocation(ValueLocation),
}

impl Eq for Arg {} // Note: needed since f32 does not support Eq

#[derive(Debug)]
pub struct InstructionDestinations {
    pub normal: bool,
    pub jump: Option<Label>,
    pub fork: Option<Label>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DecodedInstruction<'a> {
    pub opcode: &'a Opcode,
    pub args: Vec<Arg>,
    pub label: Option<String>,
}

impl DecodedInstruction<'_> {
    pub fn get_destinations(&self) -> InstructionDestinations {
        if self.opcode.id == EXIT {
            InstructionDestinations {
                normal: false,
                jump: None,
                fork: None,
            }
        } else if self.opcode.id == START_EVENT {
            if let Arg::JumpDestination(fork_dest) = self.args.first().unwrap() {
                InstructionDestinations {
                    normal: true,
                    jump: None,
                    fork: Some(fork_dest.clone()),
                }
            } else {
                panic!();
            }
        } else if let Some(Arg::JumpDestination(destination)) = self.args.first() {
            if self.opcode.id == JUMP {
                InstructionDestinations {
                    normal: false,
                    jump: Some(destination.clone()),
                    fork: None,
                }
            } else {
                InstructionDestinations {
                    normal: true,
                    jump: Some(destination.clone()),
                    fork: None,
                }
            }
        } else {
            InstructionDestinations {
                normal: true,
                jump: None,
                fork: None,
            }
        }
    }

    pub fn get_raw_size_bytes(&self, character_encoding: &CharacterEncoding) -> usize {
        let header_size = 8; // opcode + length

        let mut args_size = 0;
        for (arg, arg_kind) in self.args.iter().zip(self.opcode.arguments.iter()) {
            args_size += match arg {
                Arg::Float(_) => 4,
                Arg::JumpDestination(_) => 4,
                Arg::ValueLocation(_) => 4,
                Arg::Bytes(bytes) => bytes.len(),
                Arg::StringLit(string) => match arg_kind {
                    ArgumentKind::AsciiString => Self::round_up_to_multiple_of_4(string.len() + 1), // characters + null terminator
                    ArgumentKind::Dqmj1String => {
                        let encoded_string = character_encoding.encode_string(string);
                        Self::round_up_to_multiple_of_4(encoded_string.len())
                    }
                    _ => panic!(),
                },
            };
        }

        assert_eq!(args_size % 4, 0);

        header_size + args_size
    }

    fn round_up_to_multiple_of_4(value: usize) -> usize {
        if value.is_multiple_of(4) {
            value
        } else {
            (value / 4) * 4 + 4
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DisassembledEvt<'a> {
    pub data: [u8; 0x1000],
    pub instructions: Vec<(Option<Label>, DecodedInstruction<'a>)>,
}

impl DisassembledEvt<'_> {
    pub fn from_evt<'a>(
        evt: &Evt,
        character_encoding: &CharacterEncoding,
        opcodes: &'a [Opcode],
    ) -> DisassembledEvt<'a> {
        // Decode instructions
        let mut instructions = vec![];
        for (offset, instruction) in evt.get_instructions_by_offset() {
            //let size = instruction.length as usize;
            let opcode = &opcodes[instruction.opcode as usize];

            let args = DisassembledEvt::parse_arguments(
                &instruction.arguments,
                character_encoding,
                opcode,
            );

            instructions.push((
                Some(format!("l{}", offset - EVT_INSTRUCTIONS_BASE_OFFSET)),
                DecodedInstruction {
                    opcode,
                    args,
                    label: None,
                },
            ));
        }

        // Find labels
        let mut labels = BTreeSet::new();
        for (_, instruction) in instructions.iter() {
            let destinations = instruction.get_destinations();
            if let Some(jump_dest) = destinations.jump {
                labels.insert(jump_dest);
            }

            if let Some(fork_dest) = destinations.fork {
                labels.insert(fork_dest);
            }
        }

        // Mark instructions that have labels
        for (label, instruction) in instructions.iter_mut() {
            let label = label.as_ref().unwrap();
            if labels.contains(label) {
                instruction.label = Some(label.clone());
            }
        }

        DisassembledEvt {
            data: evt.data,
            instructions,
        }
    }

    pub fn to_evt(&self, character_encoding: &CharacterEncoding) -> Evt {
        // Find byte offsets for each label
        let mut i = 0;
        let mut label_to_offset = BTreeMap::new();
        for (label, instruction) in self.instructions.iter() {
            if let Some(label) = label {
                label_to_offset.insert(label.clone(), i);
            }

            let size = instruction.get_raw_size_bytes(character_encoding);
            i += size;
        }

        // Create instructions
        let mut instructions = vec![];
        for (_, instruction) in self.instructions.iter() {
            let mut arguments = vec![];
            for (arg, arg_kind) in instruction
                .args
                .iter()
                .zip(instruction.opcode.arguments.iter())
            {
                match arg {
                    Arg::Float(float) => arguments.extend_from_slice(&float.to_le_bytes()),
                    Arg::JumpDestination(label) => arguments.extend_from_slice(
                        &((*label_to_offset.get(label).unwrap()) as u32).to_le_bytes(),
                    ),
                    Arg::ValueLocation(location) => {
                        arguments.extend_from_slice(&location.to_u32().to_le_bytes())
                    }
                    Arg::Bytes(bytes) => arguments.extend_from_slice(bytes),
                    Arg::StringLit(string) => {
                        let mut bytes: Vec<u8> = match arg_kind {
                            ArgumentKind::AsciiString => {
                                let mut bytes: Vec<u8> = string.bytes().collect();
                                bytes.push(0x00);
                                bytes
                            } // characters + null terminator
                            ArgumentKind::Dqmj1String => character_encoding.encode_string(string),
                            _ => panic!(),
                        };

                        // Pad to multiple of 4 bytes
                        let remainder = bytes.len() % 4;
                        if remainder != 0 {
                            bytes.resize(bytes.len() + (4 - remainder), 0xCC);
                        }

                        arguments.extend_from_slice(&bytes);
                    }
                };
            }

            let size = instruction.get_raw_size_bytes(character_encoding);
            instructions.push(RawInstruction {
                opcode: instruction.opcode.id as u32,
                length: size as u32,
                arguments,
            });
        }

        Evt {
            magic: EVT_MAGIC,
            data: self.data,
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
                    let relevant_bytes =
                        raw_arguments[current..].split(|&b| b == 0).next().unwrap();
                    assert!(!relevant_bytes.is_empty());
                    let string = std::str::from_utf8(relevant_bytes)
                        .unwrap()
                        .trim_end_matches('\0');

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

                    arguments.push(Arg::JumpDestination(format!("l{}", value)));
                    current += 4;
                }
            }
        }

        assert!(current == raw_arguments.len());

        arguments
    }

    pub fn write_asm<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        // TODO: implement
        writeln!(writer, ".data:")?;
        writeln!(writer, "    {}", Self::bytes_to_literal(&self.data))?;

        writeln!(writer, ".code:")?;
        self.write_instructions(writer)?;

        Ok(())
    }

    pub fn write_instructions<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        for (_, instruction) in self.instructions.iter() {
            if let Some(label) = &instruction.label {
                writeln!(writer, "  {}:", label)?;
            }

            write!(writer, "    {:<12}", instruction.opcode.name)?;

            for arg in instruction.args.iter() {
                let string = match arg {
                    Arg::Float(f) => Self::format_f32(*f),
                    Arg::JumpDestination(label) => label.to_string(),
                    Arg::ValueLocation(location) => location.to_asm_string(),
                    Arg::StringLit(string) => format!("\"{}\"", string),
                    Arg::Bytes(bytes) => Self::bytes_to_literal(bytes),
                };

                write!(writer, " {}", string)?;
            }

            writer.write_all("\n".as_bytes())?;
        }

        Ok(())
    }

    fn bytes_to_literal(bytes: &[u8]) -> String {
        let mut parts = vec!["b\"".to_string()];
        for byte in bytes {
            parts.push(format!("\\x{:02x}", byte));
        }
        parts.push("\"".to_string());

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

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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
    pub fn get() -> Vec<Opcode> {
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

#[cfg(test)]
mod tests {
    use std::fs::File;

    use binrw::BinRead;
    use rstest::rstest;

    use crate::dqmj1_rom::regions::Region;

    use super::*;

    const SETU32: u8 = 0x15;
    const START_DIALOG: u8 = 0x25;
    const END_DIALOG: u8 = 0x26;
    const SHOW_DIALOG: u8 = 0x27;
    const SET_DIALOG: u8 = 0x29;
    const SPEAKER_NAME: u8 = 0x2A;

    fn instructions_as_string(script: &DisassembledEvt) -> String {
        let mut buf = Vec::new();
        script.write_instructions(&mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    fn read_evt_from_file(filepath: &str) -> Evt {
        let mut reader = File::open(filepath).unwrap();
        Evt::read(&mut reader).unwrap()
    }

    fn read_evt_from_file_and_disassemble<'a>(
        filepath: &str,
        opcodes: &'a [Opcode],
    ) -> DisassembledEvt<'a> {
        let mut reader = File::open(filepath).unwrap();
        let evt = Evt::read(&mut reader).unwrap();

        let character_encoding = CharacterEncoding::get(Region::NorthAmerica);

        DisassembledEvt::from_evt(&evt, &character_encoding, opcodes)
    }

    #[test]
    fn test_bytes_to_literal_empty() {
        let bytes = vec![];

        assert_eq!(DisassembledEvt::bytes_to_literal(&bytes), r#"b"""#)
    }

    #[test]
    fn test_bytes_to_literal_simple() {
        let bytes = vec![0x00, 0x01, 0x02, 0x03, 0x04];

        assert_eq!(
            DisassembledEvt::bytes_to_literal(&bytes),
            r#"b"\x00\x01\x02\x03\x04""#
        )
    }

    #[test]
    fn test_write_instructions_empty() {
        let opcodes = Opcode::get();
        let script = read_evt_from_file_and_disassemble("test/data/no_instructions.evt", &opcodes);

        assert_eq!(instructions_as_string(&script), "");
    }

    #[test]
    fn test_write_instructions_single_instruction() {
        let opcodes = Opcode::get();
        let script = read_evt_from_file_and_disassemble("test/data/only_exit.evt", &opcodes);

        assert_eq!(instructions_as_string(&script), "    Exit         0.0\n");
    }

    #[test]
    fn test_write_instructions_multiple_instructions() {
        let opcodes = Opcode::get();
        let script = read_evt_from_file_and_disassemble("test/data/nop_then_exit.evt", &opcodes);

        assert_eq!(
            instructions_as_string(&script),
            "    Nop0        \n    Exit         0.0\n"
        );
    }

    #[test]
    fn test_write_instructions_with_label() {
        let opcodes = Opcode::get();
        let script = read_evt_from_file_and_disassemble("test/data/jump_to_self.evt", &opcodes);

        assert_eq!(
            instructions_as_string(&script),
            "  l0:\n    Jump         l0\n"
        );
    }

    #[test]
    fn test_write_instructions_variety_of_instructions() {
        let opcodes = Opcode::get();
        let script = read_evt_from_file_and_disassemble("test/data/dialog.evt", &opcodes);

        assert_eq!(
            instructions_as_string(&script),
            "    SetU32       Pool_0 0.0 Const 0.0
    StartDialog 
    SpeakerName  \"Alice\"
    SetDialog    \"[0xEA]BAD APPLE\"
    SetU32       Pool_0 0.0 Const 1.0
    ShowDialog  
    EndDialog   
"
        );
    }

    #[test]
    fn test_write_instructions_with_ascii_string() {
        let opcodes = Opcode::get();
        let script = read_evt_from_file_and_disassemble("test/data/load_pos.evt", &opcodes);

        assert_eq!(
            instructions_as_string(&script),
            "    LoadPos      \"demo001.pos\"\n"
        );
    }

    #[test]
    fn test_write_instructions_with_bytes() {
        let opcodes = Opcode::get();
        let script = read_evt_from_file_and_disassemble("test/data/nopaa_bytes.evt", &opcodes);

        assert_eq!(
            instructions_as_string(&script),
            r#"    NopAA        b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c"
"#
        );
    }

    #[test]
    fn test_write_instructions_start_event() {
        let opcodes = Opcode::get();
        let script = read_evt_from_file_and_disassemble("test/data/start_event.evt", &opcodes);

        assert_eq!(
            instructions_as_string(&script),
            r#"    Nop0        
    StartEvent   l36 1.0
    Exit         1.0
  l36:
    Nop0        
    Exit         1.0
"#
        );
    }

    #[test]
    fn test_write_instructions_jump_if() {
        let opcodes = Opcode::get();
        let script = read_evt_from_file_and_disassemble("test/data/jump_if.evt", &opcodes);

        assert_eq!(
            instructions_as_string(&script),
            r#"    SetU32       Pool_0 0.0 Const 42.0
    FloatsEq     Pool_0 0.0 Const 6.0
    JumpIfFalse  l72
    Exit         1.0
  l72:
    Nop0        
    Exit         1.0
"#
        );
    }

    #[test]
    fn test_from_evt_no_instructions() {
        let opcodes = Opcode::get();
        let actual = read_evt_from_file_and_disassemble("test/data/no_instructions.evt", &opcodes);

        let expected = DisassembledEvt {
            data: [0xFFu8; 0x1000],
            instructions: vec![],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_evt_only_exit() {
        let opcodes = Opcode::get();
        let actual = read_evt_from_file_and_disassemble("test/data/only_exit.evt", &opcodes);

        let expected = DisassembledEvt {
            data: [0xFFu8; 0x1000],
            instructions: vec![(
                Some("l0".to_string()),
                DecodedInstruction {
                    opcode: &opcodes[EXIT as usize],
                    args: vec![Arg::Float(0.0)],
                    label: None,
                },
            )],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_evt_jump_to_self() {
        let opcodes = Opcode::get();
        let actual = read_evt_from_file_and_disassemble("test/data/jump_to_self.evt", &opcodes);

        let expected = DisassembledEvt {
            data: [0xFFu8; 0x1000],
            instructions: vec![(
                Some("l0".to_string()),
                DecodedInstruction {
                    opcode: &opcodes[JUMP as usize],
                    args: vec![Arg::JumpDestination("l0".to_string())],
                    label: Some("l0".to_string()),
                },
            )],
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_evt_dialog() {
        use self::ValueLocation::*;
        use Arg::*;

        let opcodes = Opcode::get();
        let actual = read_evt_from_file_and_disassemble("test/data/dialog.evt", &opcodes);

        let expected = DisassembledEvt {
            data: [0xFFu8; 0x1000],
            instructions: vec![
                (
                    Some("l0".to_string()),
                    DecodedInstruction {
                        opcode: &opcodes[SETU32 as usize],
                        args: vec![
                            ValueLocation(Pool0),
                            Float(0.0),
                            ValueLocation(Constant),
                            Float(0.0),
                        ],
                        label: None,
                    },
                ),
                (
                    Some("l24".to_string()),
                    DecodedInstruction {
                        opcode: &opcodes[START_DIALOG as usize],
                        args: vec![],
                        label: None,
                    },
                ),
                (
                    Some("l32".to_string()),
                    DecodedInstruction {
                        opcode: &opcodes[SPEAKER_NAME as usize],
                        args: vec![StringLit("Alice".to_string())],
                        label: None,
                    },
                ),
                (
                    Some("l48".to_string()),
                    DecodedInstruction {
                        opcode: &opcodes[SET_DIALOG as usize],
                        args: vec![StringLit("[0xEA]BAD APPLE".to_string())],
                        label: None,
                    },
                ),
                (
                    Some("l68".to_string()),
                    DecodedInstruction {
                        opcode: &opcodes[SETU32 as usize],
                        args: vec![
                            ValueLocation(Pool0),
                            Float(0.0),
                            ValueLocation(Constant),
                            Float(1.0),
                        ],
                        label: None,
                    },
                ),
                (
                    Some("l92".to_string()),
                    DecodedInstruction {
                        opcode: &opcodes[SHOW_DIALOG as usize],
                        args: vec![],
                        label: None,
                    },
                ),
                (
                    Some("l100".to_string()),
                    DecodedInstruction {
                        opcode: &opcodes[END_DIALOG as usize],
                        args: vec![],
                        label: None,
                    },
                ),
            ],
        };

        assert_eq!(actual.instructions, expected.instructions);
        //assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("test/data/no_instructions.evt")]
    #[case("test/data/only_exit.evt")]
    #[case("test/data/load_pos.evt")]
    #[case("test/data/nopaa_bytes.evt")]
    #[case("test/data/dialog.evt")]
    #[case("test/data/jump_to_self.evt")]
    #[case("test/data/jump_if.evt")]
    #[case("test/data/start_event.evt")]
    fn test_decode_encode(#[case] filepath: &str) {
        let evt = read_evt_from_file(filepath);

        let opcodes = Opcode::get();
        let character_encoding = CharacterEncoding::get(Region::NorthAmerica);

        let decoded = DisassembledEvt::from_evt(&evt, &character_encoding, &opcodes);
        let encoded = decoded.to_evt(&character_encoding);

        assert_eq!(encoded.data, evt.data);
        assert_eq!(encoded.instructions, evt.instructions);
    }
}
