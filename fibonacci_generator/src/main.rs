use std::io;

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

fn fibonacci(number: usize) -> usize {
    if number == 1 || number == 0 {
        return number;
    }

    fibonacci(number - 1) + fibonacci(number - 2)
}

// TODO: Use a better algorithm to handle big numbers 