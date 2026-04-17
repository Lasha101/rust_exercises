use std::io::{self, Write};

fn main() {
    let display_strings: &str = 
        "Press C to convert from Fahrenheit to Celsius.\n\
         Press F to convert from Celsius to Fahrenheit.\n";

    let inputed_values = collect_inputs(display_strings);

    let unit = inputed_values.0;
    let temp = inputed_values.1;
    
    if unit == "Celsius" {
        let to_c = FahrenheitToCelsius;
        print_final_string(&to_c, temp, &unit);
    } else if unit == "Fahrenheit" {
        let to_f = CelsiusToFahrenheit;
        print_final_string(&to_f, temp, &unit);
    }
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validator(input: &str) -> Result<i32, String> {
    match input.trim().parse::<i32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(String::from("Cannot calculate value. Please try again.")),
    }
}


fn get_temperature(source_unit: &str) -> i32 {
    let prompt_str = format!("Please enter the temperature in {}: ", source_unit);
    
    loop {
        let inputed_value = get_user_input(&prompt_str);
        match validator(&inputed_value) {
            Ok(number) => return number, 
            Err(e) => println!("{}", e), 
        }
    }
}

fn collect_inputs(txt: &str) -> (String, i32) {
    loop {
        let input = get_user_input(txt).to_uppercase();
        println!("Your choice: {}", input);
        
        if input.trim() == "C" {
            let temp = get_temperature("Fahrenheit");
            return ("Celsius".to_string(), temp);
            
        } else if input.trim() == "F" {
            let temp = get_temperature("Celsius");
            return ("Fahrenheit".to_string(), temp);
            
        } else {
            println!("Not recognized! Please enter C or F.");
        }
    }
}

trait Calculation {
    fn calculate(&self, a: i32) -> i32;
}

struct CelsiusToFahrenheit; 
struct FahrenheitToCelsius;

impl Calculation for CelsiusToFahrenheit {
    fn calculate(&self, a: i32) -> i32 {
        (a * 9 / 5) + 32
    }
}

impl Calculation for FahrenheitToCelsius {
    fn calculate(&self, a: i32) -> i32 {
        (a - 32) * 5 / 9
    }
}

fn print_final_string(calc: &dyn Calculation, temp: i32, unit: &str) {
    let result = calc.calculate(temp);
    println!("The temperature in {} is {}", unit, result);
}


