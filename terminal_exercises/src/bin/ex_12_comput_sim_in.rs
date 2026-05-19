use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["Enter the principal: ", 
    "Enter the rate of interest: ", 
    "Enter the number of years: "];

    let inputed_values = collect_inputs(
                              display_strings);

    for year in 1..=(inputed_values[2] as i32) {
        let amount = calculateSimpleInterest(
            &inputed_values[1],
            &inputed_values[0],
            &(year as f64),
        );
        print_final_string(year as f64, inputed_values[1], amount);
    }
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn collect_inputs(arr_str: &[&str]) -> [f64; 3] {
    let mut user_inputs = [0.0, 0.0, 0.0];
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

#[allow(non_snake_case)]
fn calculateSimpleInterest(rate: &f64, principal: &f64, years: &f64) -> f64 {
    principal * (1.0 + (rate / 100.0) * years)
}

fn print_final_string(years: f64, rate: f64, final_amount: f64) {
    let years_as_int = years as i32;
    println!(
        "After {} years at {}%, the investment will be worth ${:.2}.",
        years_as_int, rate, final_amount
    );
}