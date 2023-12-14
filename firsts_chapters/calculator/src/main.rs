mod calculator;

use calculator::Calculator;

fn main() {
    let calculator = Calculator {};
    let five = calculator.add(3.0, 2.0);
    let ten = calculator.substract(10.0, 0.0);
    let one = calculator.multiply(1.0, 1.0);
    let two = calculator.divide(4.0, 2.0);
    let zero = calculator.divide(3.0, 0.0);
}
