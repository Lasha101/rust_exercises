use std::io::{self, Write};


fn main() {
    let display_string: &str = &"What is the radius of the room in feet? ";

    let radius = get_user_input(display_string);

    let area = area(&radius);

    const INDEX: i32 = 350;

    let num_of_gallons = num_of_gallons_to_paint(&area, &INDEX);

    print_final_string(num_of_gallons, area);
}

fn get_user_input(prompt_text: &str) -> f64 {
    loop {
        print!("{}", prompt_text);
        io::stdout().flush().expect("Error to show the text!");
        let mut text = String::new();
        io::stdin().read_line(&mut text).expect("Fail to read text!");
        match  text.trim().parse::<f64>() {
            Ok(parsed_number) => break parsed_number,
            Err(error_message) => {
                println!("{}", error_message);
                continue;
            }
        }
    }   
}

fn area(radius: &f64) -> f64 {
    std::f64::consts::PI * radius.powf(2.0)  
}

fn num_of_gallons_to_paint(area: &f64, index: &i32) -> i32 {
    let result = *area / (*index as f64);
    result.ceil() as i32
}


fn print_final_string(num_of_gallons: i32, area: f64) {
    println!("You will need to purchase {} gallons of\n\
              paint to cover {:.2} square feet.",
              num_of_gallons, area);
}