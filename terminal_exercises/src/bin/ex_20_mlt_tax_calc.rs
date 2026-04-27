use std::io::{self, Write};

fn main() {
    let display_strings: &[&str] = 
    &["What is the order amount? ", "What is the state? "];
    let inputed_values = get_validate_amount_state(display_strings);

    if inputed_values.0 == "WI" {
        let wisconsin_tax_rate: State = State {
            tax_rate: 0.055,
        };
        let tax_wisconsin = calculator(&wisconsin_tax_rate, inputed_values.1);
        let county_name= get_user_input(
            &"Eau Claire or Dunn? ");
            if county_name.eq_ignore_ascii_case("eau claire") { 
                let eau_claire_tax_rate: State = State {
                    tax_rate: 0.005,
                };
                let tax_county = calculator(&eau_claire_tax_rate, inputed_values.1);
                let total_tax_eau_claire = tax_county[0] + tax_wisconsin[0];
                let total_wi_county = tax_wisconsin[1] + tax_county[0];
                println!("The tax is ${:.2}.\nThe total is ${:.2}.", total_tax_eau_claire, total_wi_county);
            } else if county_name.eq_ignore_ascii_case( "dunn") { 
                let dunn_tax_rate: State = State {
                    tax_rate: 0.004,
                };
                let tax_county = calculator(&dunn_tax_rate, inputed_values.1);
                let total_tax_dunn = tax_county[0] + tax_wisconsin[0];
                let total_wi_county = tax_wisconsin[1] + tax_county[0];
                println!("The tax is ${:.2}.\nThe total is ${:.2}.", total_tax_dunn, total_wi_county);
            } else {
                println!("The tax is ${:.2}.\nThe total is ${:.2}.", tax_wisconsin[0], tax_wisconsin[1])
            }
    } else if inputed_values.0 == "IL" {
        let illinois_tax_rate: State = State {
            tax_rate: 0.08,
        };
        let tax_illinois = calculator(&illinois_tax_rate, inputed_values.1);
        println!("The tax is ${:.2}.\nThe total is ${:.2}.", tax_illinois[0], tax_illinois[1])
    } else if inputed_values.0 == "" {
        println!("The total is ${:.2}", inputed_values.1)
    } else {
        println!("State name not recognized!");
    }

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

fn get_validate_amount_state(arr_str: &[&str]) -> (String, f64) {
    let mut user_inputs = (String::new(), 0.0);
    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            let input = get_user_input(*txt).to_uppercase();
            if index == 0 {
                match  input.trim().parse::<f64>() {
                Ok(parsed_number) => {
                    user_inputs.1 = parsed_number;
                    break;
                },
                Err(error_message) => println!("{}", error_message),
                }
            } else {
                let parsed = input.trim();
                if validate_alpha_only(parsed) {
                    user_inputs.0 = parsed.to_string();
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

fn calculator(calc: &dyn Calculation, amount: f64) -> [f64; 2]{
    let tax_amount = calc.calculate(amount);
    let rounded_tax_amount = (tax_amount * 100.0).round()/100.0;
    let total = tax_amount + amount;
    let rounded_total = (total * 100.0).round()/100.0; 
    [rounded_tax_amount, rounded_total]
}

