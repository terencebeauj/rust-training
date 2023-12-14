pub fn check_choice(choice: i8) {
    if choice != 1 && choice != 2 {
        println!("You need to choose between 1 and 2. Going to exit the software!");
        std::process::abort();
    }
}