use std::io;

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

fn factorial(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 1;
    }

    n*factorial(n-1)
}

// TODO: Fix overflow bug
