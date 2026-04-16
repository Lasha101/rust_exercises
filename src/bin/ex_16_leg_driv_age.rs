use std::io::{self, Write};

fn main() {
    let display_string: &str = "What is your age? ";

    let inputed_value = validate_input(display_string);
    if comparing_function(inputed_value) {
        println!("You are old enough to legally drive.");
    } else {
        println!("You are not old enough to legally drive.");
    }
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validate_input(txt: &str) -> i32 {
    loop {
        let input = get_user_input(txt);
        match  input.trim().parse::<i32>() {
            Ok(parsed_number) => return parsed_number,
            Err(_) => println!("Invalid input. Please enter a whole number."),
        }
    }
}

fn comparing_function(age: i32) -> bool {
    age >= 16
}

