use super::error::CalcError;
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
        self.tokens.get(self.current).copied().unwrap_or(Token::Eof)
    }

    fn advance(&mut self) -> Token {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current];
            self.current += 1;
            token
        } else {
            Token::Eof
        }
    }

    pub fn parse_expression(&mut self) -> Result<f64, CalcError> {
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
                Token::RParen | Token::Eof => break, // 괄호 안이나 표현식 끝에서 종료
                _ => return Err(CalcError::UnexpectedToken),
            }
        }

        Ok(result)
    }

    // term: unary (('*' | '/' | '%') unary)*
    fn parse_term(&mut self) -> Result<f64, CalcError> {
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
                        return Err(CalcError::DivisionByZero);
                    }
                    result /= divisor;
                }
                Token::Modulo => {
                    self.advance();
                    let divisor = self.parse_unary()?;
                    if divisor == 0.0 {
                        return Err(CalcError::DivisionByZero);
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
    fn parse_unary(&mut self) -> Result<f64, CalcError> {
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
    fn parse_power(&mut self) -> Result<f64, CalcError> {
        let mut result = self.parse_factor()?;

        if matches!(self.peek(), Token::Power) {
            self.advance(); // consume '^'
            let exponent = self.parse_unary()?; // 우측 결합이므로 재귀 호출
            result = result.powf(exponent);
        }

        Ok(result)
    }

    // factor: number | 'ans' | function '(' expression ')' | '(' expression ')'
    fn parse_factor(&mut self) -> Result<f64, CalcError> {
        match self.peek() {
            Token::Number(n) => {
                self.advance();
                Ok(n)
            }
            Token::Ans => {
                self.advance();
                self.ans.ok_or(CalcError::NoPreviousResult)
            }
            Token::Sqrt => {
                self.advance(); // consume 'sqrt'
                if !matches!(self.peek(), Token::LParen) {
                    return Err(CalcError::ExpectedLParenAfterSqrt);
                }
                let arg = self.parse_parenthesized()?;
                if arg < 0.0 {
                    return Err(CalcError::NegativeSqrt);
                }
                Ok(arg.sqrt())
            }
            Token::LParen => self.parse_parenthesized(),
            Token::RParen => Err(CalcError::UnmatchedRParen),
            Token::Eof => Err(CalcError::UnexpectedEnd),
            _ => Err(CalcError::ExpectedOperand),
        }
    }

    // '(' expression ')'
    fn parse_parenthesized(&mut self) -> Result<f64, CalcError> {
        self.advance(); // consume '('
        let result = self.parse_expression()?;
        match self.advance() {
            Token::RParen => Ok(result),
            Token::Eof => Err(CalcError::UnclosedParen),
            _ => Err(CalcError::ExpectedRParen),
        }
    }
}
