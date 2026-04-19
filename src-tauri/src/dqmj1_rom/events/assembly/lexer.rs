use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\r]+")] // skip whitespace
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::dqmj1_rom::events::assembly::lexer::AssemblyToken;
    use crate::dqmj1_rom::events::assembly::lexer::AssemblyToken::*;

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
}
