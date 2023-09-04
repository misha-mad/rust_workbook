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
        4 => fibonacci(),
        _ => println!("There is no such project."),
    }

    // Project "Hello, world!"
    fn hello() {
        println!("Please input your name.");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let name = if input.trim().len() > 0 {
            input.trim().to_owned()
        } else {
            "world".to_owned()
        };

        fn print_hello(name: &String) {
            let mut string = String::from("Hello, ");
            string.push_str(name);
            string.push('!');
            println!("{}", string);
        }

        print_hello(&name);

        // After passing the name reference to the function, the name variable is not moved and remains available
        println!("Have a good day, {}.", name)
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

    // Project "Fibonacci numbers"
    fn fibonacci() {
        println!("Generate Fibonacci numbers.");
        println!("Please input the value of 'n'.");

        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: usize = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number.");
                return;
            }
        };

        let mut fibonacci_numbers: Vec<u32> = vec![0, 1];

        for i in 2..n {
            let next = &fibonacci_numbers[&i - 1] + &fibonacci_numbers[&i - 2];
            fibonacci_numbers.push(next);
        }

        let fibonacci_str: Vec<String> = fibonacci_numbers
            .iter()
            .map(|num| num.to_string())
            .collect();

        let result = fibonacci_str.join(", ");
        println!("Fibonacci sequence: {:?}", result);
    }
}
