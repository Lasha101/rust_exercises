use std::io::{self, Write};


fn main() {
    let display_strings: &[&str] = 
    &["Enter a noun: ", 
    "Enter a verb: ",
    "Enter an adjective: ", 
    "Enter an adverb: ",
    "Enter a second adjective: ", 
    "Enter a place: ", 
    "Enter a feeling (e.g., happy, sad, excited): "];
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
    let noun = &arr_str[0];
    let verb = &arr_str[1];
    let adjective1 = &arr_str[2];
    let adverb = &arr_str[3];
    let adjective2 = &arr_str[4];
    let place = &arr_str[5];
    let feeling = &arr_str[6].to_lowercase();

    if feeling == "happy" || feeling == "excited" {
        println!("The {} {} {} {} {} at the {}! You seem {}, what a joyful day!", adjective1, noun, verb, adverb, adjective2, place, feeling);
    } else if feeling == "sad" {
        println!("A {} {} {} {} {} at the {}... Feeling {} made it a melancholic sight.", adjective1, noun, verb, adverb, adjective2, place, feeling);
    } else {
        println!("Do you {} your {} {} {} {} at the {}? Feeling {} makes that quite a story!", verb, adjective1, noun, adverb, adjective2, place, feeling);
    }
}