use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let display_strings: &[&str] = 
    &["How many euros are you exchanging? ", 
    "In money of which country? "];

    let inputed_values = collect_inputs(
                              display_strings);

    let final_amount: f64 = converter(&inputed_values.amount,
                                      &inputed_values.rate);

    print_final_string(inputed_values.amount, 
                                inputed_values.rate,
                                final_amount, 
                                &inputed_values.country_name
                    );
}

fn currency_rates(country_name: &str) -> Result<f64, String>{
    let mut currency_rates = HashMap::new();
    currency_rates.insert("USA", 1.0);
    currency_rates.insert("Georgia", 3.25);
    currency_rates.insert("Japan", 23.0);
    currency_rates.insert("PRC", 14.0);
    currency_rates.insert("Turkye", 61.0);
    currency_rates.insert("Indenosia", 27.0);

    match currency_rates.get(country_name) {
        Some(rate) => Ok(*rate),
        None => Err(String::from("Country not found!")),
    }
}

fn currency_name(country_name: &str) -> Option<&'static str> {
    let mut currency_names = HashMap::new();
    currency_names.insert("USA", "United States Dollar");
    currency_names.insert("Georgia", "Georgian Lari");
    currency_names.insert("Japan", "Japanese Yen");
    currency_names.insert("PRC", "Chinese Yuan");
    currency_names.insert("Turkye", "Turkish Lira");
    currency_names.insert("Indenosia", "Indonesian Rupiah");

    currency_names.get(country_name).copied()
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

struct ExchangeInput {
    amount: f64, 
    country_name: String,
    rate: f64,
}

fn collect_inputs(arr_str: &[&str]) -> ExchangeInput {
    let mut amount = 0.0;
    let mut country_name = String::new();
    let mut rate = 0.0;

    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            if index == 0 {
                let input = get_user_input(*txt);
                match  input.trim().parse::<f64>() {
                    Ok(parsed_number) => {
                            amount = parsed_number;
                            break;
                        }
                        Err(error_message) => println!("{}", error_message),
                }
            } else if index == 1 {
                let input = get_user_input(*txt);
                match currency_rates(&input) {
                    Ok(parsed_number) => {
                        country_name = currency_name(&input).unwrap().to_string();
                        rate = parsed_number;
                        break;
                    }
                    Err(error_message) => println!("{}", error_message),
                }
            }
            
        }
    }
    ExchangeInput { 
            amount, 
            country_name, 
            rate,
        }
}

fn converter(amount: &f64, rate: &f64) -> f64 {
    amount * (rate / 1.0)
}




fn print_final_string(dollars: f64, rate: f64, final_amount: f64, currency_name: &str) {
    let dollars_as_int = dollars as i32;
    let rounded_final_amount = final_amount.round() / 100.0;
    println!("{} euros at an exchange rate of {} is\n\
              {} {}.",
              dollars_as_int, rate, rounded_final_amount, currency_name);
}