use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["How many euros are you exchanging? ", 
    "What is the exchange rate? "];

    let inputed_values = collect_inputs(
                              display_strings);

    let final_amount: f64 = converter(&inputed_values[0],
                         &inputed_values[1]);

    print_final_string(inputed_values[0], 
                       inputed_values[1],
                        final_amount);
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn collect_inputs(arr_str: &[&str]) -> [f64; 2] {
    let mut user_inputs = [0.0, 0.0];
    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            let input = get_user_input(*txt);
            match  input.trim().parse::<f64>() {
                Ok(parsed_number) => {
                    user_inputs[index] = parsed_number;
                    break;
                },
                Err(error_message) => println!("{}", error_message),
            }
        }
    }
    user_inputs
}

fn converter(amount: &f64, rate: &f64) -> f64 {
    amount * (rate / 1.0)
}




fn print_final_string(dollars: f64, rate: f64, final_amount: f64) {
    let dollars_as_int = dollars as i32;
    let rounded_final_amount = final_amount.round() / 100.0;
    println!("{} euros at an exchange rate of {} is\n\
              {} U.S. dollars",
              dollars_as_int, rate, rounded_final_amount);
}