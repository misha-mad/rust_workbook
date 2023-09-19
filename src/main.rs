use projects::{
    fahrenheit_to_celsius, fibonacci, first_word, generate_random_string, guessing_game, hello,
    rectangle_area,
};

use std::io;
pub mod projects;

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
        1 => hello::hello(),
        2 => guessing_game::guessing_game(),
        3 => fahrenheit_to_celsius::fahrenheit_to_celsius(),
        4 => fibonacci::fibonacci(),
        5 => first_word::first_word(),
        6 => rectangle_area::rectangle_area(),
        7 => generate_random_string::generate_random_string(),
        _ => println!("There is no such project."),
    }
}
