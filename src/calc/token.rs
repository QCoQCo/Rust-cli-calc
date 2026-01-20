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
    LParen,
    RParen,
    EOF,
}

pub fn tokenize(expression: &str) -> Result<Vec<Token>, &'static str> {
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
            let num = num_str.parse::<f64>().map_err(|_| "Invalid number")?;
            tokens.push(Token::Number(num));
            has_number = true;
        } else if ch.is_ascii_alphabetic() {
            // 함수 이름 파싱 (예: sqrt)
            let func_name = parse_identifier(&mut chars)?;
            match func_name.as_str() {
                "sqrt" => tokens.push(Token::Sqrt),
                _ => return Err("Unknown function"),
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
                _ => return Err("Invalid character in expression"),
            }
        }
    }

    if !has_number {
        return Err("Expression must contain at least one number");
    }

    Ok(tokens)
}

fn parse_identifier<I>(chars: &mut std::iter::Peekable<I>) -> Result<String, &'static str>
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
    if ident.is_empty() {
        Err("Expected identifier")
    } else {
        Ok(ident)
    }
}

fn parse_number<I>(chars: &mut std::iter::Peekable<I>) -> Result<String, &'static str>
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

    if num_str.is_empty() {
        Err("Expected number")
    } else if num_str.ends_with('.') {
        Err("Number cannot end with '.'")
    } else {
        Ok(num_str)
    }
}
