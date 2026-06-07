use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let mut driving_age_by_country = HashMap::new();
    driving_age_by_country.insert("El Salvador".to_string(), 15);
    driving_age_by_country.insert("United Kingdom".to_string(), 17);
    driving_age_by_country.insert("Japan".to_string(), 18);

    let display_string: &str = "What is your age? ";

    let inputed_value = validate_input(display_string);

    let countries = comparing_function(inputed_value, driving_age_by_country);
    if !countries.is_empty() {
        // Joins the country names with a comma and a space
        println!("You are old enough to legally drive in {}.", countries.join(", "));
    } else {
        println!("You are not old enough to legally drive.");
    }
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validate_input(txt: &str) -> i32 {
    loop {
        let input = get_user_input(txt);
        match  input.trim().parse::<i32>() {
            Ok(parsed_number) if parsed_number >= 1 => return parsed_number,
            Ok(_) => println!("Invalid input. The number must be greater than 0."),
            Err(_) => println!("Invalid input. Please enter a whole number."),
        }
    }
}

fn comparing_function(input_age: i32, driving_age_country: HashMap<String, i32>) -> Vec<String> {
    let countries: Vec<String> = driving_age_country
        .into_iter()
        .filter(|&(_, age)| input_age >= age)
        .map(|(name, _)| name)
        .collect();
    countries
}

