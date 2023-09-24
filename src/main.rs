use projects::{
    fahrenheit_to_celsius, fibonacci, first_word, generate_random_string, guessing_game, hello,
    rectangle_area,
};

use crate::projects::grep;
use std::env;

pub mod projects;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Please use numbers from 1 to 7 as arguments.\nOr use 2 arguments: a  search string as the first argument and a file path as the second argument.");
        std::process::exit(0);
    }

    if args.len() == 2 {
        let project_index = args[1].as_str();

        match project_index {
            "1" => hello::hello(),
            "2" => guessing_game::guessing_game(),
            "3" => fahrenheit_to_celsius::fahrenheit_to_celsius(),
            "4" => fibonacci::fibonacci(),
            "5" => first_word::first_word(),
            "6" => rectangle_area::rectangle_area(),
            "7" => generate_random_string::generate_random_string(),
            _ => {
                println!("There is no such project. Please use numbers from 1 to 7 as arguments.");
                std::process::exit(0);
            }
        };
    }

    if args.len() == 3 {
        grep::grep();
    }

    if args.len() > 3 {
        println!("Too many arguments. Please use numbers from 1 to 7 as arguments.\nOr use 2 arguments: a  search string as the first argument and a file path as the second argument.");
        std::process::exit(0);
    }
}
