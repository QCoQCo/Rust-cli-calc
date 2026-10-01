/// 부동소수점 정밀도 문제를 처리하고 결과를 깔끔하게 포맷팅
pub fn format_result(value: f64) -> String {
    // 부동소수점 오차 처리 (예: 0.1 + 0.2 = 0.30000000000000004)
    // 매우 작은 오차는 반올림하여 제거
    const EPSILON: f64 = 1e-10;
    
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
    
    // 정수인지 확인 (소수점 오차 고려)
    let rounded = (value * 1e10).round() / 1e10;
    if (rounded - rounded.round()).abs() < EPSILON {
        // 정수로 표시
        format!("{}", rounded.round() as i64)
    } else {
        // 소수점이 있는 경우, 불필요한 0 제거
        let formatted = format!("{:.15}", rounded);
        let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
        
        // 최대 10자리 소수점까지만 표시 (불필요한 정밀도 제거)
        if trimmed.contains('.') {
            let parts: Vec<&str> = trimmed.split('.').collect();
            if parts.len() == 2 {
                let decimal = parts[1];
                if decimal.len() > 10 {
                    format!("{:.10}", rounded).trim_end_matches('0').trim_end_matches('.').to_string()
                } else {
                    trimmed.to_string()
                }
            } else {
                trimmed.to_string()
            }
        } else {
            trimmed.to_string()
        }
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

    // B6: 아주 작은 수가 0으로 사라지면 안 됨
    #[test]
    fn small_numbers() {
        assert_round_trip(1e-11);
        assert_round_trip(-1e-11);
        assert_round_trip(1.5e-20);
    }
}
