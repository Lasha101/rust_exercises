use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["How many people? ", 
    "How many number of slices per person? "];

    let inputed_values = collect_inputs(
                              display_strings);

    let number_pizzas = calc_num_pizzas(inputed_values[0],
                                      inputed_values[1]);

    print_final_string(
        &inputed_values[0], 
        &inputed_values[1],
        number_pizzas);           
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

fn calc_num_pizzas(people: i32, slice_per_person: i32) -> i32 {
    const SLICES_PER_PIZZA: f64 = 8.0;
    ((people as f64 * slice_per_person as f64) / SLICES_PER_PIZZA).ceil() as i32
}


fn print_final_string(person: &i32, slice_per_person: &i32, num_pizzas: i32) {
    let piece = if slice_per_person <= &1 {
        "piece"
    } else {
        "pieces"
    };
    let pizza_word = if num_pizzas == 1 {
        "pizza"
    } else {
        "pizzas"
    };
    println!("{} people, {} slices per pizza\n\
             Each person gets {} {} of pizza.\n\
             You need to purchase {} full {}.", 
             person, 8, slice_per_person, piece,
             num_pizzas, pizza_word);
}