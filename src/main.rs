use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Project selection
    println!("Please input the task ID.");
    let mut id = String::new();

    io::stdin()
        .read_line(&mut id)
        .expect("Failed to read line");

    if id.contains('1') {
        hello();
    } else if id.contains('2') {
        guessing_game();
    }

    // Project "Hello, world!"
    fn hello() {
        println!("Hello, world!");
    }

    // Project "Guessing game"
    fn guessing_game() {
        println!("Guess the number!");
        let secret_number = rand::thread_rng().gen_range(1..=100);

        loop {
            println!("Please input your guess.");
            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}
