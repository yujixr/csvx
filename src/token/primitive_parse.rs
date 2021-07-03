use super::*;
use std::borrow::Borrow;

pub fn primitive_parse<T: Borrow<str>>(raw_string: &T) -> Result<Vec<PrimitiveToken>, ()> {
    let mut tokens = vec![];

    let chars: Vec<char> = raw_string.borrow().chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' => {}
            '\"' => {
                let mut chars_pending = vec![];
                let mut is_being_escaped = false;

                loop {
                    i += 1;
                    if i >= chars.len() {
                        return Err(());
                    }

                    if is_being_escaped {
                        match chars[i] {
                            'n' => chars_pending.push('\n'),
                            'r' => chars_pending.push('\r'),
                            't' => chars_pending.push('\t'),
                            '\\' => chars_pending.push('\\'),
                            '\0' => chars_pending.push('\0'),
                            '\'' => chars_pending.push('\''),
                            '\"' => chars_pending.push('\"'),
                            _ => chars_pending.push(chars[i]),
                        }
                        is_being_escaped = false;
                    } else {
                        match chars[i] {
                            '\"' => break,
                            '\\' => is_being_escaped = true,
                            _ => chars_pending.push(chars[i]),
                        }
                    }
                }

                tokens.push(PrimitiveToken::Parsed(Token::String(
                    chars_pending.iter().collect(),
                )));
            }
            '+' => tokens.push(PrimitiveToken::Parsed(Token::Add)),
            '-' => tokens.push(PrimitiveToken::Parsed(Token::Sub)),
            '*' => tokens.push(PrimitiveToken::Parsed(Token::Mul)),
            '/' => tokens.push(PrimitiveToken::Parsed(Token::Div)),
            '%' => tokens.push(PrimitiveToken::Parsed(Token::Mod)),
            '^' => tokens.push(PrimitiveToken::Parsed(Token::Xor)),
            '&' => {
                if i + 1 < chars.len() && chars[i + 1] == '&' {
                    i += 1;
                    tokens.push(PrimitiveToken::Parsed(Token::LogicalAnd));
                } else {
                    tokens.push(PrimitiveToken::Parsed(Token::BitwiseAnd));
                }
            }
            '|' => {
                if i + 1 < chars.len() && chars[i + 1] == '|' {
                    i += 1;
                    tokens.push(PrimitiveToken::Parsed(Token::LogicalOr));
                } else {
                    tokens.push(PrimitiveToken::Parsed(Token::BitwiseOr));
                }
            }
            '=' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    i += 1;
                    tokens.push(PrimitiveToken::Parsed(Token::Equal));
                } else {
                    return Err(());
                }
            }
            '!' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    i += 1;
                    tokens.push(PrimitiveToken::Parsed(Token::NotEqual));
                } else {
                    tokens.push(PrimitiveToken::Parsed(Token::Not));
                }
            }
            '<' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    i += 1;
                    tokens.push(PrimitiveToken::Parsed(Token::LessThanEqual));
                } else if i + 1 < chars.len() && chars[i + 1] == '<' {
                    i += 1;
                    tokens.push(PrimitiveToken::Parsed(Token::LeftShift));
                } else {
                    tokens.push(PrimitiveToken::Parsed(Token::LessThan));
                }
            }
            '>' => {
                if i + 1 < chars.len() && chars[i + 1] == '=' {
                    i += 1;
                    tokens.push(PrimitiveToken::Parsed(Token::GreaterThanEqual));
                } else if i + 1 < chars.len() && chars[i + 1] == '>' {
                    i += 1;
                    tokens.push(PrimitiveToken::Parsed(Token::RightShift));
                } else {
                    tokens.push(PrimitiveToken::Parsed(Token::GreaterThan));
                }
            }
            '(' => tokens.push(PrimitiveToken::Parsed(Token::ParenthesisBegin)),
            ')' => tokens.push(PrimitiveToken::Parsed(Token::ParenthesisEnd)),
            ',' => tokens.push(PrimitiveToken::Parsed(Token::Comma)),
            _ => {
                let mut chars_pending = vec![chars[i]];

                while i + 1 < chars.len()
                    && !matches!(
                        chars[i + 1],
                        ' ' | '\t'
                            | '\"'
                            | '+'
                            | '-'
                            | '*'
                            | '/'
                            | '%'
                            | '^'
                            | '&'
                            | '|'
                            | '='
                            | '!'
                            | '<'
                            | '>'
                            | '('
                            | ')'
                            | ','
                    )
                {
                    chars_pending.push(chars[i + 1]);
                    i += 1;
                }

                tokens.push(PrimitiveToken::Raw(chars_pending.iter().collect()));
            }
        }
        i += 1;
    }

    Ok(tokens)
}
