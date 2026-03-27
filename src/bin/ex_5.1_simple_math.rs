use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["What is the first number? ", "What is the second number? "];
    let inputed_values = collect_inputs(
                              display_strings);
    for op in ops_list {
        print_final_string();
    }
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


fn print_final_string() {

    println!("{} {} {} = {}", val1, operator, val2, result);
}