use std::io::{self, Write};
use std::collections::HashMap;
fn main() {
    let mut state_bac_limit = HashMap::new();
    state_bac_limit.insert("alabama".to_string(), 0.08);
    state_bac_limit.insert("alaska".to_string(), 0.08);
    state_bac_limit.insert("arizona".to_string(), 0.08);
    state_bac_limit.insert("arkansas".to_string(), 0.08);
    state_bac_limit.insert("california".to_string(), 0.08);
    state_bac_limit.insert("colorado".to_string(), 0.08);
    state_bac_limit.insert("connecticut".to_string(), 0.08);
    state_bac_limit.insert("delaware".to_string(), 0.08);
    state_bac_limit.insert("florida".to_string(), 0.08); 
    state_bac_limit.insert("georgia".to_string(), 0.08);
    state_bac_limit.insert("hawaii".to_string(), 0.08);
    state_bac_limit.insert("idaho".to_string(), 0.08); 
    state_bac_limit.insert("illinois".to_string(), 0.08);
    state_bac_limit.insert("indiana".to_string(), 0.08);
    state_bac_limit.insert("iowa".to_string(), 0.08);
    state_bac_limit.insert("kansas".to_string(), 0.08);
    state_bac_limit.insert("kentucky".to_string(), 0.08);
    state_bac_limit.insert("louisiana".to_string(), 0.08);
    state_bac_limit.insert("maine".to_string(), 0.08);
    state_bac_limit.insert("maryland".to_string(), 0.08);
    state_bac_limit.insert("massachusetts".to_string(), 0.08);
    state_bac_limit.insert("michigan".to_string(), 0.08);
    state_bac_limit.insert("minnesota".to_string(), 0.08);
    state_bac_limit.insert("mississippi".to_string(), 0.08);
    state_bac_limit.insert("missouri".to_string(), 0.08);
    state_bac_limit.insert("montana".to_string(), 0.08);
    state_bac_limit.insert("nebraska".to_string(), 0.08);
    state_bac_limit.insert("nevada".to_string(), 0.08);
    state_bac_limit.insert("new hampshire".to_string(), 0.08);
    state_bac_limit.insert("new jersey".to_string(), 0.08);
    state_bac_limit.insert("new mexico".to_string(), 0.08);
    state_bac_limit.insert("new york".to_string(), 0.08);
    state_bac_limit.insert("north carolina".to_string(), 0.08);
    state_bac_limit.insert("north dakota".to_string(), 0.08);
    state_bac_limit.insert("ohio".to_string(), 0.08);
    state_bac_limit.insert("oklahoma".to_string(), 0.08);
    state_bac_limit.insert("oregon".to_string(), 0.08);
    state_bac_limit.insert("pennsylvania".to_string(), 0.08);
    state_bac_limit.insert("rhode island".to_string(), 0.08);
    state_bac_limit.insert("south carolina".to_string(), 0.08);
    state_bac_limit.insert("south dakota".to_string(), 0.08);
    state_bac_limit.insert("tennessee".to_string(), 0.08);
    state_bac_limit.insert("texas".to_string(), 0.08);
    state_bac_limit.insert("utah".to_string(), 0.05); 
    state_bac_limit.insert("vermont".to_string(), 0.08);
    state_bac_limit.insert("virginia".to_string(), 0.08);
    state_bac_limit.insert("washington".to_string(), 0.08);
    state_bac_limit.insert("west virginia".to_string(), 0.08);
    state_bac_limit.insert("wisconsin".to_string(), 0.08);
    state_bac_limit.insert("wyoming".to_string(), 0.08);

    let display_strings: &[&str] = 
    &[
    "Enter your weight: ", 
    "Enter your gender: ", 
    "Enter the number of drinks: ",
    "Enter the amount of alcohol by volume of the drinks consumed: ",
    "Enter the amount of time since your last drink: ",
    "Enter the name of the state: "
    ];

    let inputed_values = collect_inputs(
                              display_strings, &state_bac_limit);

    let ratio: f64 = blood_alc_calculator(
                                        &inputed_values[0],
                                        &inputed_values[1],
                                        &inputed_values[2],
                                        &inputed_values[3],
                                        &inputed_values[4]
                                    );

    print_final_string(ratio, inputed_values[5]);
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validate_nums(input:&str) -> Result<f64, String> {
    match input.trim().parse::<f64>() {
        Ok(number) => Ok(number),
        Err(_) => Err(String::from("Cannot calculate value.")),
    }
}

fn validate_gender(input: &str) -> Result<f64, String> {
    match  input.trim() {
        "m" => Ok(0.73), 
        "f" => Ok(0.66),
        _ => Err(String::from("Char not recongnized.")),
    }
}

fn get_state_bac_limit(input: &str, state_bac_limits: &HashMap<String, f64>) -> Result<f64, String> {
    let limit: Result<f64, String> = state_bac_limits.get(&input.to_lowercase())
    .copied().ok_or(String::from("State name not found.")); 
    limit
}

fn collect_inputs(arr_str: &[&str], state_bac_limits: &HashMap<String, f64>) -> [f64; 6] {
    let mut user_inputs = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            let input = get_user_input(txt);
            if index != 1 && index != 5 {
                let result = validate_nums(&input);
                match result {
                    Ok(number) => {
                        user_inputs[index] = number;
                        break;
                    }
                    Err(e) => println!("{}", e),
                };
            } else if index == 1 {
                let result = validate_gender(&input);
                match result {
                    Ok(number) => {
                        user_inputs[index] = number;
                        break;
                    }
                    Err(e) => println!("{}", e),
                };
            } else {
                let result = get_state_bac_limit(&input, state_bac_limits);
                match result {
                    Ok(number) => {
                        user_inputs[index] = number;
                        break;
                    }
                    Err(e) => println!("{}", e),
                };
            }    
        }  
    }
    user_inputs
}

fn blood_alc_calculator(weight: &f64,
                        gender: &f64,
                        num_of_drinks: &f64, 
                        alcohol_per_unit: &f64, 
                        passed_time: &f64) -> f64 {
    (num_of_drinks * alcohol_per_unit * 5.14) /
    (weight * gender) - 0.015 * passed_time
}

fn print_final_string(ratio: f64, limit: f64) {
    println!("Your BAC is {}", ratio);
    if ratio >= limit {
        println!("It is not legal for you to drive.");
    } else {
        println!("You can drive.");
    }
}