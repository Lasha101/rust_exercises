use std::io::{self, Write};
use std::collections::HashMap;

fn main() {
    let raw_state_data = [
        ("al", "alabama", 0.04),
        ("ak", "alaska", 0.00),
        ("az", "arizona", 0.056),
        ("ar", "arkansas", 0.065),
        ("ca", "california", 0.0725),
        ("co", "colorado", 0.029),
        ("ct", "connecticut", 0.0635),
        ("de", "delaware", 0.00),
        ("fl", "florida", 0.06),
        ("ga", "georgia", 0.04),
        ("hi", "hawaii", 0.04),
        ("id", "idaho", 0.06),
        ("il", "illinois", 0.0625),
        ("in", "indiana", 0.07),
        ("ia", "iowa", 0.06),
        ("ks", "kansas", 0.065),
        ("ky", "kentucky", 0.06),
        ("la", "louisiana", 0.0445),
        ("me", "maine", 0.055),
        ("md", "maryland", 0.06),
        ("ma", "massachusetts", 0.0625),
        ("mi", "michigan", 0.06),
        ("mn", "minnesota", 0.06875),
        ("ms", "mississippi", 0.07),
        ("mo", "missouri", 0.04225),
        ("mt", "montana", 0.00),
        ("ne", "nebraska", 0.055),
        ("nv", "nevada", 0.0685),
        ("nh", "new hampshire", 0.00),
        ("nj", "new jersey", 0.06625),
        ("nm", "new mexico", 0.05125),
        ("ny", "new york", 0.04),
        ("nc", "north carolina", 0.0475),
        ("nd", "north dakota", 0.05),
        ("oh", "ohio", 0.0575),
        ("ok", "oklahoma", 0.045),
        ("or", "oregon", 0.00),
        ("pa", "pennsylvania", 0.06),
        ("ri", "rhode island", 0.07),
        ("sc", "south carolina", 0.06),
        ("sd", "south dakota", 0.045),
        ("tn", "tennessee", 0.07),
        ("tx", "texas", 0.0625),
        ("ut", "utah", 0.0485),
        ("vt", "vermont", 0.06),
        ("va", "virginia", 0.053),
        ("wa", "washington", 0.065),
        ("wv", "west virginia", 0.06),
        ("wi", "wisconsin", 0.05),
        ("wy", "wyoming", 0.04),
    ];

    let raw_county_data = [
        ("wi", "eau-claire", 0.005),
        ("wi", "dunn", 0.004),
        ("il", "cook", 0.003),
    ];

    let mut county_overrides: HashMap<String, HashMap<String, f64>> = HashMap::new();
    for &(state, county, rate) in raw_county_data.iter() {
        county_overrides
            .entry(state.to_string())
            .or_insert_with(HashMap::new)
            .insert(county.to_string(), rate);
    }

    let mut state_taxes: HashMap<(String, String, bool), f64> = HashMap::new();
    for &(abbr, full, rate) in raw_state_data.iter() {
        let has_override = county_overrides.contains_key(abbr);
        let flag = if has_override { true } else { false };
        
        state_taxes.insert((abbr.to_string(), full.to_string(), flag), rate);
    }

    let display_strings: &[&str] = 
    &["What is the order amount? ", "What is the state? "];
    let inputed_values = get_validate_amount_state(display_strings);

    let tax_rates = get_rates(&inputed_values.0, &state_taxes, &county_overrides);

    // Dynamic condition block replacing hardcoded state paths
    if inputed_values.0.is_empty() {
        println!("The total is ${:.2}", inputed_values.1);
    } else if let Some([state_rate, county_rate]) = tax_rates {
        let total_rate = state_rate + county_rate;
        let final_state_struct = State { tax_rate: total_rate };
        let results = calculator(&final_state_struct, inputed_values.1);
        
        print_tax_results(results[0], results[1]);
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
                    user_inputs.0 = parsed.to_lowercase();
                    break;
                } else {
                    println!("{}", "Entered value must contain only letters!")
                }
            }
        }
    }
    user_inputs
}

fn get_rates(
    state_term: &str,
    state_taxes: &HashMap<(String, String, bool), f64>,
    county_overrides: &HashMap<String, HashMap<String, f64>>,
) -> Option<[f64; 2]> {

    let ((abbr, _, has_override), &state_rate) = state_taxes
        .iter()
        .find(|((abbr, full, _), _)| abbr == &state_term || full == &state_term)?;

    if !has_override {
        return Some([state_rate, 0.0]);
    }

    let county_input = get_user_input("What is the county? ");
    let county_term = county_input.trim().to_lowercase();

    let &county_rate = county_overrides
        .get(abbr)?
        .get(&county_term)
        .unwrap_or(&0.0);

    Some([state_rate, county_rate])
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

// Separate specialized print function
fn print_tax_results(tax: f64, total: f64) {
    println!("The tax is ${:.2}.\nThe total is ${:.2}.", tax, total);
}