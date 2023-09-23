use std::io;

// Project "Fahrenheit to celsius"
pub fn fahrenheit_to_celsius() {
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

#[test]
fn fahrenheit_to_celsius_test() {
    let fahrenheit = 32.0;
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    assert_eq!(celsius, 0.0);
}
