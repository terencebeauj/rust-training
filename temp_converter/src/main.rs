mod converters;
mod utils;

use std::io;

use crate::converters::{celsius_to_farenheit, farenheit_to_celsius};
use crate::utils::check_choice;

fn main() {
    println!("Do you want to convert from:\n 1 - Celsius to Farenheit\n 2 - Farenheit to Celsius");
    let mut choice: String = String::new();
    let mut temp: String = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read choice input");

    let choice: i8 = choice
        .trim()
        .parse()
        .expect("You need to provide an integer.");

    check_choice(choice);

    println!("Enter your temperature:");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read temperature input.");

    let temp: f64 = temp
        .trim()
        .parse()
        .expect("You need to provide a floating number.");

    let result: f64;

    if choice == 1 {
        result = celsius_to_farenheit(temp);
        println!("{temp} C = {result} F");
    } else {
        result = farenheit_to_celsius(temp);
        println!("{temp} F = {result} C");
    };
}

// TO DO
// Add code to handle min temperature (ex -273.15 C)
// Add code to Kelvin as well
