pub mod range {
    pub const MIN_NUMBER: u32 = 1;
    pub const MAX_NUMBER: u32 = 100;
}

pub mod messages {
    pub const WELCOME: &str = "Guess the number!";
    pub const INPUT_PROMPT: &str = "Please input your guess number:";
    pub const TOO_SMALL: &str = "Too small!";
    pub const TOO_BIG: &str = "Too big!";
    pub const WIN: &str = "You win!";
}

pub mod errors {
    pub const READ_INPUT: &str = "Failed to read line";
    pub const PARSE_NUMBER: &str = "Please type a number!";
}
