use std::collections::HashMap;
use std::io::{self, Write};

struct Rule {
    required_facts: &'static [(&'static str, char)],
    success_message: &'static str,
}

fn main() {
    let rules = vec![
        Rule {
            required_facts: &[("silent", 'y'), ("corroded", 'y')],
            success_message: "Clean terminals and try starting again.",
        },
        Rule {
            required_facts: &[("silent", 'y'), ("corroded", 'n')],
            success_message: "Replace cables and try again.",
        },
        Rule {
            required_facts: &[("silent", 'n'), ("clicking", 'y')],
            success_message: "Replace the battery.",
        },
        Rule {
            required_facts: &[("silent", 'n'), ("clicking", 'n'), ("crank_fail", 'y')],
            success_message: "Check spark plug connections.",
        },
        Rule {
            required_facts: &[("silent", 'n'), ("clicking", 'n'), ("crank_fail", 'n'), ("die", 'y'), ("injection", 'y')],
            success_message: "Get it in for service.",
        },
        Rule {
            required_facts: &[("silent", 'n'), ("clicking", 'n'), ("crank_fail", 'n'), ("die", 'y'), ("injection", 'n')],
            success_message: "Check to ensure the choke is opening and closing.",
        },
    ];

    let questions = HashMap::from([
        ("silent", "Is the car silent when you turn the key? "),
        ("corroded", "Are the battery terminals corroded? "),
        ("clicking", "Does the car make a clicking noise? "),
        ("crank_fail", "Does the car crank up but fail to start? "),
        ("die", "Does the engine start and then die? "),
        ("injection", "Does your car have fuel injection? "),
    ]);

    let mut runtime_facts: HashMap<String, char> = HashMap::new();

    loop {
        let mut matching_rule_found = false;

        for rule in &rules {
            let mut rule_is_possible = true;
            let mut next_fact_to_ask: Option<&str> = None;
            for &(fact_key, expected_val) in rule.required_facts {
                if let Some(&actual_val) = runtime_facts.get(fact_key) {
                    if actual_val != expected_val {
                        rule_is_possible = false; // Disqualified
                        break;
                    }
                } else {
                    next_fact_to_ask = Some(fact_key);
                    break;
                }
            }

            if !rule_is_possible {
                continue;
            }

            if next_fact_to_ask.is_none() {
                println!("{}", rule.success_message);
                matching_rule_found = true;
                break;
            }

            if let Some(fact_key) = next_fact_to_ask {
                if let Some(prompt) = questions.get(fact_key) {
                    let answer = get_user_input(prompt);
                    runtime_facts.insert(fact_key.to_string(), answer);
                    matching_rule_found = true; 
                    break; // Restart loop evaluation with new fact
                }
            }
        }

        if !matching_rule_found || runtime_facts.len() == questions.len() {
            break;
        }
    }
}

fn get_user_input(prompt_text: &str) -> char {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().chars().next().unwrap_or(' ')
}


