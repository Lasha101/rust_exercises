use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["Enter the principal: ", 
    "Enter the rate of interest: ", 
    "Enter the number of years: ",
    "What is the number of times the interest is compounded per year? "];

    let inputed_values = collect_inputs(
                              display_strings);

    let final_amount: f64 = calc_compound_interest(
                                        &inputed_values[0],
                                        &inputed_values[1],
                                        &inputed_values[2],
                                        &inputed_values[3]
                                    );

    print_final_string(inputed_values[0], 
                       inputed_values[1],
                       inputed_values[2],
                       inputed_values[3],
                       final_amount);
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn collect_inputs(arr_str: &[&str]) -> [f64; 4] {
    let mut user_inputs = [0.0, 0.0, 0.0, 0.0];
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

fn calc_compound_interest(principal: &f64,
                        interest: &f64, 
                        years: &f64, 
                        per_year: &f64) -> f64 {
    principal * (1.0 + (interest/100.0)/per_year).powf(per_year*years)
}




fn print_final_string(principal: f64, rate: f64, years: f64, per_year: f64, final_amount: f64) {
    let years_as_int = years as i32;
    let rounded_final_amount = (final_amount * 100.0).round()/100.0;
    println!("${} invested at {}% for {} years\n\
              compounded {} times per year is ${}.",
              principal, rate, years_as_int, per_year, rounded_final_amount);
}