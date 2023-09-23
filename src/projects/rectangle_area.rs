use std::io;
use std::process::exit;

// Project "The area of the rectangle"
pub fn rectangle_area() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    println!("The area of the rectangle.");
    println!("Please input the width of the rectangle.");
    let mut width = String::new();

    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read line");

    let width: u32 = match width.trim().parse() {
        Ok(num) => num,
        Err(_) => exit(1),
    };

    println!("Please input the height of the rectangle.");
    let mut height = String::new();

    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read line");

    let height: u32 = match height.trim().parse() {
        Ok(num) => num,
        Err(_) => exit(1),
    };

    let rect = Rectangle { width, height };
    let result = rect.area();
    println!("The area of the triangle is: {}", result);
}

#[cfg(test)]
mod rectangle_area_test {
    #[test]
    fn rectangle_area_test() {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        let rect = Rectangle {
            width: 10,
            height: 20,
        };

        let result = rect.area();
        assert_eq!(result, 200);
    }
}
