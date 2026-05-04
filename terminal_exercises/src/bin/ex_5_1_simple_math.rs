use std::{io::{self, Write}};

fn main() {
    let display_strings: &[&str] = 
    &["What is the first number? ", "What is the second number? "];
    let inputed_values = collect_inputs(
                              display_strings);
    
    let mut local_buffer = [0; 4];
    let mut length_tracker: usize = 0;
    
    
    calculate(
        inputed_values[0], 
        inputed_values[1], 
        &mut local_buffer, 
        &mut length_tracker
    );
    
    let active_slice = &local_buffer[0..4];
    print_final_string(&inputed_values, active_slice);
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
                Ok(parsed_number) if parsed_number >= 0 => {
                    user_inputs.push(parsed_number);
                    break;
                },
                Ok(_) => println!("Please enter a non-negative number"),
                Err(error_message) => println!("{}", error_message),
            }
        }
    }
    user_inputs
}

fn calculate(val1: i32, val2: i32,
             buffer: &mut[i32], 
             current_length: &mut usize) {
    
    let start = *current_length;
    
    let chunk = &mut buffer[start..start + 4]; 

    chunk[0] = val1 + val2;
    chunk[1] = val1 - val2;
    chunk[2] = val1 * val2;
    chunk[3] = val1 / val2;
    
    *current_length += 4;
}


fn print_final_string(vals: &[i32], results: &[i32]) {
    let ops = ["+", "-", "*", "/"];
    for (i, op) in ops.iter().enumerate() {
        println!("{} {} {} = {}", vals[0], op, vals[1], results[i]);
    }
    
}