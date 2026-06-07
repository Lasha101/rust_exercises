use std::io::{self, Write};
use std::collections::HashMap;
use bcrypt::{hash, verify, DEFAULT_COST};



fn main() {
    let mut plain_credentials = HashMap::new();
    plain_credentials.insert("alice", "wonder123");
    plain_credentials.insert("bob", "builder123");
    plain_credentials.insert("rust", "coder123");

    let mut hashed_credentials = HashMap::new();
    for (username, plain_password) in plain_credentials.iter() {
        match hash(plain_password, DEFAULT_COST) {
            Ok(hashed_password) => {
                hashed_credentials.insert(username.to_string(), hashed_password);
            }
            Err(error) => {
                println!("Failed to hash password for user {}: {}", username, error);
            }
        }
    }

    let display_strings: &[&str] = 
    &["What is the username? ", "What is the password? "];
    let inputed_values = collect_inputs(
                              display_strings);

    let slice_tuple = (inputed_values.0.as_str(), inputed_values.1.as_str());
    if  validator(&hashed_credentials, slice_tuple) {
        println!("Welcome!")
    } else {
        println!("I don't know you.")
    }

}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validate_alpha_only(text: &str) -> bool {
    let is_valid = text != "";
    is_valid
}

fn collect_inputs(arr_str: &[&str]) -> (String, String) {
    let mut user_inputs = (String::new(), String::new());
    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            let input = get_user_input(*txt);
                if index == 0 && validate_alpha_only(&input) {
                    user_inputs.0 = input;
                    break;
                } else if index == 1 && validate_alpha_only(&input){
                    user_inputs.1 = input;
                    break;
                } else {
                    println!("{}", "Input cannot be empty! Please try again.")
                }
                
            }
        }
    user_inputs
}


trait Validation {
    fn validation(&self, a: (&str, &str)) -> bool;
}

impl Validation for HashMap<String, String> {
    fn validation(&self, a: (&str, &str)) -> bool {
        let (input_username, input_password) = a;
        match self.get(input_username) {
            Some(hashed_password) => verify(input_password, hashed_password).unwrap_or(false),
            None => false,
        }
    }
}

fn validator(validation: &dyn Validation, a: (&str, &str)) -> bool {
    validation.validation(a) 
}