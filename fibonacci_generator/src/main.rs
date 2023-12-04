mod fibonacci;

use std::io;

use crate::fibonacci::fibonacci;

fn main() {
    let mut number = String::new();

    println!("Enter the nth Fibonacci number that you want to retrieve: ");
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read your input number");

    let number: usize = number.trim().parse().expect("You should enter an integer!");

    let result = fibonacci(number);

    println!("{number}th fibonacci number is: {result}")
}

// TODO: Use a better algorithm to handle big numbers
