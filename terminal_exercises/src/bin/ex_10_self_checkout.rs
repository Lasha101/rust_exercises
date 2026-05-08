use std::io::{self, Write};

fn main() {
    let items = collect_inputs();

    let subtotal = calc_dot_product(&items);
    let tax_rate = 0.055;
    let tax = calc_tax(subtotal, tax_rate);
    let total = calc_total(subtotal, tax);

    print_final_string(subtotal, tax, total);
}

fn get_user_input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut text = String::new();
    io::stdin().read_line(&mut text).unwrap();

    text.trim().to_string()
}

fn collect_inputs() -> Vec<[i32; 2]> {
    let mut user_inputs = Vec::new();
    let mut item_number = 1;

    loop {
        // PRICE
        let price = loop {
            let input =
                get_user_input(&format!("Enter price of item {} (or 'done'): ", item_number));

            if input.to_lowercase() == "done" {
                return user_inputs;
            }

            match input.parse::<i32>() {
                Ok(n) => break n,
                Err(_) => println!("Please enter a numeric value."),
            }
        };

        // QUANTITY
        let quantity = loop {
            let input = get_user_input(&format!("Enter quantity of item {}: ", item_number));

            match input.parse::<i32>() {
                Ok(n) => break n,
                Err(_) => println!("Please enter a numeric value."),
            }
        };

        user_inputs.push([price, quantity]);
        item_number += 1;
    }
}

fn calc_dot_product(items: &[[i32; 2]]) -> i32 {
    items.iter().map(|item| item[0] * item[1]).sum()
}

fn calc_tax(subtotal: i32, tax_rate: f64) -> f64 {
    let tax = subtotal as f64 * tax_rate;
    (tax * 100.0).round() / 100.0
}

fn calc_total(subtotal: i32, tax: f64) -> f64 {
    subtotal as f64 + tax
}

fn print_final_string(subtotal: i32, tax: f64, total: f64) {
    println!("\nSubtotal: ${:.2}", subtotal as f64);
    println!("Tax: ${:.2}", tax);
    println!("Total: ${:.2}", total);
}

