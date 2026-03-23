use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["Enter a noun: ", "Enter a verb: ", 
    "Enter an adjective: ", "Enter an adverb: "];
    let inputed_values = collect_inputs(
                              display_strings);
    print_final_string(&inputed_values);
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn collect_inputs(arr_str: &[&str]) -> Vec<String> {
    let mut user_inputs = Vec::new();
    for txt in arr_str {
        let input = get_user_input(*txt);
        user_inputs.push(input);
    }
    user_inputs
}

fn print_final_string(arr_str: &[String]){
    println!("Do you {} your {} {} {}? That's hilarious!",
             arr_str[1], arr_str[2], arr_str[0], arr_str[3]);
}