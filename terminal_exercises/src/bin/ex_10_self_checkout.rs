use std::io::{self, Write};

fn main() {
    let display_strings: &[&str] = &["price", "quantity"];

    let inputed_values = collect_inputs(display_strings);

    let subtotal = calc_dot_product(&inputed_values);
    

    let tax_rate: f64 = 0.055;

    let tax = calc_tax(&subtotal, &tax_rate);

    let total = calc_total(&subtotal, &tax);

    print_final_string(subtotal, tax, total);
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

// fn collect_inputs(arr_str: &[&str]) -> Vec<HashMap<String, HashMap<String, i32>>> {
//     let mut user_inputs = Vec::new();
    
//     for num_of_item in 0..3 {
//         let mut item: HashMap<String, HashMap<String, i32>> = HashMap::new();
//         let mut item_data= HashMap::new();
//         for txt in arr_str {
            
//             let formated_text = format!("Enter the {} of item {}: ",
//                                                     *txt, num_of_item);
//             loop {
//                 let input = get_user_input(&formated_text);
//                 match  input.trim().parse::<i32>() {
//                     Ok(parsed_number) => {
//                         item_data.insert(txt.to_string(), parsed_number);
//                         break;
//                     },
//                     Err(error_message) => println!("{}", error_message),
//                 }   
                
//             }  
//         }   
//         item.insert(format!("Item {}", num_of_item), item_data);
//         user_inputs.push(item); 
//     }
//     user_inputs
// }

fn collect_inputs(arr_str: &[&str]) -> Vec<[i32; 2]> {
    let mut user_inputs = Vec::new();
    for num_of_item in 0..3 {
        let mut price_quantity  = [0, 0];

        for (index, txt) in arr_str.iter().enumerate() {

            let formated_text = format!("Enter the {} of item {}: ",
                                                    *txt, num_of_item + 1);
            loop {
                let input = get_user_input(&formated_text);
                match  input.trim().parse::<i32>() {
                    Ok(parsed_number) => {
                        price_quantity[index] = parsed_number;
                        break;
                    },
                    Err(error_message) => println!("{}", error_message),
                }   
                
            } 
            
        }  
        user_inputs.push(price_quantity); 
    }
    user_inputs
}



fn calc_dot_product(pair_numbers: &[[i32; 2]]) -> i32 {
    let subtotal = pair_numbers.iter().map(|item| item[0] * item[1]).sum();
    subtotal
}

fn calc_tax(subtotal: &i32, index: &f64) -> f64 {
    let result = (*subtotal as f64) * index;
    (result * 100.0).round() / 100.0
}

fn calc_total(subtotal: &i32, tax: &f64) -> f64 {
    (*subtotal as f64) + *tax
}


fn print_final_string(subtotal: i32, tax: f64, total: f64) {
    let subtotal_as_float = subtotal as f64;
    println!("Subtotal: ${:.2}\nTax: ${}\nTotal: ${}", subtotal_as_float, tax, total);
}

