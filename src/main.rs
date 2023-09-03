use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Project selection
    println!("Please input the task ID.");
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read line");

    let id: u32 = match id.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a valid task ID.");
            return;
        }
    };

    match id {
        1 => hello(),
        2 => guessing_game(),
        3 => fahrenheit_to_celsius(),
        _ => println!("There is no such project."),
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

    // Project "Fahrenheit to celsius"
    fn fahrenheit_to_celsius() {
        println!("Convert Fahrenheit to Celsius.");
        println!("Please input the temperature in Fahrenheit.");
        let mut fahrenheit = String::new();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid temperature.");
                return;
            }
        };

        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        println!("{fahrenheit} degrees Fahrenheit is equal to {celsius} degrees Celsius.");
    }
}
