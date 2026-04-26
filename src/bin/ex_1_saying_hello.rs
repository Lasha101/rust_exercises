use std::io::{self, Write};

fn main() {
    print!("What is your name? ");
    io::stdout().flush().expect("Fail to write.");
    println!(
        "Hello, {}, nice to meet you!", 
        io::stdin().lines().next().unwrap().expect("Fail to read").trim()
    );
}



