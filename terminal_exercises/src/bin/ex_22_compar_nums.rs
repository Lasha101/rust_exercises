use std::io::{self, Write};

fn main() {
    let display_string: &str = &"Enter the number: ";

    let inputed_values = collect_inputs(display_string);
    
    let largest_num = comparing_function(inputed_values);

    println!("The largest number is {}.", largest_num);
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validate_nums(input:&str) -> Result<i32, String> {
    match input.trim().parse::<i32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(String::from("Cannot calculate value.")),
    }
}

fn collect_inputs(txt: &str) -> Vec<i32> {
    let mut user_inputs = Vec::new(); 
    loop {
        let input = get_user_input(txt);
        
        if input == "done" {
            break; 
        }

        let result = validate_nums(&input);
        
        match result {
            Ok(num) => {
                if user_inputs.contains(&num) {
                    println!("Number already entered. Try again.");
                    continue;
                } else {
                    user_inputs.push(num);
                }
            }
            Err(_) => {
                println!("Invalid input. Type a number or 'done' to finish.");
                continue;
            }
        }   
    }  
    user_inputs
}

fn comparing_function(arr: Vec<i32>) -> i32 {
    *arr.iter().max().unwrap_or(&0)
}
