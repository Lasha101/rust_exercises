use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["What is the first number? ", "What is the second number? "];
    let inputed_values = collect_inputs(
                              display_strings);
    
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn collect_inputs(arr_str: &[&str]) -> Vec<i32> {
    let mut user_inputs = Vec::new();
    for txt in arr_str {
        loop {
            let input = get_user_input(*txt);
            match  input.trim().parse::<i32>() {
                Ok(parsed_number) => {
                    user_inputs.push(parsed_number);
                    break;
                },
                Err(error_message) => println!("{}", error_message),
            }
        }
    }
    user_inputs
}

trait Operation {
    fn calculate(&self, a: i32, b: i32) -> i32;
    fn symbol(&self) -> char;
}

struct AddAction;
struct SubAction;
struct MultAction;
struct DivAction;

impl Operation for AddAction {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        a + b
    }
    fn symbol(&self) -> char {
        '+'
    }

}

impl Operation for SubAction {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        a - b
    }
    fn symbol(&self) -> char {
        '-'
    }
}

impl Operation for MultAction {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        a * b
    }
    fn symbol(&self) -> char {
        '*'
    }
}

impl Operation for DivAction {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        a / b
    }
    fn symbol(&self) -> char {
        '/'
    }
}

fn print_final_string(action: &dyn Operation, val1: i32, val2: i32) {
    let result = action.calculate(val1, val2);
    let operator = action.symbol();

    println!("{} {} {} = {}", val1, operator, val2, result);
}