use std::fmt;

use chumsky::{error::RichReason, prelude::*};

use crate::events::{
    assembly::lexer::{lex_dqmj1_asm, AssemblyToken, LexError, Position},
    disassembly::{Arg, DecodedInstruction, DisassembledEvt, Opcode, ValueLocation},
};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
        }
    }

    pub fn from_rich(
        tokens_with_position: &[(AssemblyToken, Position)],
        rich: &Rich<'_, AssemblyToken>,
    ) -> ParseError {
        let message = match rich.reason() {
            RichReason::ExpectedFound { expected, found } => {
                format!("expected: {:?}, found: {:?}", expected, found)
            }
            RichReason::Custom(msg) => msg.to_string(),
        };

        ParseError {
            message: format!(
                "{} at {}",
                message,
                Position::from_token_span(tokens_with_position, rich.span().start)
            ),
        }
    }
}

#[derive(Debug)]
pub enum ParseLexError {
    Lex(LexError),
    Parse(ParseError),
}

impl fmt::Display for ParseLexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let full_message = match self {
            ParseLexError::Lex(error) => format!("{}", error),
            ParseLexError::Parse(error) => error.message.to_string(),
        };

        write!(f, "{}", full_message)
    }
}

pub type ParseLexErrors = Vec<ParseLexError>;

pub fn parse_dqmj1_asm<'a>(
    contents: &str,
    opcodes: &'a [Opcode],
) -> Result<DisassembledEvt<'a>, ParseLexErrors> {
    let mut errors: ParseLexErrors = vec![];

    let (tokens_with_position, lex_errors) = lex_dqmj1_asm(contents);
    for lex_error in lex_errors {
        errors.push(ParseLexError::Lex(lex_error))
    }

    let tokens: Vec<AssemblyToken> = tokens_with_position
        .iter()
        .map(|(token, _)| token)
        .cloned()
        .collect();

    let result = get_parser(opcodes).parse(&tokens);

    for error in result.errors() {
        errors.push(ParseLexError::Parse(ParseError::from_rich(
            &tokens_with_position,
            error,
        )));
    }

    if errors.is_empty() {
        Ok(result.unwrap())
    } else {
        Err(errors)
    }
}

pub fn get_parser<'a, 'src>(
    opcodes: &'a [Opcode],
) -> impl Parser<'src, &'src [AssemblyToken], DisassembledEvt<'a>, extra::Err<Rich<'src, AssemblyToken>>>
{
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
    .then_ignore(just(AssemblyToken::Colon))
    .then_ignore(just(AssemblyToken::Newline));

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
    let arguments = argument.repeated().collect::<Vec<_>>();

    let instruction = ident
        .then(arguments)
        .validate(
            |(name, args), extra, emitter| match parse_instruction(opcodes, &name, args) {
                Ok(inst) => inst,
                Err(e) => {
                    emitter.emit(chumsky::error::Rich::custom(extra.span(), e));
                    DecodedInstruction::dummy(opcodes)
                }
            },
        );

    let instruction_statement = label.or_not().then(instruction);

    let data_section = just(AssemblyToken::DataSection)
        .then(one_or_more_newlines.clone().then(byte_string))
        .try_map(|(_, (_, data)), span| {
            parse_data_section(data).map_err(|e| chumsky::error::Rich::custom(span, e))
        });
    let code_section = just(AssemblyToken::CodeSection)
        .then(
            one_or_more_newlines
                .clone()
                .then(instruction_statement)
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
) -> Result<DecodedInstruction<'a>, String> {
    let mut matching_opcode = None;
    for opcode in opcodes.iter() {
        if opcode.name == name {
            matching_opcode = Some(opcode);
            break;
        }
    }

    if matching_opcode.is_none() {
        return Err(format!("Unrecognized instruction name: \"{}\"", name));
    }

    Ok(DecodedInstruction {
        opcode: matching_opcode.unwrap(),
        args,
        label: None,
    })
}

fn parse_data_section(data: Vec<u8>) -> Result<[u8; 0x1000], String> {
    if data.len() != 0x1000 {
        return Err(format!(
            "Data section has wrong number of bytes, should be {} was {}",
            0x1000,
            data.len()
        ));
    }

    if let Ok(data) = data.try_into() {
        Ok(data)
    } else {
        Err(
            "Data section failed to convert to array of 4096 bytes due to unknown reason"
                .to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::events::assembly::lexer::AssemblyToken;
    use crate::events::assembly::lexer::AssemblyToken::*;
    use crate::events::assembly::parser::{parse_dqmj1_asm, ParseLexErrors};
    use crate::events::disassembly::{
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

    fn parse_dqmj1_asm_for_test<'a>(
        filepath: &str,
        opcodes: &'a [Opcode],
    ) -> Result<DisassembledEvt<'a>, ParseLexErrors> {
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
        let actual =
            parse_dqmj1_asm_for_test("test/data/no_instructions.dqmj1_asm", &opcodes).unwrap();

        let expected = vec![];

        assert_eq!(actual.data, [0x00; 0x1000]);
        assert_eq!(actual.instructions, expected);
    }

    #[test]
    fn test_parse_dqmj1_asm_single_instruction() {
        let opcodes = Opcode::get();
        let actual = parse_dqmj1_asm_for_test("test/data/only_exit.dqmj1_asm", &opcodes).unwrap();

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
        let actual =
            parse_dqmj1_asm_for_test("test/data/jump_to_self.dqmj1_asm", &opcodes).unwrap();

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
        let actual = parse_dqmj1_asm_for_test("test/data/load_pos.dqmj1_asm", &opcodes).unwrap();

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
        let actual = parse_dqmj1_asm_for_test("test/data/dialog.dqmj1_asm", &opcodes).unwrap();

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
        let actual =
            parse_dqmj1_asm_for_test("test/data/unnecessary_newlines.dqmj1_asm", &opcodes).unwrap();

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
