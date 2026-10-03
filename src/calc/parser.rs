use super::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    ans: Option<f64>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, ans: Option<f64>) -> Self {
        Self { tokens, current: 0, ans }
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

    // term: unary (('*' | '/' | '%') unary)*
    fn parse_term(&mut self) -> Result<f64, &'static str> {
        let mut result = self.parse_unary()?;

        loop {
            match self.peek() {
                Token::Multiply => {
                    self.advance();
                    result *= self.parse_unary()?;
                }
                Token::Divide => {
                    self.advance();
                    let divisor = self.parse_unary()?;
                    if divisor == 0.0 {
                        return Err("Cannot divide by ZERO");
                    }
                    result /= divisor;
                }
                Token::Modulo => {
                    self.advance();
                    let divisor = self.parse_unary()?;
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

    // unary: ('-' | '+') unary | power
    // 단항 연산자는 '^'보다 우선순위가 낮음: -2^2 = -(2^2)
    fn parse_unary(&mut self) -> Result<f64, &'static str> {
        match self.peek() {
            Token::Minus => {
                self.advance(); // consume '-'
                Ok(-self.parse_unary()?)
            }
            Token::Plus => {
                self.advance(); // consume '+'
                self.parse_unary()
            }
            _ => self.parse_power(),
        }
    }

    // power: factor ('^' unary)? (우측 결합, 지수에 단항 연산자 허용: 2^-1)
    fn parse_power(&mut self) -> Result<f64, &'static str> {
        let mut result = self.parse_factor()?;

        if matches!(self.peek(), Token::Power) {
            self.advance(); // consume '^'
            let exponent = self.parse_unary()?; // 우측 결합이므로 재귀 호출
            result = result.powf(exponent);
        }

        Ok(result)
    }

    // factor: number | 'ans' | function '(' expression ')' | '(' expression ')'
    fn parse_factor(&mut self) -> Result<f64, &'static str> {
        match self.peek() {
            Token::Number(n) => {
                self.advance();
                Ok(n)
            }
            Token::Ans => {
                self.advance();
                self.ans.ok_or("No previous result for 'ans'")
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
            _ => Err("Expected number, '-', '+', 'ans', 'sqrt', or '('"),
        }
    }
}
