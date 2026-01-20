use super::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn peek(&self) -> Token {
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

    pub fn parse_expression(&mut self) -> Result<f64, &'static str> {
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
                Token::RParen | Token::EOF => break, // 괄호 안이나 표현식 끝에서 종료
                _ => return Err("Unexpected token in expression"),
            }
        }

        Ok(result)
    }

    // term: power (('*' | '/' | '%') power)*
    fn parse_term(&mut self) -> Result<f64, &'static str> {
        let mut result = self.parse_power()?;

        loop {
            match self.peek() {
                Token::Multiply => {
                    self.advance();
                    result *= self.parse_power()?;
                }
                Token::Divide => {
                    self.advance();
                    let divisor = self.parse_power()?;
                    if divisor == 0.0 {
                        return Err("Cannot divide by ZERO");
                    }
                    result /= divisor;
                }
                Token::Modulo => {
                    self.advance();
                    let divisor = self.parse_power()?;
                    if divisor == 0.0 {
                        return Err("Cannot divide by ZERO");
                    }
                    result %= divisor;
                }
                _ => break,
            }
        }

        Ok(result)
    }

    // power: factor ('^' power)* (우측 결합)
    fn parse_power(&mut self) -> Result<f64, &'static str> {
        let mut result = self.parse_factor()?;

        if matches!(self.peek(), Token::Power) {
            self.advance(); // consume '^'
            let exponent = self.parse_power()?; // 우측 결합이므로 재귀 호출
            result = result.powf(exponent);
        }

        Ok(result)
    }

    // factor: number | '-' factor | '+' factor | function '(' expression ')' | '(' expression ')'
    fn parse_factor(&mut self) -> Result<f64, &'static str> {
        match self.peek() {
            Token::Number(n) => {
                self.advance();
                Ok(n)
            }
            Token::Minus => {
                // 단항 마이너스 처리
                self.advance(); // consume '-'
                Ok(-self.parse_factor()?)
            }
            Token::Plus => {
                // 단항 플러스 처리 (선택적)
                self.advance(); // consume '+'
                self.parse_factor()
            }
            Token::Sqrt => {
                // sqrt 함수 처리
                self.advance(); // consume 'sqrt'
                match self.peek() {
                    Token::LParen => {
                        self.advance(); // consume '('
                        let arg = self.parse_expression()?;
                        match self.peek() {
                            Token::RParen => {
                                self.advance(); // consume ')'
                                if arg < 0.0 {
                                    return Err("Cannot take square root of negative number");
                                }
                                Ok(arg.sqrt())
                            }
                            Token::EOF => Err("Unclosed parenthesis: expected ')'"),
                            _ => Err("Expected ')' after sqrt argument"),
                        }
                    }
                    _ => Err("Expected '(' after sqrt"),
                }
            }
            Token::LParen => {
                self.advance(); // consume '('
                let result = self.parse_expression()?;
                match self.peek() {
                    Token::RParen => {
                        self.advance(); // consume ')'
                        Ok(result)
                    }
                    Token::EOF => Err("Unclosed parenthesis: expected ')'"),
                    _ => Err("Expected ')' after expression"),
                }
            }
            Token::RParen => Err("Unexpected ')' - no matching '('"),
            Token::EOF => Err("Unexpected end of expression"),
            _ => Err("Expected number, '-', '+', 'sqrt', or '('"),
        }
    }
}
