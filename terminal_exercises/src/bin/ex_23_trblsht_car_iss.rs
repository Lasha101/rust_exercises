use std::io::{self, Write};

fn main() {
    let display_string: &str = "Is the car silent when you turn the key? ";
    let answer = get_user_input(display_string);
    if answer == 'y' {
        let answer = get_user_input(&"Are the battery terminals corroded? ");
        if answer == 'y' {
            println!("Clean terminals and try starting again.");
        } else if answer == 'n' {
            println!("Replace cables and try again.");
        }    
    } else if answer == 'n' {
        let answer = get_user_input(&"Does the car make a clicking noise? ");
        if answer == 'y' {
            println!("Replace the battery.");
        } else if answer == 'n' {
            let answer = get_user_input("Does the car crank up but fail to start? ");
            if answer == 'y' {
                println!("Check spark plug connections.");
            } else if answer == 'n' {
                let answer = get_user_input("Does the engine start and then die? ");
                if answer == 'y' {
                    let answer = get_user_input("Does your car have fuel injection? ");
                    if answer == 'y' {
                        println!("Get it in for service.");
                    } else if answer == 'n' {
                        println!("Check to ensure the choke is opening and closing.");
                    }
                }
            }
        }
    }

}

fn get_user_input(prompt_text: &str) -> char {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().chars().next().expect("Option did not contain value.")
}


