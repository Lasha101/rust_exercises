use std::io::{self, Write};

fn main() {
    let display_string: &str = &"Enter password: ";
    let input = get_user_input(display_string);
    let strength_indicator = password_validator(&input);

    if strength_indicator == 1 {
        println!("The password '{}' is a very weak password.", input);
    } else if strength_indicator == 2 {
        println!("The password '{}' is a weak password.", input);
    } else if strength_indicator == 3 {
        println!("The password '{}' is a strong password.", input);

    } else if strength_indicator == 4 {
        println!("The password '{}' is a very strong password.", input);
    } else {
        println!("Pattern not recognized.")
    }
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn password_validator(input: &str) -> i32 {
    
    if !input.is_empty() && input.chars().count() < 8 
                         &&  input.chars().all(|c| c.is_numeric()) {
        1
    } else if !input.is_empty() && input.chars().count() < 8 
                                &&  input.chars().all(|c| c.is_alphabetic()) {
        2
    } else if input.chars().count() >= 8 
                              && input.chars().any(|c| c.is_alphabetic()) 
                              && input.chars().any(|c| c.is_numeric()) 
                              && !input.chars().any(|c| !c.is_alphanumeric()) {
        3
    } else if input.chars().count() >= 8 
                              && input.chars().any(|c| c.is_alphabetic()) 
                              && input.chars().any(|c| c.is_numeric()) 
                              && input.chars().any(|c| !c.is_alphanumeric()) {
        4
    } else {
        5
    }
}