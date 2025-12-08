mod config;
mod utils;

use utils::{generate_secret_number, get_user_input, show_introduction, show_result};

fn main() {
    show_introduction();
    let secret_number = generate_secret_number();
    loop {
        let user_guess = get_user_input();
        show_result(&user_guess, &secret_number);
        if user_guess == secret_number {
            break;
        };
    }
}
