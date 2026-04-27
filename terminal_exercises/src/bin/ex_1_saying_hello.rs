use std::io::{self, Write};

fn main() {
    let name = user_input("Enter your name: ");
    match name.as_str() {
        "Anya"  => println!("Hello, {name}, nice to meet you."),
        "Sonya" => println!("Hi, {name}, glad to see you." ),
        "Tanya" => println!("Welcome, {name}."),
        _=> println!("Name not found."),
    };
}

fn user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Fail to write.");
    io::stdin().lines().next().unwrap().expect("Fail to read").trim().to_string()
}
