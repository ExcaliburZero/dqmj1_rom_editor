use chumsky::prelude::*;
use logos::Logos;

use crate::dqmj1_rom::events::disassembly::{
    Arg, DecodedInstruction, DisassembledEvt, Opcode, ValueLocation,
};

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r]+")] // skip whitespace
pub enum AssemblyToken {
    #[token(".data:")]
    DataSection,

    #[token(".code:")]
    CodeSection,

    #[token(":")]
    Colon,

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<u32>().ok())]
    Int(u32),

    #[regex(r"-?[0-9]+\.[0-9]+(?:e[+-]?[0-9]+)?", |lex| lex.slice().parse::<f32>().ok())]
    Float(f32),

    #[regex(r"(Pool_0)|(Pool_1)|(Const)|(Pool3)", |lex| lex.slice().to_owned(), priority = 3)]
    ValueLocation(String),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_\.]+", |lex| lex.slice().to_owned(), priority = 2)]
    Ident(String),

    #[regex(r#""[^"]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_owned())]
    StringLit(String),

    #[token("b\"", lex_byte_string)]
    ByteString(Vec<u8>),

    #[token("\n")]
    Newline,
}

impl Eq for AssemblyToken {} // Note: Needed since Float contains an f32

fn lex_byte_string(lex: &mut logos::Lexer<AssemblyToken>) -> Option<Vec<u8>> {
    let remainder = lex.remainder();
    let mut bytes = Vec::new();
    let mut chars = remainder.char_indices();

    loop {
        match chars.next() {
            // Closing quote
            Some((i, '"')) => {
                lex.bump(i + 1);
                return Some(bytes);
            }
            // Byte (ex. \xA0)
            Some((_, '\\')) => {
                match chars.next() {
                    Some((_, 'x')) => {
                        let hi = chars.next()?.1.to_digit(16)? as u8;
                        let lo = chars.next()?.1.to_digit(16)? as u8;
                        bytes.push((hi << 4) | lo);
                    }
                    _ => return None, // unexpected escape
                }
            }
            // Unexpected end of input or bad char
            _ => return None,
        }
    }
}

pub fn parse_dqmj1_asm<'a>(contents: &str, opcodes: &'a [Opcode]) -> DisassembledEvt<'a> {
    //let tokens: Vec<_> = AssemblyToken::lexer(contents).collect();

    let tokens: Vec<AssemblyToken> = AssemblyToken::lexer(contents).map(|t| t.unwrap()).collect();
    //.filter_map(|t| t.ok())

    println!("{:?}", tokens);

    let result = get_parser(opcodes).parse(&tokens).unwrap();
    result
}

pub fn get_parser<'a, 'src>(
    opcodes: &'a [Opcode],
) -> impl Parser<'src, &'src [AssemblyToken], DisassembledEvt<'a>> {
    let int = select! { AssemblyToken::Int(int) => int };
    let float = select! { AssemblyToken::Float(float) => float };
    let ident = select! { AssemblyToken::Ident(string) => string };
    let string_lit = select! { AssemblyToken::StringLit(string) => string };
    let byte_string = select! { AssemblyToken::ByteString(bytes) => bytes };
    let value_location = select! { AssemblyToken::ValueLocation(string) => ValueLocation::from_asm_string(&string).unwrap() };

    let label = choice((
        select! { AssemblyToken::Ident(string) => string },
        select! { AssemblyToken::Int(int) => int.to_string() },
    ))
    .then(just(AssemblyToken::Colon))
    .then(just(AssemblyToken::Newline))
    .map(|((label, _), _)| label);

    let newline = just(AssemblyToken::Newline);
    let zero_or_more_newlines = newline.clone().repeated();
    let one_or_more_newlines = newline.clone().repeated().at_least(1);

    let argument = choice((
        int.map(|int| Arg::Float(int as f32)),
        float.map(Arg::Float),
        ident.map(Arg::JumpDestination),
        string_lit.map(Arg::StringLit),
        byte_string.map(Arg::Bytes),
        value_location.map(Arg::ValueLocation),
    ));

    let instruction = label.or_not().then(
        ident
            .then(argument.repeated().collect::<Vec<_>>())
            .map(|(name, args)| parse_instruction(opcodes, &name, args)),
    );

    let data_section = just(AssemblyToken::DataSection)
        .then(one_or_more_newlines.clone().then(byte_string))
        .map(|(_, (_, data))| parse_data_section(data));
    let code_section = just(AssemblyToken::CodeSection)
        .then(
            one_or_more_newlines
                .clone()
                .then(instruction)
                .repeated()
                .collect::<Vec<_>>(),
        )
        .then(zero_or_more_newlines.clone())
        .map(|((_, entries), _)| {
            entries
                .iter()
                .map(|(_, (label, instruction))| {
                    let mut new_instruction = instruction.clone();
                    new_instruction.label = label.clone();
                    (label.clone(), new_instruction)
                })
                .collect::<Vec<_>>()
        });

    zero_or_more_newlines
        .then(data_section.then(one_or_more_newlines.clone().then(code_section)))
        .map(|(_, (data, (_, code)))| DisassembledEvt {
            data,
            instructions: code,
        })
}

fn parse_instruction<'a>(
    opcodes: &'a [Opcode],
    name: &str,
    args: Vec<Arg>,
) -> DecodedInstruction<'a> {
    let mut matching_opcode = None;
    for opcode in opcodes.iter() {
        if opcode.name == name {
            matching_opcode = Some(opcode);
            break;
        }
    }

    DecodedInstruction {
        opcode: matching_opcode.unwrap(),
        args,
        label: None,
    }
}

fn parse_data_section(data: Vec<u8>) -> [u8; 0x1000] {
    data.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::dqmj1_rom::events::assembly::AssemblyToken;
    use crate::dqmj1_rom::events::assembly::{parse_dqmj1_asm, AssemblyToken::*};
    use crate::dqmj1_rom::events::disassembly::{
        Arg, DecodedInstruction, DisassembledEvt, Opcode, ValueLocation,
    };

    const EXIT: u8 = 0x02;
    const JUMP: u8 = 0x0C;
    const SETU32: u8 = 0x15;
    const START_DIALOG: u8 = 0x25;
    const END_DIALOG: u8 = 0x26;
    const SHOW_DIALOG: u8 = 0x27;
    const SET_DIALOG: u8 = 0x29;
    const SPEAKER_NAME: u8 = 0x2A;
    const LOAD_POS: u8 = 0x99;

    fn parse_dqmj1_asm_for_test<'a>(filepath: &str, opcodes: &'a [Opcode]) -> DisassembledEvt<'a> {
        let contents = std::fs::read_to_string(filepath).unwrap();

        parse_dqmj1_asm(&contents, opcodes)
    }

    #[rstest]
    #[case(".data:", vec![DataSection])]
    #[case(".code:", vec![CodeSection])]
    #[case("1.0", vec![Float(1.0)])]
    #[case("1", vec![Int(1)])]
    #[case("abCD_ef", vec![Ident("abCD_ef".to_string())])]
    #[case("\"abCD_ef... !!!\"", vec![StringLit("abCD_ef... !!!".to_string())])]
    #[case(r#"b"""#, vec![ByteString(vec![])])]
    #[case(r#"b"\x00""#, vec![ByteString(vec![0x00])])]
    #[case(r#"b"\xa1""#, vec![ByteString(vec![0xA1])])]
    #[case(r#"b"\x00\xA1""#, vec![ByteString(vec![0x00, 0xA1])])]
    #[case("FloatsEq     Pool_0 8.0 Const 5.0", vec![
        Ident("FloatsEq".to_string()),
        ValueLocation("Pool_0".to_string()),
        Float(8.0),
        ValueLocation("Const".to_string()),
        Float(5.0)
    ])]
    #[case("100:", vec![Int(100), Colon])]
    fn test_assembly_token_lexing(#[case] string: &str, #[case] expected: Vec<AssemblyToken>) {
        use logos::Logos;

        let tokens: Vec<AssemblyToken> = AssemblyToken::lexer(string)
            .filter_map(|token| token.ok())
            .collect();

        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_parse_dqmj1_asm_no_instructions() {
        let opcodes = Opcode::get();
        let actual = parse_dqmj1_asm_for_test("test/data/no_instructions.dqmj1_asm", &opcodes);

        let expected = vec![];

        assert_eq!(actual.data, [0x00; 0x1000]);
        assert_eq!(actual.instructions, expected);
    }

    #[test]
    fn test_parse_dqmj1_asm_single_instruction() {
        let opcodes = Opcode::get();
        let actual = parse_dqmj1_asm_for_test("test/data/only_exit.dqmj1_asm", &opcodes);

        let expected = vec![(
            None,
            DecodedInstruction {
                opcode: &opcodes[EXIT as usize],
                args: vec![Arg::Float(0.0)],
                label: None,
            },
        )];

        assert_eq!(actual.data, [0x00; 0x1000]);
        assert_eq!(actual.instructions, expected);
    }

    #[test]
    fn test_parse_dqmj1_asm_with_label() {
        let opcodes = Opcode::get();
        let actual = parse_dqmj1_asm_for_test("test/data/jump_to_self.dqmj1_asm", &opcodes);

        let expected = vec![(
            Some("l0".to_string()),
            DecodedInstruction {
                opcode: &opcodes[JUMP as usize],
                args: vec![Arg::JumpDestination("l0".to_string())],
                label: Some("l0".to_string()),
            },
        )];

        assert_eq!(actual.data, [0x00; 0x1000]);
        assert_eq!(actual.instructions, expected);
    }

    #[test]
    fn test_parse_dqmj1_asm_with_ascii_string_literal() {
        let opcodes = Opcode::get();
        let actual = parse_dqmj1_asm_for_test("test/data/load_pos.dqmj1_asm", &opcodes);

        let expected = vec![(
            None,
            DecodedInstruction {
                opcode: &opcodes[LOAD_POS as usize],
                args: vec![Arg::StringLit("demo001.pos".to_string())],
                label: None,
            },
        )];

        assert_eq!(actual.data, [0x00; 0x1000]);
        assert_eq!(actual.instructions, expected);
    }

    #[test]
    fn test_parse_dqmj1_asm_dialog() {
        let opcodes = Opcode::get();
        let actual = parse_dqmj1_asm_for_test("test/data/dialog.dqmj1_asm", &opcodes);

        let expected = vec![
            (
                None,
                DecodedInstruction {
                    opcode: &opcodes[SETU32 as usize],
                    args: vec![
                        Arg::ValueLocation(ValueLocation::Pool0),
                        Arg::Float(0.0),
                        Arg::ValueLocation(ValueLocation::Constant),
                        Arg::Float(0.0),
                    ],
                    label: None,
                },
            ),
            (
                None,
                DecodedInstruction {
                    opcode: &opcodes[START_DIALOG as usize],
                    args: vec![],
                    label: None,
                },
            ),
            (
                None,
                DecodedInstruction {
                    opcode: &opcodes[SPEAKER_NAME as usize],
                    args: vec![Arg::StringLit("Alice".to_string())],
                    label: None,
                },
            ),
            (
                None,
                DecodedInstruction {
                    opcode: &opcodes[SET_DIALOG as usize],
                    args: vec![Arg::StringLit("[0xEA]BAD APPLE".to_string())],
                    label: None,
                },
            ),
            (
                None,
                DecodedInstruction {
                    opcode: &opcodes[SETU32 as usize],
                    args: vec![
                        Arg::ValueLocation(ValueLocation::Pool0),
                        Arg::Float(0.0),
                        Arg::ValueLocation(ValueLocation::Constant),
                        Arg::Float(1.0),
                    ],
                    label: None,
                },
            ),
            (
                None,
                DecodedInstruction {
                    opcode: &opcodes[SHOW_DIALOG as usize],
                    args: vec![],
                    label: None,
                },
            ),
            (
                None,
                DecodedInstruction {
                    opcode: &opcodes[END_DIALOG as usize],
                    args: vec![],
                    label: None,
                },
            ),
        ];

        assert_eq!(actual.data, [0x00; 0x1000]);
        assert_eq!(actual.instructions, expected);
    }

    #[test]
    fn test_parse_dqmj1_asm_unnecessary_newlines() {
        let opcodes = Opcode::get();
        let actual = parse_dqmj1_asm_for_test("test/data/unnecessary_newlines.dqmj1_asm", &opcodes);

        let expected = vec![(
            None,
            DecodedInstruction {
                opcode: &opcodes[EXIT as usize],
                args: vec![Arg::Float(0.0)],
                label: None,
            },
        )];

        assert_eq!(actual.data, [0x00; 0x1000]);
        assert_eq!(actual.instructions, expected);
    }
}
