use std::io::{self, Write};

fn main() {
    let display_strings: &[&str] = 
    &[
    "Enter your weight: ", 
    "Enter your gender: ", 
    "Enter the number of drinks: ",
    "Enter the amount of alcohol by volume of the drinks consumed: ",
    "Enter the amount of time since your last drink: "
    ];

    let inputed_values = collect_inputs(
                              display_strings);

    let ratio: f64 = blood_alc_calculator(
                                        &inputed_values[0],
                                        &inputed_values[1],
                                        &inputed_values[2],
                                        &inputed_values[3],
                                        &inputed_values[4]
                                    );

    print_final_string(ratio);
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

fn collect_inputs(arr_str: &[&str]) -> [f64; 5] {
    let mut user_inputs = [0.0, 0.0, 0.0, 0.0, 0.0];
    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            let input = get_user_input(txt);
            if index != 1 {
                let result = validate_nums(&input);
                match result {
                    Ok(number) => {
                        user_inputs[index] = number;
                        break;
                    }
                    Err(e) => println!("{}", e),
                };
            } else {
                let result = validate_gender(&input);
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

fn print_final_string(ratio: f64) {
    println!("Your BAC is {}", ratio);
    if ratio >= 0.08 {
        println!("It is not legal for you to drive.");
    } else {
        println!("You can drive.");
    }
}