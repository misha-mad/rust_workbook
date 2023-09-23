use std::io;

// Project "First word"
pub fn first_word() {
    println!("Return a first word.");
    println!("Please input some string.");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    fn word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let result = word(&n);
    println!("The first word is: {}", result);
}

#[test]
fn first_word_test() {
    fn word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let n = String::from("Hello, world!");
    let result = word(&n);
    assert_eq!(result, "Hello,");
}
