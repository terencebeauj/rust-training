pub struct Calculator {}

impl Calculator {
    pub fn add(&self, x: f32, y: f32) -> f32 {
        x + y
    }

    pub fn substract(&self, x: f32, y: f32) -> f32 {
        x - y
    }

    pub fn multiply(&self, x: f32, y: f32) -> f32 {
        x * y
    }

    pub fn divide(&self, x: f32, y: f32) -> f32 {
        if y != 0.0 {
            return x / y;
        }

        println!("Division by 0 forbidden !");
        0.0
    }
}