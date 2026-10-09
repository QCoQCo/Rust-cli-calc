use std::fmt;

/// 계산 중 발생할 수 있는 에러
#[derive(Debug, Clone, PartialEq)]
pub enum CalcError {
    // 토큰화
    InvalidCharacter(char),
    UnknownFunction(String),
    InvalidNumber(String),
    TrailingDot,
    NoNumber,

    // 파싱
    UnexpectedToken,
    TrailingInput,
    ExpectedOperand,
    UnexpectedEnd,
    UnmatchedRParen,
    UnclosedParen,
    ExpectedRParen,
    ExpectedLParenAfterSqrt,

    // 평가
    DivisionByZero,
    NegativeSqrt,
    NoPreviousResult,
    NonFiniteResult,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::InvalidCharacter(ch) => write!(f, "Invalid character '{}' in expression", ch),
            CalcError::UnknownFunction(name) => write!(f, "Unknown function '{}'", name),
            CalcError::InvalidNumber(num) => write!(f, "Invalid number '{}'", num),
            CalcError::TrailingDot => write!(f, "Number cannot end with '.'"),
            CalcError::NoNumber => write!(f, "Expression must contain at least one number"),
            CalcError::UnexpectedToken => write!(f, "Unexpected token in expression"),
            CalcError::TrailingInput => write!(f, "Unexpected token after expression"),
            CalcError::ExpectedOperand => write!(f, "Expected number, '-', '+', 'ans', 'sqrt', or '('"),
            CalcError::UnexpectedEnd => write!(f, "Unexpected end of expression"),
            CalcError::UnmatchedRParen => write!(f, "Unexpected ')' - no matching '('"),
            CalcError::UnclosedParen => write!(f, "Unclosed parenthesis: expected ')'"),
            CalcError::ExpectedRParen => write!(f, "Expected ')' after expression"),
            CalcError::ExpectedLParenAfterSqrt => write!(f, "Expected '(' after sqrt"),
            CalcError::DivisionByZero => write!(f, "Cannot divide by ZERO"),
            CalcError::NegativeSqrt => write!(f, "Cannot take square root of negative number"),
            CalcError::NoPreviousResult => write!(f, "No previous result for 'ans'"),
            CalcError::NonFiniteResult => write!(f, "Result is not a finite number"),
        }
    }
}

impl std::error::Error for CalcError {}
