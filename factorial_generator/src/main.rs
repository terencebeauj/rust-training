mod factorial;

use std::io;

use factorial::factorial;

fn main() {
    let mut number = String::new();
    println!("Enter your positive number:");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read your number");

    let number: usize = number.trim().parse().expect("You should enter an integer");

    let result = factorial(number);

    println!("{number}! = {result}");
}

// TODO: Fix overflow bug
