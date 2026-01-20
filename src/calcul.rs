#[derive(Debug, Clone, Copy)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    EOF,
}

pub fn evl_ex(expression: &str) -> Result<f64, &'static str> {
    let tokens = tokenize(expression)?;
    let mut parser = Parser::new(tokens);
    parser.parse_expression()
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).copied().unwrap_or(Token::EOF)
    }

    fn advance(&mut self) -> Token {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current];
            self.current += 1;
            token
        } else {
            Token::EOF
        }
    }

    // expression: term (('+' | '-') term)*
    fn parse_expression(&mut self) -> Result<f64, &'static str> {
        let mut result = self.parse_term()?;

        loop {
            match self.peek() {
                Token::Plus => {
                    self.advance();
                    result += self.parse_term()?;
                }
                Token::Minus => {
                    self.advance();
                    result -= self.parse_term()?;
                }
                _ => break,
            }
        }

        if !matches!(self.peek(), Token::EOF) {
            return Err("Unexpected token after expression");
        }

        Ok(result)
    }

    // term: factor (('*' | '/') factor)*
    fn parse_term(&mut self) -> Result<f64, &'static str> {
        let mut result = self.parse_factor()?;

        loop {
            match self.peek() {
                Token::Multiply => {
                    self.advance();
                    result *= self.parse_factor()?;
                }
                Token::Divide => {
                    self.advance();
                    let divisor = self.parse_factor()?;
                    if divisor == 0.0 {
                        return Err("Cannot divide by ZERO");
                    }
                    result /= divisor;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    // factor: number | '(' expression ')'
    fn parse_factor(&mut self) -> Result<f64, &'static str> {
        match self.peek() {
            Token::Number(n) => {
                self.advance();
                Ok(n)
            }
            Token::LParen => {
                self.advance(); // consume '('
                let result = self.parse_expression()?;
                match self.peek() {
                    Token::RParen => {
                        self.advance(); // consume ')'
                        Ok(result)
                    }
                    _ => Err("Expected ')' after expression"),
                }
            }
            _ => Err("Expected number or '('"),
        }
    }
}

fn tokenize(expression: &str) -> Result<Vec<Token>, &'static str> {
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
