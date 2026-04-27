use std::io::{self, Write};

fn main() {
    let quote = get_user_input("What is the quote? ");
    let author = get_user_input("Who said it? ");
    let final_sentence = author + " says, \"" + &quote + "\"";
    println!("{}",final_sentence);
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string()
}