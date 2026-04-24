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

    let arr_bools = validating_inputs(display_strings);

    
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validating_name(input: &str) -> bool {
    input.trim().chars().count() >= 2
}

fn validating_zip_code(input:&str) -> bool {
    input.chars().all(|c| c.is_ascii_digit())
}

fn validating_employee_id(input: &str) -> bool {
    let matching_pattern = Regex::new(r"^[A-Z]{2}-\d{4}$").unwrap();
    matching_pattern.is_match(input)
}

fn validating_inputs(arr_str: &[&str]) -> [bool; 4] {
    let mut answers = [false; 4]; 
    for (index, txt) in arr_str.iter().enumerate() { 
        if index < 2 {
            let input = get_user_input(txt);
            answers[index] = validating_name(&input);
        } else if index == 2 {
            let input = get_user_input(txt);
            answers[index] = validating_zip_code(&input);
        } else if index == 3 {
            let input = get_user_input(txt);
            answers[index] = validating_employee_id(&input);
        }       
    }
    answers
}


