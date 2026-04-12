use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum AssemblyToken {
    #[token(".data:")]
    DataSection,

    #[token(".code:")]
    CodeSection,

    #[token(":")]
    Colon,

    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<u32>().ok())]
    Int(u32),

    #[regex(r"-?[0-9]+\.[0-9]+(?:e[+-]?[0-9]+)?", |lex| lex.slice().parse::<f32>().ok())]
    Float(f32),

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_\.]+", |lex| lex.slice().to_owned())]
    Ident(String),

    #[regex(r#""[^"]*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_owned())]
    StringLit(String),

    #[regex(r#"b"(\\x[0-9a-fA-F][0-9a-fA-F])*""#, |lex| {
        let inner = &lex.slice()[2..lex.slice().len()-1]; // strip b" and "
        parse_byte_string(inner).ok()
    })]
    ByteString(Vec<u8>),
}

fn parse_byte_string(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(4) {
        return Err(format!(
            "Byte string length must be evenly divisible by 4, actual length: {}",
            s.len()
        )
        .to_string());
    }

    let mut bytes = vec![];
    for chunk in s.chars().collect::<Vec<_>>().chunks(4) {
        if chunk[0..2] != ['\\', 'x'] {
            return Err(format!(
                "All bytes must start with \"\\x\", found byte that starts with: \"{}\"",
                chunk[0..2].iter().collect::<String>()
            )
            .to_string());
        }

        bytes.push(u8::from_str_radix(&chunk[2..4].iter().collect::<String>(), 16).unwrap());
    }

    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::dqmj1_rom::events::assembly::AssemblyToken::*;
    use crate::dqmj1_rom::events::assembly::{parse_byte_string, AssemblyToken};

    #[rstest]
    #[case(r#"\x00"#, vec![0x00])]
    #[case(r#"\xA1"#, vec![0xA1])]
    #[case(r#"\xa1"#, vec![0xA1])]
    #[case(r#"\xA1\x00\x12"#, vec![0xA1, 0x00, 0x12])]
    fn test_parse_byte_string(#[case] string: &str, #[case] expected: Vec<u8>) {
        let actual = parse_byte_string(string).unwrap();

        assert_eq!(actual, expected);
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
        Ident("Pool_0".to_string()),
        Float(8.0),
        Ident("Const".to_string()),
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
