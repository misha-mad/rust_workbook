use std::io;

// Project "Fibonacci numbers"
pub fn fibonacci() {
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

#[test]
fn fibonacci_test() {
    let mut fibonacci_numbers: Vec<u32> = vec![0, 1];

    for i in 2..10 {
        let next = &fibonacci_numbers[&i - 1] + &fibonacci_numbers[&i - 2];
        fibonacci_numbers.push(next);
    }

    let fibonacci_str: Vec<String> = fibonacci_numbers
        .iter()
        .map(|num| num.to_string())
        .collect();

    let result = fibonacci_str.join(", ");
    assert_eq!(result, "0, 1, 1, 2, 3, 5, 8, 13, 21, 34");
}
