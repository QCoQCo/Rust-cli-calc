mod calcul;
use std::io::{self,Write};
fn main() {
    loop{
        print!("Enter an expression (or 'exit' to quit): ");
        io::stdout().flush().unwrap();

        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input=input.trim();

        if input.eq_ignore_ascii_case("exit"){
            break;
        }
        match calcul::evl_ex(input) {
            Ok(result)=>println!("Result: {}",result),
            Err(e)=>println!("Error: {}",e),
        }
    }
}
