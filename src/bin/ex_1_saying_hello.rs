use std::io::{self, Write};

fn main() {
    print!("What is your name? ");
    io::stdout().flush().expect("Fail to write.");
    for line in io::stdin().lines() {
        println!("{}", format!("Hello, {}, nice to meet you!", line.expect("Fail to read.").trim()));
        break;
    }   
}

// fn main() {
//     print!("What is your name? ");
//     io::stdout().flush().expect("Error");
//     let mut name = String::new();
//     io::stdin().read_line(&mut name).expect("Fail to read");
//     let greeting = format!("Hello, {}, nice to meet you!", name.trim());
//     println!("{}", greeting);
// }



