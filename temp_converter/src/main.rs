use std::io;

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

fn celsius_to_farenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}

fn farenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}

fn check_choice(choice: i8) {
    if choice != 1 && choice != 2 {
        println!("You need to choose between 1 and 2. Going to exit the software!");
        std::process::abort();
    }
}

// TO DO
// Add code to handle min temperature (ex -273.15 C)
// Add code to Kelvin as well
