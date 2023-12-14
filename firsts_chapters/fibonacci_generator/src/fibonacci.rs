pub fn fibonacci(number: usize) -> usize {
    if number == 1 || number == 0 {
        return number;
    }

    fibonacci(number - 1) + fibonacci(number - 2)
}
