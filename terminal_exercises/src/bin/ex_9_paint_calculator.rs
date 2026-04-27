use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["What is the length of the room in feet? ", 
    "What is the width of the room in feet? "];

    let inputed_values = collect_inputs(
                              display_strings);

    let total_area = area(inputed_values[0],
                         inputed_values[1]);

    const INDEX: i32 = 350;

    let num_of_gallons = num_of_gallons_to_paint(&total_area, &INDEX);

    print_final_string(num_of_gallons, total_area);
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

fn area(length: i32, width: i32) -> i32 {
    length * width 
}

fn num_of_gallons_to_paint(area: &i32, index: &i32) -> i32 {
    let result = (*area as f64) / (*index as f64);
    result.ceil() as i32
}


fn print_final_string(num_of_gallons: i32, area: i32) {
    println!("You will need to purchase {} gallons of\n\
              paint to cover {} square feet.",
             num_of_gallons, area);
}