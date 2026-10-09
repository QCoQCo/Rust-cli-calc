#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Sqrt,
    Ans,
    LParen,
    RParen,
    Eof,
}

use super::error::CalcError;

pub fn tokenize(expression: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let mut chars = expression.chars().peekable();
    let mut has_number = false;

    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
            continue;
        }

        if ch.is_ascii_digit() || ch == '.' {
            let num_str = parse_number(&mut chars)?;
            let num = num_str.parse::<f64>().map_err(|_| CalcError::InvalidNumber(num_str.clone()))?;
            tokens.push(Token::Number(num));
            has_number = true;
        } else if ch.is_ascii_alphabetic() {
            // 식별자 파싱 (예: sqrt, ans)
            let ident = parse_identifier(&mut chars);
            match ident.as_str() {
                "sqrt" => tokens.push(Token::Sqrt),
                "ans" => {
                    tokens.push(Token::Ans);
                    has_number = true;
                }
                _ => return Err(CalcError::UnknownFunction(ident)),
            }
        } else {
            match ch {
                '+' => {
                    tokens.push(Token::Plus);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    chars.next();
                }
                '%' => {
                    tokens.push(Token::Modulo);
                    chars.next();
                }
                '^' => {
                    tokens.push(Token::Power);
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    chars.next();
                }
                _ => return Err(CalcError::InvalidCharacter(ch)),
            }
        }
    }

    if !has_number {
        return Err(CalcError::NoNumber);
    }

    Ok(tokens)
}

/// 표현식이 `ans`를 참조하는지 확인
pub fn uses_ans(expression: &str) -> bool {
    tokenize(expression).is_ok_and(|tokens| tokens.iter().any(|t| matches!(t, Token::Ans)))
}

// 호출 전에 첫 글자가 알파벳임을 확인하므로 결과는 비어 있지 않음
fn parse_identifier<I>(chars: &mut std::iter::Peekable<I>) -> String
where
    I: Iterator<Item = char>,
{
    let mut ident = String::new();
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            ident.push(ch);
            chars.next();
        } else {
            break;
        }
    }
    ident
}

// 호출 전에 첫 글자가 숫자나 '.'임을 확인하므로 결과는 비어 있지 않음
fn parse_number<I>(chars: &mut std::iter::Peekable<I>) -> Result<String, CalcError>
where
    I: Iterator<Item = char>,
{
    let mut num_str = String::new();
    let mut has_dot = false;

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            num_str.push(ch);
            chars.next();
        } else if ch == '.' && !has_dot {
            num_str.push(ch);
            has_dot = true;
            chars.next();
        } else {
            break;
        }
    }

    if num_str.ends_with('.') {
        Err(CalcError::TrailingDot)
    } else {
        Ok(num_str)
    }
}

#[cfg(test)]
mod tests {
    use super::{tokenize, uses_ans, Token};
    use crate::calc::error::CalcError;

    #[test]
    fn numbers() {
        let tokens = tokenize("12.5 + .5").unwrap();
        assert!(matches!(
            tokens.as_slice(),
            [Token::Number(a), Token::Plus, Token::Number(b)] if *a == 12.5 && *b == 0.5
        ));
    }

    #[test]
    fn operators_and_functions() {
        let tokens = tokenize("sqrt(4) - 2 * 3 / 1 % 2 ^ 2").unwrap();
        assert!(matches!(
            tokens.as_slice(),
            [
                Token::Sqrt,
                Token::LParen,
                Token::Number(_),
                Token::RParen,
                Token::Minus,
                Token::Number(_),
                Token::Multiply,
                Token::Number(_),
                Token::Divide,
                Token::Number(_),
                Token::Modulo,
                Token::Number(_),
                Token::Power,
                Token::Number(_),
            ]
        ));
    }

    #[test]
    fn invalid_input() {
        assert_eq!(tokenize("1.").err(), Some(CalcError::TrailingDot));
        assert_eq!(tokenize(".").err(), Some(CalcError::TrailingDot));
        assert_eq!(tokenize("1 & 2").err(), Some(CalcError::InvalidCharacter('&')));
        assert_eq!(tokenize("foo(1)").err(), Some(CalcError::UnknownFunction("foo".to_string())));
        assert_eq!(tokenize("()").err(), Some(CalcError::NoNumber));
    }

    #[test]
    fn detects_ans() {
        assert!(uses_ans("ans * 2"));
        assert!(uses_ans("sqrt(ans)"));
        assert!(!uses_ans("2 * 3"));
        assert!(!uses_ans("answer"));
    }
}
