use std::{
    collections::HashMap,
    io::{self, Write},
};

use reqwest::get;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Response {
    rates: HashMap<String, f64>,
}

#[tokio::main]
async fn main() {
    let display_strings: &[&str] = &[
        "How many euros are you exchanging? ",
        "In money of which country? ",
    ];

    let inputed_values = collect_inputs(display_strings).await;

    let final_amount = converter(
        &inputed_values.amount,
        &inputed_values.rate,
    );

    print_final_string(
        inputed_values.amount,
        inputed_values.rate,
        final_amount,
        &inputed_values.country_name,
    );
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);

    io::stdout()
        .flush()
        .expect("Error to show the text!");

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Fail to read text!");

    text.trim().to_string()
}

struct ExchangeInput {
    amount: f64,
    country_name: String,
    rate: f64,
}

async fn collect_inputs(arr_str: &[&str]) -> ExchangeInput {
    let mut amount = 0.0;
    let mut country_name = String::new();
    let mut rate = 0.0;

    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            if index == 0 {
                let input = get_user_input(*txt);

                match input.parse::<f64>() {
                    Ok(parsed_number) => {
                        amount = parsed_number;
                        break;
                    }

                    Err(error_message) => {
                        println!("{}", error_message)
                    }
                }
            } else if index == 1 {
                let from = "EUR";

                let to = get_user_input(*txt);

                let url = format!(
                    "https://api.frankfurter.app/latest?from={}&to={}",
                    from, to
                );

                let resp: Response =
                    get(&url).await.unwrap().json().await.unwrap();

                rate = *resp.rates.get(&to).unwrap();

                country_name = to;

                break;
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
    amount * rate
}

fn print_final_string(
    euros: f64,
    rate: f64,
    final_amount: f64,
    currency_name: &str,
) {
    println!(
        "{} euros at rate {} = {} {}",
        euros,
        rate,
        final_amount,
        currency_name
    );
}