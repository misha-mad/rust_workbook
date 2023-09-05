use std::io;

// Project "Hello, world!"
pub fn hello() {
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

    // After passing the name reference to the function,
    // the name variable is not moved and remains available
    println!("Have a good day, {}.", name)
}
