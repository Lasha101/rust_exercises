use std::io::{self, Write};

fn main() {
    let display_strings: &[&str] = 
    &[
    "Choice unit! \nFor Metric enter \"1.0\" for Imperial enter \"2.0\":",
    "Enter your weight: ", 
    "Enter your height: ",
    ];

    let inputed_values = collect_inputs(
                              display_strings);

    let bmi_metric: f64 = bmi_calculator_metric(
                                    &inputed_values[1],
                                    &inputed_values[2],
                                    );
    let bmi_imperial: f64 = bmi_calculator(&bmi_metric);
    
    let ratio:f64;
    if inputed_values[0] == 1.0 {
        ratio = bmi_metric;
    } else {
        ratio = bmi_imperial;
    }

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

fn bmi_calculator_metric(weight: &f64, height: &f64) -> f64 {
    weight / (height * height) 
}

fn bmi_calculator(bmi_metric: &f64) -> f64 {
    bmi_metric * 703.0
}

fn print_final_string(ratio: f64) {
    println!("Your BMI is {}", ratio);
    if ratio <= 18.5 {
        println!("You are underweight. You should see your doctor.");
    } else if ratio >= 25.0 {
        println!("You are overweight. You should see your doctor.");
    } else {
        println!("You are within the ideal weight range.")
    }
}