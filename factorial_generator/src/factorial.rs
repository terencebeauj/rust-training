pub fn factorial(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 1;
    }

    n*factorial(n-1)
}