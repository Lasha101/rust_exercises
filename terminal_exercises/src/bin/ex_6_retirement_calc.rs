use std::{io::{self, Write}};
use chrono::{Local, Datelike};

fn main() {
    let display_strings: &[&str] = 
    &["What is your current age? ", "At what age would you like to retire? "];
    let inputed_values = collect_inputs(
                              display_strings);
    let dt = Local::now().year();
    
    let left_ret_years = calculate_left_ret_year(
        inputed_values[0], 
        inputed_values[1], 
        dt, 
    );
    
    if left_ret_years.0 > 0 {
        print_final_string(&left_ret_years.0,
                       &left_ret_years.1,
                       dt);
    } else if left_ret_years.0 == 0 {
        println!("You can retire in this year.")
    } else {
        println!("You can already retire.")
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

fn calculate_left_ret_year(age: i32, want_to_retire: i32, current_year: i32) -> (i32, i32) {
    let left_years = want_to_retire - age;
    let ret_year = current_year + left_years;
    
    (left_years, ret_year)  
}


fn print_final_string(left_years: &i32, ret_year: &i32, current_year: i32) {
    println!(
            "You have {} years left until you can retire.\n\
            It's {}, so you can retire in {}.",
              left_years, current_year, ret_year
            );   
}


