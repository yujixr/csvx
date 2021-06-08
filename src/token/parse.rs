use super::*;
use std::str::FromStr;
use std::{borrow::Borrow, error::Error, f64::consts};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("\"{0}\" has prohibited characters")]
    ProhibitedCharacters(String),
}

pub fn parse(primitive_tokens: Vec<PrimitiveToken>) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens = vec![];

    for token in primitive_tokens {
        tokens.push(match token {
            PrimitiveToken::Parsed(token) => token,
            PrimitiveToken::Raw(raw_string) => match raw_string.as_str() {
                "true" => Token::Boolean(true),
                "false" => Token::Boolean(false),
                "inf" => Token::Float(f64::INFINITY),
                "NaN" => Token::Float(f64::NAN),
                "e" => Token::Float(consts::E),
                "pi" => Token::Float(consts::PI),
                "tau" => Token::Float(consts::TAU),
                "ref" => Token::FnRef,
                "if" => Token::FnIf,
                "round" => Token::FnRound,
                "floor" => Token::FnFloor,
                "ceil" => Token::FnCeil,
                "log" => Token::FnLog,
                "ln" => Token::FnLn,
                "log2" => Token::FnLog2,
                "log10" => Token::FnLog10,
                "sqrt" => Token::FnSqrt,
                "pow" => Token::FnPow,
                "sin" => Token::FnSin,
                "cos" => Token::FnCos,
                "tan" => Token::FnTan,
                "asin" => Token::FnAsin,
                "acos" => Token::FnAcos,
                "atan" => Token::FnAtan,
                "sinh" => Token::FnSinh,
                "cosh" => Token::FnCosh,
                "tanh" => Token::FnTanh,
                "asinh" => Token::FnAsinh,
                "acosh" => Token::FnAcosh,
                "atanh" => Token::FnAtanh,
                raw_string => {
                    if raw_string.contains('.') {
                        Token::Float(f64::from_str(raw_string)?)
                    } else {
                        let chars: Vec<char> = raw_string.chars().collect();
                        match chars[0] {
                            '0' => Token::Integer(match chars.get(1) {
                                Some('b') => i64::from_str_radix(raw_string[2..].borrow(), 2)?,
                                Some('x') => i64::from_str_radix(raw_string[2..].borrow(), 16)?,
                                Some(_) => i64::from_str_radix(raw_string, 8)?,
                                None => 0,
                            }),
                            '1'..='9' => Token::Integer(i64::from_str(raw_string)?),
                            'a'..='z' | '_' => {
                                if chars.iter().all(|char| {
                                    char.is_ascii_alphabetic() || char.is_digit(10) || *char == '_'
                                }) {
                                    Token::Var(raw_string.to_string())
                                } else {
                                    return Err(Box::new(ParseError::ProhibitedCharacters(
                                        raw_string.to_string(),
                                    )));
                                }
                            }
                            'A'..='Z' => {
                                let mut i = 1;
                                let mut x = chars[0] as usize - 'A' as usize + 1;
                                while i < raw_string.len() && matches!(chars[i], 'A'..='Z') {
                                    x = x * 26 + chars[i] as usize - 'A' as usize + 1;
                                    i += 1;
                                }

                                if i < raw_string.len() {
                                    let raw_string = chars[i..].iter().collect::<String>();
                                    if let Ok(y) = usize::from_str(raw_string.as_str()) {
                                        Token::Ref(x - 1, y - 1)
                                    } else if chars[i..].iter().all(|char| {
                                        char.is_ascii_alphabetic()
                                            || char.is_digit(10)
                                            || *char == '_'
                                    }) {
                                        Token::Var(raw_string)
                                    } else {
                                        return Err(Box::new(ParseError::ProhibitedCharacters(
                                            raw_string,
                                        )));
                                    }
                                } else {
                                    Token::Var(raw_string.to_string())
                                }
                            }
                            _ => {
                                return Err(Box::new(ParseError::ProhibitedCharacters(
                                    raw_string.to_string(),
                                )))
                            }
                        }
                    }
                }
            },
        });
    }

    Ok(tokens)
}
