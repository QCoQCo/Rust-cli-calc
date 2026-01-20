mod calc;
use std::io::{self,Write};
use std::collections::VecDeque;

struct HistoryEntry {
    expression: String,
    result: f64,
}

struct Calculator {
    history: VecDeque<HistoryEntry>,
    max_history: usize,
    last_result: Option<f64>,
}

impl Calculator {
    fn new(max_history: usize) -> Self {
        Self {
            history: VecDeque::with_capacity(max_history),
            max_history,
            last_result: None,
        }
    }

    fn add_to_history(&mut self, expression: String, result: f64) {
        if self.history.len() >= self.max_history {
            self.history.pop_front();
        }
        self.history.push_back(HistoryEntry { expression, result });
        self.last_result = Some(result);
    }

    fn show_history(&self) {
        if self.history.is_empty() {
            println!("No history available.");
            return;
        }
        println!("\n--- Calculation History ---");
        for (i, entry) in self.history.iter().enumerate() {
            println!("{}: {} = {}", 
                i + 1, 
                entry.expression, 
                calc::format_result(entry.result)
            );
        }
        println!("---------------------------\n");
    }

    fn clear_history(&mut self) {
        self.history.clear();
        println!("History cleared.");
    }

    fn show_help(&self) {
        println!("\n--- Calculator Commands ---");
        println!("  <expression>  - Calculate expression");
        println!("  history, h    - Show calculation history");
        println!("  clear, c      - Clear history");
        println!("  help, ?       - Show this help");
        println!("  exit, quit    - Exit calculator");
        println!("  ans           - Use last result in expression");
        println!("---------------------------\n");
    }
}

fn main() {
    let mut calc = Calculator::new(50);
    println!("Calculator with history. Type 'help' for commands.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // 명령어 처리
        match input.to_lowercase().as_str() {
            "exit" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "history" | "h" => {
                calc.show_history();
                continue;
            }
            "clear" | "c" => {
                calc.clear_history();
                continue;
            }
            "help" | "?" => {
                calc.show_help();
                continue;
            }
            _ => {}
        }

        // ans를 마지막 결과로 치환
        let expression = if let Some(last) = calc.last_result {
            input.replace("ans", &calc::format_result(last))
        } else {
            input.to_string()
        };

        match calc::evl_ex(&expression) {
            Ok(result) => {
                let formatted = calc::format_result(result);
                println!("Result: {}", formatted);
                calc.add_to_history(input.to_string(), result);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
