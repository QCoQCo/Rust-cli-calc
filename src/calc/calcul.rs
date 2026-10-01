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

#[cfg(test)]
mod tests {
    use super::evl_ex;

    fn assert_eval(expression: &str, expected: f64) {
        let result = evl_ex(expression)
            .unwrap_or_else(|e| panic!("`{}` failed: {}", expression, e));
        assert!(
            (result - expected).abs() < 1e-9,
            "`{}` = {}, expected {}",
            expression,
            result,
            expected
        );
    }

    fn assert_eval_err(expression: &str) {
        let result = evl_ex(expression);
        assert!(result.is_err(), "`{}` should fail, got {:?}", expression, result);
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
        assert_eq!(evl_ex("1 / 0"), Err("Cannot divide by ZERO"));
        assert_eq!(evl_ex("1 % 0"), Err("Cannot divide by ZERO"));
        assert_eq!(evl_ex("1 / (2 - 2)"), Err("Cannot divide by ZERO"));
    }

    #[test]
    fn syntax_errors() {
        assert_eval_err("");
        assert_eval_err("2 +");
        assert_eval_err("(2 + 3");
        assert_eval_err("2 + 3)");
        assert_eval_err("2 3");
        assert_eval_err("sqrt 4");
        assert_eval_err("sqrt(-1)");
        assert_eval_err("foo(1)");
    }

    // B5: 결과가 유한한 수가 아니면 에러
    #[test]
    fn non_finite_result_is_error() {
        assert_eval_err("10^400");
        assert_eval_err("-10^400");
        assert_eval_err("(-8)^(1/3)");
    }
}
