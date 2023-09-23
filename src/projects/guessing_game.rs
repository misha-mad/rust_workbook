use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// Project "Guessing game"
pub fn guessing_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = Guess::new(guess.trim().parse().expect("Error during input parsing"));
        println!("You guessed: {:?}", guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod guessing_game_tests {
    use super::*;

    #[test]
    fn less_then_100() {
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let guess = Guess::new(secret_number);
        assert_eq!(guess.value(), secret_number);
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
