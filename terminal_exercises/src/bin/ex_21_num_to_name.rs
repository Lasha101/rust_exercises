use std::io::{self, Write};

fn main() {
    let display_string: &str = "Please enter the number of the month: ";

    let answer = validate_input(display_string);
    
    println!("{}", answer);
  
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validate_input(txt: &str) -> &str {
    loop {
        let input: Result<i32, _> = get_user_input(txt).parse();
        match input {
            Ok(1) => return "The name of the month is January.",
            Ok(2) => return "The name of the month is February.",
            Ok(3) => return "The name of the month is March.",
            Ok(4) => return "The name of the month is April.",
            Ok(5) => return "The name of the month is May.",
            Ok(6) => return "The name of the month is June.",
            Ok(7) => return "The name of the month is July.",
            Ok(8) => return "The name of the month is August.",
            Ok(9) => return "The name of the month is September.",
            Ok(10) => return "The name of the month is October.",
            Ok(11) => return "The name of the month is November.",
            Ok(12) => return "The name of the month is December.",
            _ => println!("Enter number 1 - 12."),
        }
    }
}

