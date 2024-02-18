use std::io;
use bcrypt::{hash, DEFAULT_COST};

fn main() {
    println!("Please enter your password, that you wanna encrypt");
    println!("...");

    let user_input = get_user_input();

    let passwd = user_input.trim();

    match hash(passwd, DEFAULT_COST) {
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