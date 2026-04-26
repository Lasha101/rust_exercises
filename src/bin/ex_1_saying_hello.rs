use std::io::{self, Write};

fn main() {
    let name = user_input("Enter your name: ");
    let greeting = match name.as_str() {
        "Anya" | "Sonya" | "Tanya" => greeting_constructor(&name),
        _=> "Name not found.".to_string(),
    };
    println!("{}", greeting);
}

fn user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Fail to write.");
    io::stdin().lines().next().unwrap().expect("Fail to read").trim().to_string()
}

fn greeting_constructor(name: &str) -> String {
    format!("Hello, {}, nice to meet you.", name)  
}