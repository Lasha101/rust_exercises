use std::io::{self, Write};



fn main() {
    let display_strings: &[&str] = 
    &["What is the username? ", "What is the password? "];
    let inputed_values = collect_inputs(
                              display_strings);

    let credentials = User {
        username: "user_user".to_string(),
        password: "123".to_string(), 
    };
    let slice_tuple = (inputed_values.0.as_str(), inputed_values.1.as_str());
    if  validator(&credentials, slice_tuple) {
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

struct User {
    username: String,
    password: String,
}



impl Validation for User {
    fn validation(&self, a: (&str, &str)) -> bool {
        a.0 == self.username && a.1 == self.password 
    }
}


fn validator(validation: &dyn Validation, a: (&str, &str)) -> bool {
    validation.validation(a) 
}