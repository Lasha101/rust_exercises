use std::io::{self, Write};

fn main() {
    let display_strings: &[&str] = 
    &[
    "What is your balance? ", 
    "What is the APR on the card (as a percent)? ",
    "What is the monthly payment you can make? "
    ];

    let inputed_values = collect_inputs(
                              display_strings);

    let num_of_months: f64 = calculator(&inputed_values).ceil();

    println!("It will take you {} months to pay off this card.", num_of_months)
    
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

fn collect_inputs(arr_str: &[&str]) -> [f64; 3] {
    let mut user_inputs = [0.0, 0.0, 0.0];
    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            let input = get_user_input(txt);
            
            let result = validate_nums(&input);
            match result {
                Ok(number) => {
                    user_inputs[index] = number;
                    break;
                }
                Err(e) => println!("{}", e),
            };   
        }  
    }
    user_inputs
}

fn calculator(values: &[f64; 3]) -> f64 {
    let mltp = -1.0 / 30.0;
    let i = (values[1] / 100.0) / 365.0;
    let numerator = (1.0 + (values[0] / values[2]) * (1.0 - (1.0 + i).powf(30.0))).ln();
    let denominator = (1.0 + i).ln();
    mltp * numerator / denominator
}
