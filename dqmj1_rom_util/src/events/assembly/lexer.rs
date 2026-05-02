use std::fmt;

use logos::{Lexer, Logos};
use thiserror::Error;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn line_and_column(line: usize, column: usize) -> Position {
        Position { line, column }
    }

    pub fn from_token_span(
        tokens_with_position: &[(AssemblyToken, Position)],
        offset: usize,
    ) -> Position {
        if tokens_with_position.is_empty() {
            Position::line_and_column(1, 1)
        } else {
            // NOTE: Not sure if this is the correct handling for cases where the lookup fails, but
            // at least it avoids the panic
            tokens_with_position
                .get(offset)
                .map(|r| r.1.clone())
                .unwrap_or(tokens_with_position.last().unwrap().1.clone())
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line={}, col={}", self.line, self.column)
    }
}

#[derive(Default, Debug, Clone)]
pub struct LexerExtras {
    pub line_0based: usize,
    pub line_start_index: usize,
}

fn newline_callback(lex: &mut Lexer<AssemblyToken>) {
    lex.extras.line_0based += 1;
    lex.extras.line_start_index = lex.span().end - 1;
}

#[derive(Error, Default, Debug, Clone, PartialEq)]
pub enum LexError {
    #[error("Unexpected char \"{1}\" at {0}")]
    UnexpectedChar(Position, char),
    #[error("Invalid bytestring \"{1}\" at {0}")]
    InvalidByteString(Position, String),
    #[default]
    #[error("Unknown lexing error")]
    Other,
}

fn get_position(lex: &Lexer<AssemblyToken>) -> Position {
    let line = lex.extras.line_0based + 1; // 1-based
    let column = lex.span().start - lex.extras.line_start_index + if line == 1 { 1 } else { 0 }; // 1-based
    Position { line, column }
}

fn error_unexpected(lex: &mut Lexer<AssemblyToken>) -> LexError {
    let position = get_position(lex);
    let char = lex.slice().chars().next().unwrap_or('?');
    LexError::UnexpectedChar(position, char)
}

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r]+")] // skip whitespace
#[logos(extras = LexerExtras)]
#[logos(error(LexError, error_unexpected))]
pub enum AssemblyToken {
    #[token(".data:")]
    DataSection,

    #[token(".code:")]
    CodeSection,

    #[token(":")]
    Colon,

    #[regex(r"[0-9]+", |lex| lex.slice().parse::<u32>().ok(), priority = 2)]
    Int(u32),

    #[regex(r"-?[0-9]+(?:\.[0-9]+)?(?:e[+-]?[0-9]+)?", |lex| lex.slice().parse::<f32>().ok(), priority = 1)]
    Float(f32),

    #[regex(r"(Pool_0)|(Pool_1)|(Const)|(Pool_3)", |lex| lex.slice().to_owned(), priority = 3)]
    ValueLocation(String),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_\.]+", |lex| lex.slice().to_owned(), priority = 2)]
    Ident(String),

    #[regex(r#""[^"]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_owned())]
    StringLit(String),

    #[token("b\"", lex_byte_string)]
    ByteString(Vec<u8>),

    #[token("\n", newline_callback)]
    Newline,

    Error,
}

impl Eq for AssemblyToken {} // Note: Needed since Float contains an f32

fn lex_byte_string(lex: &mut logos::Lexer<AssemblyToken>) -> Result<Vec<u8>, LexError> {
    let remainder = lex.remainder();
    let mut bytes = Vec::new();
    let mut chars = remainder.char_indices();

    let position = get_position(lex);

    loop {
        match chars.next() {
            // Closing quote
            Some((i, '"')) => {
                lex.bump(i + 1);
                return Ok(bytes);
            }
            // Byte (ex. \xA0)
            Some((_, '\\')) => {
                match chars.next() {
                    Some((_, 'x')) => {
                        let hi_char = chars.next();
                        let lo_char = chars.next();

                        if hi_char.is_none() || lo_char.is_none() {
                            break;
                        }

                        let hi = hi_char.unwrap().1.to_digit(16);
                        let lo = lo_char.unwrap().1.to_digit(16);

                        if hi.is_none() || lo.is_none() {
                            break;
                        }

                        let hi = hi.unwrap() as u8;
                        let lo = lo.unwrap() as u8;
                        bytes.push((hi << 4) | lo);
                    }
                    _ => {
                        break; // unexpected escape
                    }
                }
            }
            _ => {
                // Unexpected end of input or bad char
                break;
            }
        }
    }

    // Failed to parse, so find how much to bump by
    let mut chars = remainder.char_indices();
    let mut stop_i = None;
    loop {
        match chars.next() {
            // Closing quote
            Some((i, '"')) => {
                stop_i = Some(i + 1);
                lex.bump(i + 1);
                break;
            }
            Some(_) => {}
            None => {
                break;
            }
        }
    }

    let relevant_remainder: String = stop_i
        .map(|i| remainder[..i].to_string())
        .unwrap_or_else(|| remainder.to_string());

    let num_chars_to_show = 20;
    let should_truncate = relevant_remainder.chars().count() > num_chars_to_show;
    let suffix = if should_truncate { "..." } else { "" };

    Err(LexError::InvalidByteString(
        position,
        format!(
            "b\"{}",
            relevant_remainder
                .chars()
                .take(num_chars_to_show)
                .collect::<String>()
                + suffix
        ),
    ))
}

pub fn lex_dqmj1_asm(contents: &str) -> (Vec<(AssemblyToken, Position)>, Vec<LexError>) {
    let mut lexer = AssemblyToken::lexer(contents);

    let mut tokens = vec![];
    let mut errors = vec![];

    while let Some(result) = lexer.next() {
        match result {
            Ok(token) => {
                tokens.push((token, get_position(&lexer)));
            }
            Err(e) => {
                tokens.push((AssemblyToken::Error, get_position(&lexer)));
                errors.push(e);
            }
        }
    }

    (tokens, errors)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::events::assembly::lexer::{lex_dqmj1_asm, AssemblyToken, LexError};
    use crate::events::assembly::lexer::{AssemblyToken::*, Position};

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
    #[case("1\n2", vec![Int(1), Newline, Int(2)])]
    fn test_assembly_token_lexing(#[case] string: &str, #[case] expected: Vec<AssemblyToken>) {
        let actual = lex_dqmj1_asm(string);

        let tokens: Vec<AssemblyToken> = actual.0.iter().map(|(token, _)| token).cloned().collect();

        assert_eq!(tokens, expected);
        assert_eq!(actual.1, vec![]);
    }

    #[test]
    fn test_assembly_token_lexing_errors_none() {
        let contents = "";

        let actual = lex_dqmj1_asm(contents);

        assert_eq!(actual.0, vec![]);
        assert_eq!(actual.1, vec![]);
    }

    #[test]
    fn test_assembly_token_lexing_errors_unrecognized_symbol() {
        let contents = "@";

        let actual = lex_dqmj1_asm(contents);

        let expected_tokens = vec![(AssemblyToken::Error, Position::line_and_column(1, 1))];
        let expected_errors = vec![LexError::UnexpectedChar(
            Position::line_and_column(1, 1),
            '@',
        )];

        assert_eq!(actual.0, expected_tokens);
        assert_eq!(actual.1, expected_errors);
    }

    #[test]
    fn test_assembly_token_lexing_errors_unrecognized_symbol_a_few_columns_in() {
        let contents = "Pool_1 @";

        let actual = lex_dqmj1_asm(contents);

        let expected_tokens = vec![
            (
                AssemblyToken::ValueLocation("Pool_1".to_string()),
                Position::line_and_column(1, 1),
            ),
            (AssemblyToken::Error, Position::line_and_column(1, 8)),
        ];
        let expected_errors = vec![LexError::UnexpectedChar(
            Position::line_and_column(1, 8),
            '@',
        )];

        assert_eq!(actual.0, expected_tokens);
        assert_eq!(actual.1, expected_errors);
    }

    #[test]
    fn test_assembly_token_lexing_errors_unrecognized_symbol_on_second_line() {
        let contents = "Pool_1\n@";

        let actual = lex_dqmj1_asm(contents);

        let expected_tokens = vec![
            (
                AssemblyToken::ValueLocation("Pool_1".to_string()),
                Position::line_and_column(1, 1),
            ),
            (AssemblyToken::Newline, Position::line_and_column(2, 0)),
            (AssemblyToken::Error, Position::line_and_column(2, 1)),
        ];
        let expected_errors = vec![LexError::UnexpectedChar(
            Position::line_and_column(2, 1),
            '@',
        )];

        assert_eq!(actual.0, expected_tokens);
        assert_eq!(actual.1, expected_errors);
    }

    #[test]
    fn test_assembly_token_lexing_errors_unrecognized_symbol_on_second_line_and_spaces() {
        let contents = "Pool_1\n     @";

        let actual = lex_dqmj1_asm(contents);

        let expected_tokens = vec![
            (
                AssemblyToken::ValueLocation("Pool_1".to_string()),
                Position::line_and_column(1, 1),
            ),
            (AssemblyToken::Newline, Position::line_and_column(2, 0)),
            (AssemblyToken::Error, Position::line_and_column(2, 6)),
        ];
        let expected_errors = vec![LexError::UnexpectedChar(
            Position::line_and_column(2, 6),
            '@',
        )];

        assert_eq!(actual.0, expected_tokens);
        assert_eq!(actual.1, expected_errors);
    }

    #[test]
    fn test_assembly_token_lexing_errors_bad_bytestring() {
        let contents = r#"b"\" 123"#;

        let actual = lex_dqmj1_asm(contents);

        let expected_tokens = vec![
            (AssemblyToken::Error, Position::line_and_column(1, 1)),
            (AssemblyToken::Int(123), Position::line_and_column(1, 6)),
        ];
        let expected_errors = vec![LexError::InvalidByteString(
            Position::line_and_column(1, 1),
            r#"b"\""#.to_string(),
        )];

        assert_eq!(actual.0, expected_tokens);
        assert_eq!(actual.1, expected_errors);
    }
}
