use std::io;
use bcrypt::hash;

fn main() {
    // Getting user input from get_user_input function
    let passwd = get_user_input("password");

    if passwd.is_empty() {
        eprintln!("Error: Password cannot be empty.");
        main();
    }

    // Getting user input as int (to declare hash type)
    let user_cost: u32 = match get_user_input("hash cost").trim().parse() {
        Ok(cost) => cost,
        Err(err) => {
            eprintln!("Error parsing hash cost: {}", err);
            return;
        }
    };

    // Using match to hash the password with user_cost hashing (which is 12 by default)
    match hash(passwd, user_cost) {
        Ok(passwd) => {
            println!("Hashed password: {}", passwd);
        } Err (err) => {
            eprintln!("Error hashing password: {}", err);
        }
    };
}

fn get_user_input(prompt: &str) -> String {
    let mut user_input = String::new();

    println!("Please enter your {}: ", prompt);

    io::stdin()
    .read_line(&mut user_input)
    .expect("Failed to readline");

    user_input.trim().to_string()
}