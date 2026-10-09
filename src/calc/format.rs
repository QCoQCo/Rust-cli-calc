/// 부동소수점 정밀도 문제를 처리하고 결과를 깔끔하게 포맷팅
pub fn format_result(value: f64) -> String {
    // 이 범위를 벗어나면 지수 표기 (예: 1e20, 1e-11)
    const SCI_UPPER: f64 = 1e15;
    const SCI_LOWER: f64 = 1e-6;
    // 소수점 이하 최대 자릿수
    const MAX_DECIMALS: i32 = 10;
    // 표시할 최대 유효숫자 (f64는 약 15~17자리까지 정확)
    const MAX_DIGITS: i32 = 15;

    // 무한대나 NaN 체크
    if value.is_infinite() {
        return if value.is_sign_positive() {
            "infinity".to_string()
        } else {
            "-infinity".to_string()
        };
    }
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value == 0.0 {
        return "0".to_string(); // -0.0 포함
    }

    let abs = value.abs();
    if !(SCI_LOWER..SCI_UPPER).contains(&abs) {
        // 가수부를 반올림한 뒤 불필요한 0 제거 (1.5000000000e20 → 1.5e20)
        let formatted = format!("{:.*e}", MAX_DECIMALS as usize, value);
        let (mantissa, exponent) = formatted.split_once('e').unwrap();
        return format!("{}e{}", trim_decimal(mantissa), exponent);
    }

    // 정수부가 길면 소수 자릿수를 줄여 유효숫자 밖의 오차가 보이지 않게 함
    // (예: 0.1 + 0.2 = 0.30000000000000004 → 0.3)
    let int_digits = if abs >= 1.0 { abs.log10().floor() as i32 + 1 } else { 1 };
    let decimals = (MAX_DIGITS - int_digits).clamp(0, MAX_DECIMALS) as usize;
    let formatted = format!("{:.*}", decimals, value);
    match trim_decimal(&formatted) {
        "-0" => "0".to_string(),
        trimmed => trimmed.to_string(),
    }
}

/// 소수점 이하의 끝자리 0과, 남은 소수점을 제거
fn trim_decimal(s: &str) -> &str {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.')
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::format_result;

    // 출력 문자열을 다시 파싱했을 때 원래 값과 (상대오차 기준으로) 같아야 함
    fn assert_round_trip(value: f64) {
        let formatted = format_result(value);
        let parsed: f64 = formatted
            .parse()
            .unwrap_or_else(|_| panic!("{} formatted as unparseable `{}`", value, formatted));
        let tolerance = value.abs() * 1e-9;
        assert!(
            (parsed - value).abs() <= tolerance,
            "{} formatted as `{}`",
            value,
            formatted
        );
    }

    #[test]
    fn integers() {
        assert_eq!(format_result(4.0), "4");
        assert_eq!(format_result(-10.0), "-10");
        assert_eq!(format_result(0.0), "0");
        assert_eq!(format_result(-0.0), "0");
    }

    #[test]
    fn decimals() {
        assert_eq!(format_result(2.5), "2.5");
        assert_eq!(format_result(-0.25), "-0.25");
        assert_eq!(format_result(10.0 / 3.0), "3.3333333333");
    }

    #[test]
    fn floating_point_noise_is_removed() {
        assert_eq!(format_result(0.1 + 0.2), "0.3");
        assert_eq!(format_result(1.1 * 3.0), "3.3");
    }

    // B3: i64 범위를 넘는 큰 수
    #[test]
    fn large_numbers() {
        assert_round_trip(1e20);
        assert_round_trip(-1e20);
        assert_round_trip(1.5e300);
        assert_ne!(format_result(1e20), i64::MAX.to_string());
    }

    #[test]
    fn scientific_notation_boundaries() {
        assert_eq!(format_result(1e20), "1e20");
        assert_eq!(format_result(-1.5e20), "-1.5e20");
        assert_eq!(format_result(1e15), "1e15");
        assert_eq!(format_result(123456789012345.0), "123456789012345");
        assert_eq!(format_result(0.000001), "0.000001");
        assert_eq!(format_result(1e-11), "1e-11");
        assert_eq!(format_result(2.5e-7), "2.5e-7");
    }

    // B6: 아주 작은 수가 0으로 사라지면 안 됨
    #[test]
    fn small_numbers() {
        assert_round_trip(1e-11);
        assert_round_trip(-1e-11);
        assert_round_trip(1.5e-20);
    }
}
