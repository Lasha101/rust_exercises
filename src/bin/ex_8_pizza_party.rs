use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["How many people? ", 
    "How many pizzas do you have? ",
    "How many number of slices per pizza? "];

    let inputed_values = collect_inputs(
                              display_strings);

    let pizza_per_person = calc_pizza_per_person();

    let leftover = calc_leftover();

    print_final_string();
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

fn calc_pizza_per_person(person: i32, pizza: i32, slice: i32) -> i32 {
    todo!()
}

fn calc_leftover(person: &i32, pizza: &i32, slice: i32) -> i32 {
    todo!()
}


fn print_final_string(person: i32, pizza: i32, slice: i32, leftover: i32) {
    println!("{} people with {} pizzas\n\
             Each person gets {} pieces of pizza.\n\
             There are {} leftover pieces.", 
             person, pizza, slice, leftover);
}