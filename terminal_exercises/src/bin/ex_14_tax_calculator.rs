use std::io::{self, Write};

fn main() {
    let display_strings: &[&str] = 
        &["What is the order amount? ", "What is the state? "];
    let inputed_values = collect_inputs(display_strings);

    let wisconsin_tax_rate = State {
        tax_rate: 0.055,
    };
    print_final_string(inputed_values.0, &wisconsin_tax_rate, inputed_values.1);
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validate_alpha_only(text: &str) -> bool {
    let is_valid = text.chars().all(|c| c.is_alphabetic());
    is_valid
}

fn collect_inputs(arr_str: &[&str]) -> (String, f64) {
    let mut user_inputs = (String::new(), 0.0);
    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            let input = get_user_input(*txt);
            if index == 0 {
                match input.trim().parse::<f64>() {
                    Ok(parsed_number) => {
                        user_inputs.1 = parsed_number;
                        break;
                    },
                    Err(error_message) => println!("{}", error_message),
                }
            } else {
                let parsed = input.trim();
                if validate_alpha_only(parsed) {
                    user_inputs.0 = parsed.to_string().to_ascii_lowercase();
                    break;
                } else {
                    println!("{}", "Entered value must contain only letters!")
                }
            }
        }
    }
    user_inputs
}

trait Calculation {
    fn calculate(&self, a: f64) -> f64;
}

struct State {
    tax_rate: f64,
}

impl Calculation for State {
    fn calculate(&self, a: f64) -> f64 {
        a * self.tax_rate
    }
}

fn print_final_string(state_name: String, calc: &dyn Calculation, amount: f64) {
    // Added a trailing period to match standard sentence punctuation
    let mut final_string = format!("The total is ${:.2}.", amount);
    
    if state_name == "wi" || state_name == "wisconsin" { 
        let result = calc.calculate(amount);
        let rounded_result = (result * 100.0).ceil() / 100.0;
        
        // MODIFICATION: Base the total on the safely rounded tax amount
        let total = amount + rounded_result; 
        
        final_string = format!(
            "The subtotal is ${:.2}.\nThe tax is ${:.2}.\nThe total is ${:.2}.",
            amount, rounded_result, total
        );
    }
    println!("{}", final_string);
}