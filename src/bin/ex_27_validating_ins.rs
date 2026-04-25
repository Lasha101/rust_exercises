use std::io::{self, Write};
use regex::Regex;

fn main() {
    let display_strings: &[&str] = 
    &[
    "Enter the first name: ", 
    "Enter the last name: ", 
    "Enter the ZIP code: ",
    "Enter an employee ID: "
    ];

    let arr_messages = validating_inputs(display_strings);
    
    let mut error_detected = false;

    for msg in arr_messages {
        if !msg.is_empty() {
            println!("{}", msg);
            error_detected = true;
        }
    }
    if !error_detected {
        println!("There were no errors found.")
    }
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validating_empty_or_not(input: &str) -> bool {
    !input.is_empty()
}

fn validating_str_length(input: &str) -> bool {
    input.trim().chars().count() >= 2
}

fn validating_zip_code(input:&str) -> bool {
    input.chars().all(|c| c.is_ascii_digit())
}

fn validating_employee_id(input: &str) -> bool {
    let matching_pattern = Regex::new(r"^[A-Z]{2}-\d{4}$").unwrap();
    matching_pattern.is_match(input)
}

fn validating_inputs(arr_str: &[&str]) -> [String; 4] {
    let mut answers = std::array::from_fn(|_| String::new()); 

    for (index, txt) in arr_str.iter().enumerate() { 
        let input = get_user_input(txt);
        if index == 0 {
            if !validating_empty_or_not(&input) {
                answers[index] = format!("The first name must be filled in.");
            } else if !validating_str_length(&input) {
                answers[index] = format!(
                    "{} is not a valid first name. It is too short.", input
                );
            }
        } else if index == 1 {
            if !validating_empty_or_not(&input) {
                answers[index] = format!("The last name must be filled in.");
            } else if !validating_str_length(&input) {
                answers[index] = format!(
                    "\"{}\" is not a valid last name. It is too short.", input
                );
            }
        } else if index == 2 {
            if !validating_zip_code(&input) {
                answers[index] = String::from("The ZIP code must be numeric.");
            }
        } else if index == 3 {
            if !validating_employee_id(&input) {
                answers[index] = format!("{} is not a valid ID.", input);
            }
        } 
    }
    answers      
}



