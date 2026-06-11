use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let lang_prompt = "Select language (en / fr): ";
    let lang = get_user_input(lang_prompt);

    let display_string: &str = "Please enter the number of the month: ";

    let answer = validate_input(display_string, &lang);
    
    println!("{}", answer);
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validate_input(txt: &str, lang: &str) -> String {
    let mut months = HashMap::new();

    months.insert(("en", 1), "The name of the month is January.");
    months.insert(("en", 2), "The name of the month is February.");
    months.insert(("en", 3), "The name of the month is March.");
    months.insert(("en", 4), "The name of the month is April.");
    months.insert(("en", 5), "The name of the month is May.");
    months.insert(("en", 6), "The name of the month is June.");
    months.insert(("en", 7), "The name of the month is July.");
    months.insert(("en", 8), "The name of the month is August.");
    months.insert(("en", 9), "The name of the month is September.");
    months.insert(("en", 10), "The name of the month is October.");
    months.insert(("en", 11), "The name of the month is November.");
    months.insert(("en", 12), "The name of the month is December.");

    months.insert(("fr", 1), "Le nom du mois est Janvier.");
    months.insert(("fr", 2), "Le nom du mois est Février.");
    months.insert(("fr", 3), "Le nom du mois est Mars.");
    months.insert(("fr", 4), "Le nom du mois est Avril.");
    months.insert(("fr", 5), "Le nom du mois est Mai.");
    months.insert(("fr", 6), "Le nom du mois est Juin.");
    months.insert(("fr", 7), "Le nom du mois est Juillet.");
    months.insert(("fr", 8), "Le nom du mois est Août.");
    months.insert(("fr", 9), "Le nom du mois est Septembre.");
    months.insert(("fr", 10), "Le nom du mois est Octobre.");
    months.insert(("fr", 11), "Le nom du mois est Novembre.");
    months.insert(("fr", 12), "Le nom du mois est Décembre.");

    let language_key = if lang == "fr" { "fr" } else { "en" };

    loop {
        let input: Result<i32, _> = get_user_input(txt).parse();
        
        if let Ok(num) = input {
            if let Some(&month_name) = months.get(&(language_key, num)) {
                return month_name.to_string();
            }
        }
        
        println!("Enter number 1 - 12.");
    }
}

