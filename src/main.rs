use std::io;

fn main() {
    // Project "Hello, world!"
    println!("Hello, world!");

    // Project "Guessing game"
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
