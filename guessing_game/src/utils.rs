use rand::Rng;
use std::{cmp::Ordering, io};

use crate::config::{errors, messages, range};

pub fn show_introduction() {
    println!("{}", messages::WELCOME);
    println!(
        "I'm thinking of a number between {} and {}.",
        range::MIN_NUMBER,
        range::MAX_NUMBER
    );
    println!("{}", messages::INPUT_PROMPT);
}

pub fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(range::MIN_NUMBER..=range::MAX_NUMBER)
}

pub fn get_user_input() -> u32 {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect(errors::READ_INPUT);

    guess.trim().parse().expect(errors::PARSE_NUMBER)
}

pub fn show_result(guess: &u32, secret_number: &u32) {
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{}", messages::TOO_SMALL),
        Ordering::Greater => println!("{}", messages::TOO_BIG),
        Ordering::Equal => println!("{}", messages::WIN),
    }
}
