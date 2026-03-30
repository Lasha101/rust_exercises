use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["What is the length of the room in feet? ", 
    "What is the width of the room in feet? "];

    let inputed_values = collect_inputs(
                              display_strings);

    println!("You entered dimensions of 15 feet by 20 feet.");
    
    let area = area(&inputed_values[0],
                         &inputed_values[1]);

    let area_in_meters = area_in_meters(&area);

    print_final_string(area, area_in_meters);
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

fn area(length: &i32, width: &i32) -> i32 {
    length * width 
}

fn area_in_meters(area: &i32) -> f64 {
    let result = (*area as f64) * 0.09290304;
    (result * 1000.0).round() / 1000.0
}


fn print_final_string(area: i32, area_in_meters: f64) {
    println!("The area is\n{} square feet\n{} square meters",
             area, area_in_meters);
}