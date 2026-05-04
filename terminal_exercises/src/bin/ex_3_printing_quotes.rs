use std::io::{self, Write};
use std::collections::HashMap;


fn main() {
    let author = get_user_input("Who is the author? ");

    let quotes = [
    HashMap::from([("Yogi Berra", "Never answer an anonymous letter.")]),
    HashMap::from([("Albert Einstein", "Imagination is more important than knowledge.")]),
    HashMap::from([("Oscar Wilde", "Be yourself; everyone else is already taken.")]),
    ];

    let result = quotes.iter().find_map(|quote| quote.get(author.as_str()));
    if let Some(quote) =  result {
        println!("{}", quote);
    } else {
        println!("Author not found!");
    }
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string()
}