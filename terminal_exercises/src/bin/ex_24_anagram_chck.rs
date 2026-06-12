use std::io::{self, Write};

fn main() {
    let display_strings: &[&str] = 
    &["Enter two strings and I'll tell you if they are anagrams!\n\
       Enter the first string: ", "Enter the second string: "];

    let mut inputed_values = get_validate_strings(display_strings);

    let original_word_1 = inputed_values[0].clone();
    let original_word_2 = inputed_values[1].clone();

    let (first_part, second_part) = inputed_values.split_at_mut(1);
    let is_anagram = is_anagram(&mut first_part[0], &mut second_part[0]);
    
    if is_anagram {
        println!("\"{}\" and \"{}\" are anagrams.", original_word_1, original_word_2)
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

fn is_anagram(word_1: &mut String, word_2: &mut String) -> bool {
    let mut total_1 = 0;
    let mut total_2 = 0;
    for _ in word_1.chars() {
        total_1 = total_1 + 1;
    }
    for _ in word_2.chars() {
        total_2 = total_2 + 1;
    }
    if total_1 == total_2 {
        let mut chars_1: Vec<char> = word_1.chars().collect();
        let mut chars_2: Vec<char> = word_2.chars().collect();

        for idx_1 in (0..chars_1.len()).rev() {
            let ch_1 = chars_1[idx_1];
            let mut removed = false;

            for idx_2 in (0..chars_2.len()).rev() {
                let ch_2 = chars_2[idx_2];
                
                if ch_1 == ch_2 {
                    chars_1.remove(idx_1);
                    chars_2.remove(idx_2);
                    removed = true;
                    break;
                }
            }
            if removed {
                continue;
            }
        }    
        word_1.clear();
        word_2.clear();
        for ch in chars_1 { word_1.push(ch); }
        for ch in chars_2 { word_2.push(ch); }
    }
    word_1 == word_2
}