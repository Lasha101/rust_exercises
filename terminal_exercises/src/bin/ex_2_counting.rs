use std::io::{self, Write};


fn main() {
    print!("What is the input string? ");
    io::stdout()
    .flush()
    .expect("Error! Impossible to handle the buffered data!");
    let mut user_input = String::new();
    io::stdin()
    .read_line(&mut user_input)
    .expect("Impossible to read the input!");
    let character_count = user_input.trim().chars().count();
    println!("{} has {} characters.", user_input.trim(), character_count);
}