use std::io;
use bcrypt::hash;

fn main() {
    println!("Please enter your password, that you wanna encrypt");
    println!("...");

    // Getting user input from get_user_input function
    let passwd: String = get_user_input();

    println!("Please enter your wanted hash type: ");
    println!("...");

    // Getting user input as int (to declare hash type)
    let user_cost: u32 = get_user_cost();

    // Using match to hash the password with user_cost hashing (which is 12 by default)
    match hash(passwd, user_cost) {
        Ok(passwd) => {
            println!("Hashed password: {}", passwd);
        } Err (err) => {
            eprintln!("Error hashing password: {}", err);
        }
    };
}

fn get_user_input() -> String {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .unwrap();

    user_input
} 

fn get_user_cost() -> u32 {
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .unwrap();

    user_input.trim().parse().unwrap()
}