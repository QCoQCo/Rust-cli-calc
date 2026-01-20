use super::token::{tokenize, Token};
use super::parser::Parser;

pub fn evl_ex(expression: &str) -> Result<f64, &'static str> {
    let tokens = tokenize(expression)?;
    if tokens.is_empty() {
        return Err("Empty expression");
    }
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expression()?;
    
    // 모든 토큰이 소비되었는지 확인
    if !matches!(parser.peek(), Token::EOF) {
        return Err("Unexpected token after expression");
    }
    
    Ok(result)
}
