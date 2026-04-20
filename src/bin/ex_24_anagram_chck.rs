use std::io::{self, Write};

fn main() {
    let display_strings: &[&str] = 
    &["Enter two strings and I'll tell you if they are anagrams!\n\
    Enter the first string: ", 
    "Enter the second string: "];

    let inputed_values = get_validate_strings(display_strings);

    let is_anagram = is_anagram(&inputed_values);
    
    if is_anagram {
        println!("\"{}\" and \"{}\" are anagrams.", inputed_values[0], inputed_values[1])
    } else {
        println!("They are not anagrams.")
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

fn get_validate_strings(arr_str: &[&str]) -> [String; 2] {
    let mut user_inputs = [String::new(), String::new()];
    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            let input = get_user_input(*txt).to_lowercase();
            let parsed = input.trim();
            if validate_alpha_only(parsed) {
                user_inputs[index] = parsed.to_string();
                break;
            } else {
                println!("{}", "Entered value must contain only letters!")
            }
            
        }
    }
    user_inputs
}



fn is_anagram(data: &[String; 2]) -> bool {
    if data[0].len() != data[1].len() {
        return false; 
    }
    let mut counts: [i32; 26] = [0; 26];
    for b in data[0].bytes() {counts[(b - b'a') as usize] += 1;}
    for b in data[1].bytes() {counts[(b - b'a') as usize] -= 1;}
    counts.iter().all(|&count| count == 0)
}