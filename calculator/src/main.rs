struct Calculator {}

impl Calculator {
    fn add(&self, x: f32, y: f32) -> f32 {
        x + y
    }

    fn substract(&self, x: f32, y: f32) -> f32 {
        x - y
    }

    fn multiply(&self, x: f32, y: f32) -> f32 {
        x * y
    }

    fn divide(&self, x: f32, y: f32) -> f32 {
        if y != 0.0 {
            return x / y;
        }

        println!("Division by 0 forbidden !");
        0.0
    }
}

fn main() {
    let calculator = Calculator {};
    let five = calculator.add(3.0, 2.0);
    let ten = calculator.substract(10.0, 0.0);
    let one = calculator.multiply(1.0, 1.0);
    let two = calculator.divide(4.0, 2.0);
    let zero = calculator.divide(3.0, 0.0);
}
