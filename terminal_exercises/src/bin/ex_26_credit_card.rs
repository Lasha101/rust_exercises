use std::io::{self, Write};

fn main() {
    let display_strings: &[&str] = &[
        "What is your balance? ", 
        "What is the APR on the card (as a percent)? ",
        "Choice: for calculate months(0) or monthly payment(1): "
    ];

    let inputed_values = collect_inputs(display_strings);
    
    let result: f64;

    if inputed_values[2] == 0.0 {
        let monthly_payment = loop {
            let additional_input = get_user_input("What is the monthly payment you can make? ");
            match validate_nums(&additional_input) {
                Ok(num) => break num,
                Err(e) => println!("{}", e),
            }
        };
        let calc_values = [inputed_values[0], inputed_values[1], monthly_payment];
        result = monts_calculator(&calc_values).ceil();
        println!("It will take you {} months to pay off this card.", result);
    } else {
        let months = loop {
            let additional_input = get_user_input("What is the number of months to payoff? ");
            match validate_nums(&additional_input) {
                Ok(num) => break num,
                Err(e) => println!("{}", e),
            }
        };
        
        let calc_values = [inputed_values[0], inputed_values[1], months];
        result = monthly_payment_calculator(&calc_values);
        println!("Amount of monthly payment is {:.2}", result);
    }  
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validate_nums(input: &str) -> Result<f64, String> {
    match input.trim().parse::<f64>() {
        Ok(number) => Ok(number),
        Err(_) => Err(String::from("Cannot calculate value. Please enter a valid number.")),
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

fn monts_calculator(values: &[f64; 3]) -> f64 {
    let mltp = -1.0 / 30.0;
    let i = (values[1] / 100.0) / 365.0;
    let numerator = (1.0 + (values[0] / values[2]) * (1.0 - (1.0 + i).powf(30.0))).ln();
    let denominator = (1.0 + i).ln();
    mltp * numerator / denominator
}

fn monthly_payment_calculator(values: &[f64; 3]) -> f64 {
    let i = (values[1] / 100.0) / 365.0;

    let time_factor = (1.0 + i).powf(30.0 * values[2]);
    let numerator = values[0] * i * time_factor;
    let denominator = time_factor - 1.0;
    
    (numerator / denominator) * 30.0 
}