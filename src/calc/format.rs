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
