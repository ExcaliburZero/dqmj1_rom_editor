use binrw::binrw;
use serde::Deserialize;

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Instruction {
    pub opcode: u32,
    pub length: u32,

    #[br(count = length - 8)]
    pub arguments: Vec<u8>,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Evt {
    pub magic: u32,
    pub data: [u8; 0x1000],

    #[br(parse_with = binrw::helpers::until_eof)]
    pub instructions: Vec<Instruction>,
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
