use super::token::{tokenize, Token};
use super::error::CalcError;
use super::parser::Parser;

/// `ans`는 `last_result` 값으로 평가됨
pub fn evl_ex(expression: &str, last_result: Option<f64>) -> Result<f64, CalcError> {
    let tokens = tokenize(expression)?;
    let mut parser = Parser::new(tokens, last_result);
    let result = parser.parse_expression()?;
    
    // 모든 토큰이 소비되었는지 확인
    if !matches!(parser.peek(), Token::Eof) {
        return Err(CalcError::TrailingInput);
    }

    // 오버플로(inf)나 정의되지 않는 연산(NaN) 결과는 에러로 처리
    if !result.is_finite() {
        return Err(CalcError::NonFiniteResult);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::evl_ex;
    use crate::calc::error::CalcError;

    fn assert_eval(expression: &str, expected: f64) {
        let result = evl_ex(expression, None)
            .unwrap_or_else(|e| panic!("`{}` failed: {}", expression, e));
        assert!(
            (result - expected).abs() < 1e-9,
            "`{}` = {}, expected {}",
            expression,
            result,
            expected
        );
    }

    fn assert_eval_err(expression: &str, expected: CalcError) {
        assert_eq!(evl_ex(expression, None), Err(expected), "`{}`", expression);
    }

    // 기본 연산 (README 예시)
    #[test]
    fn basic_operations() {
        assert_eval("10 + 20 - 5", 25.0);
        assert_eval("2 * 3 + 4 * 5", 26.0);
        assert_eval("2 + 3 * 4", 14.0);
        assert_eval("10 / 4", 2.5);
        assert_eval("10 % 3", 1.0);
    }

    #[test]
    fn parentheses() {
        assert_eval("(2 + 3) * 4", 20.0);
        assert_eval("((2 + 3) * 4) + 5", 25.0);
        assert_eval("(10 - 5) / (2 + 3)", 1.0);
    }

    #[test]
    fn power_and_sqrt() {
        assert_eval("2^3", 8.0);
        assert_eval("2^3^2", 512.0); // 우측 결합
        assert_eval("sqrt(16)", 4.0);
        assert_eval("sqrt(2 + 2)", 2.0);
        assert_eval("sqrt(16) + 2^3 * 2", 20.0);
    }

    #[test]
    fn unary_operators() {
        assert_eval("-5 + 3", -2.0);
        assert_eval("-5 * 2", -10.0);
        assert_eval("-(-5)", 5.0);
        assert_eval("+5", 5.0);
        assert_eval("(-2 + 3) * 4", 4.0);
        assert_eval("2 * -3", -6.0);
    }

    // B2: 단항 마이너스는 거듭제곱보다 우선순위가 낮아야 함
    #[test]
    fn unary_minus_binds_looser_than_power() {
        assert_eval("-2^2", -4.0);
        assert_eval("-2^2 + 1", -3.0);
        assert_eval("(-2)^2", 4.0);
    }

    #[test]
    fn negative_exponent() {
        assert_eval("2^-1", 0.5);
        assert_eval("2^-2^2", 0.0625); // 2^(-(2^2))
    }

    #[test]
    fn division_by_zero() {
        assert_eq!(evl_ex("1 / 0", None), Err(CalcError::DivisionByZero));
        assert_eq!(evl_ex("1 % 0", None), Err(CalcError::DivisionByZero));
        assert_eq!(evl_ex("1 / (2 - 2)", None), Err(CalcError::DivisionByZero));
    }

    #[test]
    fn syntax_errors() {
        assert_eval_err("", CalcError::NoNumber);
        assert_eval_err("2 +", CalcError::UnexpectedEnd);
        assert_eval_err("2 * * 3", CalcError::ExpectedOperand);
        assert_eval_err("(2 + 3", CalcError::UnclosedParen);
        assert_eval_err("2 + 3)", CalcError::TrailingInput);
        assert_eval_err("())", CalcError::NoNumber);
        assert_eval_err("(1))", CalcError::TrailingInput);
        assert_eval_err("1 + )", CalcError::UnmatchedRParen);
        assert_eval_err("2 3", CalcError::UnexpectedToken);
        assert_eval_err("sqrt 4", CalcError::ExpectedLParenAfterSqrt);
        assert_eval_err("sqrt(-1)", CalcError::NegativeSqrt);
        assert_eval_err("foo(1)", CalcError::UnknownFunction("foo".to_string()));
        assert_eval_err("1 $ 2", CalcError::InvalidCharacter('$'));
    }

    // B4: ans는 이전 결과 값을 그대로 사용
    #[test]
    fn ans_uses_last_result() {
        assert_eq!(evl_ex("ans * 2", Some(15.0)), Ok(30.0));
        assert_eq!(evl_ex("ans * 3", Some(1.0 / 3.0)), Ok(1.0));
        assert_eq!(evl_ex("ans^2", Some(-3.0)), Ok(9.0));
        assert_eq!(evl_ex("-ans", Some(-3.0)), Ok(3.0));
        assert_eq!(evl_ex("sqrt(ans)", Some(16.0)), Ok(4.0));
    }

    #[test]
    fn ans_without_previous_result() {
        assert_eq!(evl_ex("ans + 1", None), Err(CalcError::NoPreviousResult));
        assert_eq!(evl_ex("1 + 1", Some(5.0)), Ok(2.0));
    }

    // B5: 결과가 유한한 수가 아니면 에러
    #[test]
    fn non_finite_result_is_error() {
        assert_eval_err("10^400", CalcError::NonFiniteResult);
        assert_eval_err("-10^400", CalcError::NonFiniteResult);
        assert_eval_err("(-8)^(1/3)", CalcError::NonFiniteResult);
    }
}
