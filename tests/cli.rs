//! 실제 바이너리를 실행해 표준 입력/출력으로 동작을 검증하는 통합 테스트

use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const TIMEOUT: Duration = Duration::from_secs(5);

/// 입력을 전달하고 stdin을 닫은 뒤 출력을 반환. 제한 시간 안에 종료되지 않으면 None
fn run(input: &str) -> Option<String> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_cli_calcul"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start calculator");

    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();

    // 출력이 파이프 버퍼를 채워 자식 프로세스가 멈추지 않도록 별도 스레드에서 읽음
    let mut stdout = child.stdout.take().unwrap();
    let reader = thread::spawn(move || {
        let mut output = Vec::new();
        let _ = stdout.read_to_end(&mut output);
        output
    });

    let start = Instant::now();
    loop {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        if start.elapsed() > TIMEOUT {
            child.kill().unwrap();
            child.wait().unwrap();
            return None;
        }
        thread::sleep(Duration::from_millis(20));
    }

    Some(String::from_utf8_lossy(&reader.join().unwrap()).into_owned())
}

/// 각 표현식에 대한 "Result: ..." 또는 "Error: ..." 줄만 순서대로 추출
fn outcomes(input: &str) -> Vec<String> {
    let output = run(input).expect("calculator did not exit in time");
    output
        .lines()
        .map(|line| line.trim_start_matches("> "))
        .filter(|line| line.starts_with("Result: ") || line.starts_with("Error: "))
        .map(str::to_string)
        .collect()
}

// B1: EOF를 만나면 종료해야 함
#[test]
fn exits_on_eof() {
    assert!(run("1+1\n").is_some(), "calculator kept running after EOF");
    assert!(run("").is_some(), "calculator kept running after EOF");
}

#[test]
fn exits_on_command() {
    let output = run("exit\n").expect("calculator did not exit in time");
    assert!(output.contains("Goodbye!"));
}

#[test]
fn evaluates_expressions() {
    assert_eq!(
        outcomes("2 + 3 * 4\n1 / 0\nexit\n"),
        ["Result: 14", "Error: Cannot divide by ZERO"]
    );
}

#[test]
fn ans_uses_last_result() {
    assert_eq!(
        outcomes("10 + 5\nans * 2\nans + 10\nexit\n"),
        ["Result: 15", "Result: 30", "Result: 40"]
    );
}

// B4: ans는 반올림된 문자열이 아닌 원래 값을 사용해야 함
#[test]
fn ans_keeps_full_precision() {
    assert_eq!(outcomes("1/3\nans*3\nexit\n"), ["Result: 0.3333333333", "Result: 1"]);
    assert_eq!(outcomes("2^0.5\nans^2\nexit\n"), ["Result: 1.4142135624", "Result: 2"]);
}

// B4: 음수 ans는 하나의 값으로 취급되어야 함 (-3^2 가 아닌 (-3)^2)
#[test]
fn negative_ans_is_a_single_value() {
    assert_eq!(outcomes("0-3\nans^2\nexit\n"), ["Result: -3", "Result: 9"]);
}

// B4: 이전 결과가 없을 때 명확한 에러
#[test]
fn ans_without_previous_result() {
    let results = outcomes("ans + 1\nexit\n");
    assert_eq!(results.len(), 1);
    assert!(
        results[0].starts_with("Error: ") && results[0].contains("No previous result"),
        "got {:?}",
        results[0]
    );
}

// B5: 유한하지 않은 결과는 에러이며 ans를 덮어쓰지 않음
#[test]
fn non_finite_result_does_not_replace_ans() {
    let results = outcomes("2 + 3\n10^400\nans + 1\nexit\n");
    assert_eq!(results.len(), 3);
    assert_eq!(results[0], "Result: 5");
    assert!(results[1].starts_with("Error: "), "got {:?}", results[1]);
    assert_eq!(results[2], "Result: 6");
}
