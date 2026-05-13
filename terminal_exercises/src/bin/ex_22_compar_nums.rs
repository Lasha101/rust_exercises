use std::io::{self, Write};

fn main() {
    let display_strings: &[&str] = 
    &[
    "Enter the first number: ", 
    "Enter the second number: ", 
    "Enter the third number:: ",
    ];

    let inputed_values = collect_inputs(
                              display_strings);
    
    let largest_num = comparing_function(inputed_values);

    println!("The largest number is {}.", largest_num);
}

fn get_user_input(prompt_text: &str) -> String {
    print!("{}", prompt_text);
    io::stdout().flush().expect("Error to show the text!");
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Fail to read text!");
    text.trim().to_string() 
}

fn validate_nums(input:&str) -> Result<i32, String> {
    match input.trim().parse::<i32>() {
        Ok(number) => Ok(number),
        Err(_) => Err(String::from("Cannot calculate value.")),
    }
}

fn collect_inputs(arr_str: &[&str]) -> [i32; 3] {
    let mut user_inputs = [0, 0, 0];
    for (index, txt) in arr_str.iter().enumerate() {
        loop {
            let input = get_user_input(txt);
            let result = validate_nums(&input);
            match result {
                Ok(number) => {
                    let populated_slice = &user_inputs[..index];
                    if !populated_slice.contains(&number) {
                        user_inputs[index] = number;
                        break;
                    } else {
                        println!("Enter different number!");
                    }  
                }
                Err(e) => println!("{}", e),
            };   
        }  
    }
    user_inputs
}

fn comparing_function(arr: [i32; 3]) -> i32 {
    if arr[0] > arr[1] && arr[0] > arr[2] {
        arr[0]
    } else if arr[1] > arr[2] {
        arr[1]
    } else {
        arr[2]
    }
}
